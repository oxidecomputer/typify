// Copyright 2023 Oxide Computer Company

use std::{
    collections::{BTreeMap, BTreeSet},
    iter::repeat,
};

use log::debug;
use schemars::schema::{
    ArrayValidation, InstanceType, NumberValidation, ObjectValidation, Schema, SchemaObject,
    SingleOrVec, StringValidation, SubschemaValidation,
};

use crate::{util::ref_key, validate::schema_value_validate, RefKey};

/// Merge all schemas in array of schemas. If the result is unsatisfiable, this
/// returns `Schema::Bool(false)`.
pub(crate) fn merge_all(schemas: &[Schema], defs: &BTreeMap<RefKey, Schema>) -> Schema {
    let mut ss = schemas.iter();
    let (Some(a), Some(b)) = (ss.next(), ss.next()) else {
        panic!("merge_all requires at least two schemas")
    };

    try_merge_schema(a, b, defs)
        .and_then(|start| {
            ss.try_fold(start, |schema, other| {
                try_merge_schema(&schema, other, defs)
            })
        })
        .unwrap_or_else(|_| Schema::Bool(false))
}

/// Given two additionalItems schemas that might be None--which is equivalent
/// to Schema::Bool(true)--this returns the appropriate value. This is only
/// called in a situation where additionalItems are relevant, so we prefer
/// `true` to the (equivalent) absence of the schema. In other words, this will
/// never return None.
fn merge_additional_items(
    a: Option<&Schema>,
    b: Option<&Schema>,
    defs: &BTreeMap<RefKey, Schema>,
) -> Option<Schema> {
    match (a, b) {
        (None, None) => Some(Schema::Bool(true)),
        (None, other) | (other, None) => other.cloned(),
        (Some(aa), Some(bb)) => {
            Some(try_merge_schema(aa, bb, defs).unwrap_or_else(|_| Schema::Bool(false)))
        }
    }
}

/// Merge two schemas returning the resulting schema. If the two schemas are
/// incompatible (i.e. if there is no data that can satisfy them both
/// simultaneously) then this returns Err.
fn try_merge_schema(a: &Schema, b: &Schema, defs: &BTreeMap<RefKey, Schema>) -> Result<Schema, ()> {
    match (a, b) {
        (Schema::Bool(false), _) | (_, Schema::Bool(false)) => Ok(Schema::Bool(false)),
        (Schema::Bool(true), other) | (other, Schema::Bool(true)) => Ok(other.clone()),

        // Resolve references here before we start to merge the objects.
        //
        // TODO: need to mitigate circular references so we don't go into a
        // spin loop. We can do this by wrapping defs in a structure that
        // remembers what we've already looked up; if we hit a cycle we can
        // consider the proper handling, but it might be to ignore it--a
        // circular allOf chain is a bit hard to reason about.
        //
        // TODO if we merge a referenced schema with another schema **and**
        // the resulting schema is identical to the referenced schema (i.e.
        // the other schema is strictly more permissive) then we should just
        // return the reference schema.
        (
            Schema::Object(SchemaObject {
                reference: Some(ref_name),
                ..
            }),
            other,
        )
        | (
            other,
            Schema::Object(SchemaObject {
                reference: Some(ref_name),
                ..
            }),
        ) => {
            let key = ref_key(ref_name);
            let resolved = defs.get(&key).unwrap();
            try_merge_schema(resolved, other, defs)
        }

        (Schema::Object(aa), Schema::Object(bb)) => Ok(merge_schema_object(aa, bb, defs)?.into()),
    }
}

fn merge_schema_object(
    a: &SchemaObject,
    b: &SchemaObject,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<SchemaObject, ()> {
    debug!(
        "merging {}\n{}",
        serde_json::to_string_pretty(a).unwrap(),
        serde_json::to_string_pretty(b).unwrap(),
    );

    assert!(a.reference.is_none());
    assert!(b.reference.is_none());

    let instance_type = merge_so_instance_type(a.instance_type.as_ref(), b.instance_type.as_ref())?;
    let format = merge_so_format(a.format.as_ref(), b.format.as_ref())?;

    let number = merge_so_number(a.number.as_deref(), b.number.as_deref())?;
    let string = merge_so_string(a.string.as_deref(), b.string.as_deref())?;
    let array = merge_so_array(a.array.as_deref(), b.array.as_deref(), defs)?;
    let object = merge_so_object(a.object.as_deref(), b.object.as_deref(), defs)?;

    // We could clean up this schema to eliminate data irrelevant to the
    // instance type, but logic in the conversion path should already handle
    // that.
    let merged_schema = SchemaObject {
        metadata: None,
        instance_type,
        format,
        enum_values: None,
        const_value: None,
        subschemas: None,
        number,
        string,
        array,
        object,
        reference: None,
        extensions: Default::default(),
    };

    // If we have subschemas for either schema then we merge the body of the
    // two schemas and then do the appropriate merge with subschemas (i.e.
    // potentially twice). This is effectively an `allOf` between the merged
    // "body" schema and the component subschemas.
    let merged_schema = try_merge_with_subschemas(merged_schema, a.subschemas.as_deref(), defs)?;
    let merged_schema = try_merge_with_subschemas(merged_schema, b.subschemas.as_deref(), defs)?;

    assert_ne!(merged_schema, Schema::Bool(false).into_object());

    let enum_values = merge_so_enum_values(
        a.enum_values.as_ref(),
        a.const_value.as_ref(),
        b.enum_values.as_ref(),
        b.const_value.as_ref(),
    )?;

    debug!(
        "merged {}",
        serde_json::to_string_pretty(&merged_schema).unwrap(),
    );

    match enum_values {
        None => Ok(merged_schema),
        Some(enum_values) => {
            let enum_values = enum_values
                .into_iter()
                .filter(|value| {
                    schema_value_validate(&Schema::Object(merged_schema.clone()), value, defs)
                        .is_ok()
                })
                .collect::<Vec<_>>();
            if enum_values.is_empty() {
                Err(())
            } else {
                Ok(SchemaObject {
                    enum_values: Some(enum_values),
                    ..merged_schema
                })
            }
        }
    }
}

fn merge_so_enum_values(
    a_enum: Option<&Vec<serde_json::Value>>,
    a_const: Option<&serde_json::Value>,
    b_enum: Option<&Vec<serde_json::Value>>,
    b_const: Option<&serde_json::Value>,
) -> Result<Option<Vec<serde_json::Value>>, ()> {
    let aa = match (a_enum, a_const) {
        (None, None) => None,
        (Some(enum_values), None) => Some(enum_values.clone()),
        (None, Some(value)) => Some(vec![value.clone()]),
        (Some(_), Some(_)) => unimplemented!(),
    };
    let bb = match (b_enum, b_const) {
        (None, None) => None,
        (Some(enum_values), None) => Some(enum_values.clone()),
        (None, Some(value)) => Some(vec![value.clone()]),
        (Some(_), Some(_)) => unimplemented!(),
    };

    match (aa, bb) {
        (None, None) => Ok(None),
        (None, Some(values)) | (Some(values), None) => Ok(Some(values)),
        (Some(aa), Some(bb)) => {
            let values = aa
                .into_iter()
                .filter(|value| bb.contains(value))
                .collect::<Vec<_>>();

            if values.is_empty() {
                Err(())
            } else {
                Ok(Some(values))
            }
        }
    }
}

pub(crate) fn merge_with_subschemas(
    schema_object: SchemaObject,
    maybe_subschemas: Option<&SubschemaValidation>,
    defs: &BTreeMap<RefKey, Schema>,
) -> SchemaObject {
    try_merge_with_subschemas(schema_object, maybe_subschemas, defs).unwrap()
}

/// Merge the schema with a subschema validation object. It's important that
/// the return value reduces the complexity of the problem so avoid infinite
/// recursion.
fn try_merge_with_subschemas(
    schema_object: SchemaObject,
    maybe_subschemas: Option<&SubschemaValidation>,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<SchemaObject, ()> {
    // TODO conceivably these different subschema types could appear at the
    // same time; maybe it's fine to just handle them distinctly?
    match maybe_subschemas {
        Some(SubschemaValidation {
            all_of: Some(all_of),
            any_of: None,
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        }) => {
            let merged_schema = all_of
                .iter()
                .try_fold(schema_object.into(), |schema, other| {
                    try_merge_schema(&schema, other, defs)
                })?;
            assert_ne!(merged_schema, Schema::Bool(false));
            Ok(merged_schema.into_object())
        }

        Some(SubschemaValidation {
            all_of: None,
            any_of: Some(subschemas),
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        })
        | Some(SubschemaValidation {
            all_of: None,
            any_of: None,
            one_of: Some(subschemas),
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        }) => {
            // First we do a pairwise merge the schemas; if the result is
            // invalid / unresolvable / never / whatever, we exclude it
            // from the list. If it is valid, *then* we do the join to preserve
            // information (though we probably only need to *that* if at least
            // one schema contains a ref). This could probably be an
            // opportunity for memoization, but this is an infrequent
            // construction so... whatever.
            let joined_schemas = Some(
                subschemas
                    .iter()
                    .filter_map(|other| {
                        // Skip if the merged schema is unsatisfiable.
                        let _ =
                            try_merge_schema(&schema_object.clone().into(), other, defs).ok()?;
                        Some(join_schema(&schema_object, other))
                    })
                    .collect(),
            );

            // TODO we'd want to do something special here? maybe return an
            // error?
            assert!(!subschemas.is_empty());

            let subschemas = match maybe_subschemas {
                Some(SubschemaValidation {
                    any_of: Some(_), ..
                }) => SubschemaValidation {
                    any_of: joined_schemas,
                    ..Default::default()
                },
                Some(SubschemaValidation {
                    one_of: Some(_), ..
                }) => SubschemaValidation {
                    one_of: joined_schemas,
                    ..Default::default()
                },
                _ => unreachable!(),
            };
            Ok(SchemaObject {
                metadata: schema_object.metadata,
                subschemas: Some(Box::new(subschemas)),
                ..Default::default()
            })
        }

        Some(SubschemaValidation {
            all_of: None,
            any_of: None,
            one_of: None,
            not: Some(not),
            if_schema: None,
            then_schema: None,
            else_schema: None,
        }) => try_merge_schema_not(schema_object, not.as_ref(), defs),

        // If it's any of the subschemas we don't know how to handle here,
        // we'll try to leave them in place... probably to encounter an error
        // later when we try to convert them...
        Some(SubschemaValidation {
            all_of: None,
            any_of: None,
            one_of: None,
            not: None,
            if_schema,
            then_schema,
            else_schema,
        }) if if_schema.is_some() || then_schema.is_some() || else_schema.is_some() => {
            unimplemented!("if/then/else schemas are not supported")
        }

        Some(unknown) => {
            todo!("{}", serde_json::to_string_pretty(unknown).unwrap());
        }
        None => Ok(schema_object),
    }
}

/// "Subtract" the "not" schema from the schema object.
///
/// TODO Exactly where and how we handle not constructions is... tricky! As we
/// find and support more and more useful uses of not we will likely move some
/// of this into the conversion methods.
fn try_merge_schema_not(
    mut schema_object: SchemaObject,
    not_schema: &Schema,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<SchemaObject, ()> {
    match not_schema {
        // Subtracting everything leaves nothing...
        Schema::Bool(true) => Err(()),
        // ... whereas subtracting nothing leaves everything.
        Schema::Bool(false) => Ok(schema_object),

        Schema::Object(SchemaObject {
            metadata: None,
            instance_type: None,
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: None,
            string: None,
            array: None,
            object: Some(not_object),
            reference: None,
            extensions: _,
        }) => {
            // TODO this is incomplete, but seems sufficient for the schemas
            // we've seen in the wild.
            if let Some(ObjectValidation {
                required,
                properties,
                ..
            }) = schema_object.object.as_deref_mut()
            {
                not_object.required.iter().for_each(|not_required| {
                    let _ = required.remove(not_required);
                    let _ = properties.remove(not_required);
                });
            }

            Ok(schema_object)
        }

        Schema::Object(SchemaObject {
            metadata: None,
            instance_type: None,
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: Some(not_subschemas),
            number: None,
            string: None,
            array: None,
            object: None,
            reference: None,
            extensions: _,
        }) => try_merge_with_subschemas_not(schema_object, not_subschemas, defs),

        // If we can't usefully reduce the complexity, leave it for the
        // coversion pass.
        _ => {
            schema_object.subschemas().not = Some(Box::new(not_schema.clone()));
            Ok(schema_object)
        }
    }
}

fn try_merge_with_subschemas_not(
    schema_object: SchemaObject,
    not_subschemas: &SubschemaValidation,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<SchemaObject, ()> {
    match not_subschemas {
        SubschemaValidation {
            all_of: None,
            any_of: Some(any_of),
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => {
            // A not of anyOf is equivalent to an allOf of not... and the
            // latter is easier to merge with other schemas by subtraction.
            let all_of = any_of
                .iter()
                .map(|ss| {
                    Schema::Object(SchemaObject {
                        subschemas: Some(Box::new(SubschemaValidation {
                            not: Some(Box::new(ss.clone())),
                            ..Default::default()
                        })),
                        ..Default::default()
                    })
                })
                .collect::<Vec<_>>();
            let new_other = SchemaObject {
                subschemas: Some(Box::new(SubschemaValidation {
                    all_of: Some(all_of),
                    ..Default::default()
                })),
                ..Default::default()
            };
            merge_schema_object(&schema_object, &new_other, defs)
        }

        SubschemaValidation {
            all_of: None,
            any_of: None,
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => Ok(schema_object),

        _ => todo!(),
    }
}

fn join_schema(a: &SchemaObject, b: &Schema) -> Schema {
    SchemaObject {
        subschemas: Some(Box::new(SubschemaValidation {
            all_of: Some(vec![Schema::Object(a.clone()), b.clone()]),
            ..Default::default()
        })),
        ..Default::default()
    }
    .into()
}

/// Merge instance types which could be None (meaning type is valid), a
/// singleton type, or an array of types. An error result indicates that the
/// types were non-overlappin and therefore incompatible.
fn merge_so_instance_type(
    a: Option<&SingleOrVec<InstanceType>>,
    b: Option<&SingleOrVec<InstanceType>>,
) -> Result<Option<SingleOrVec<InstanceType>>, ()> {
    match (a, b) {
        (None, None) => Ok(None),
        (None, other @ Some(_)) | (other @ Some(_), None) => Ok(other.map(Clone::clone)),

        // If each has a single type, it must match.
        (Some(SingleOrVec::Single(aa)), Some(SingleOrVec::Single(bb))) => {
            if aa == bb {
                Ok(Some(SingleOrVec::Single(aa.clone())))
            } else {
                Err(())
            }
        }

        // If one has a single type and the other is an array, the type must
        // appear in the array (and that's the resulting type).
        (Some(SingleOrVec::Vec(types)), Some(SingleOrVec::Single(it)))
        | (Some(SingleOrVec::Single(it)), Some(SingleOrVec::Vec(types))) => {
            if types.contains(it) {
                Ok(Some(SingleOrVec::Single(it.clone())))
            } else {
                Err(())
            }
        }

        // If both are arrays, we take the intersection; if the intersection is
        // empty, we return an error.
        (Some(SingleOrVec::Vec(aa)), Some(SingleOrVec::Vec(bb))) => {
            let types = aa
                .iter()
                .collect::<BTreeSet<_>>()
                .intersection(&bb.iter().collect::<BTreeSet<_>>())
                .cloned()
                .cloned()
                .collect::<Vec<_>>();

            match types.len() {
                // No intersection
                0 => Err(()),
                1 => Ok(Some(types.into_iter().next().unwrap().into())),
                _ => Ok(Some(types.into())),
            }
        }
    }
}

/// By and large, formats are pretty free-form and aren't really compatible
/// with each other. That is to say, if you have two formats at the same time
/// that's probably unsatisfiable. There are a few notable exceptions to this:
///
/// o integer widths -- take the narrowest
/// o "ip" vs. "ipv4" / "ipv6" -- take the more specific ip flavor
///
/// TODO incorporate the instance type / types here to limit what formats we
/// consider.
/// TODO We might need to handle this in a very type-specific way in order to
/// properly handle cases such as
/// "int8" and "uint8" -> { min: 0, max: 127, format: None }
fn merge_so_format(a: Option<&String>, b: Option<&String>) -> Result<Option<String>, ()> {
    match (a.map(String::as_str), b.map(String::as_str)) {
        (None, other) | (other, None) => Ok(other.map(String::from)),

        (Some("ip"), result @ Some("ipv4"))
        | (Some("ip"), result @ Some("ipv6"))
        | (result @ Some("ipv4"), Some("ip"))
        | (result @ Some("ipv6"), Some("ip")) => Ok(result.map(String::from)),

        // Fine if they're both the same
        (Some(aa), Some(bb)) if aa == bb => Ok(Some(aa.into())),
        // ... they're not the same...
        (Some(_), Some(_)) => Err(()),
    }
}

fn merge_so_number(
    a: Option<&NumberValidation>,
    b: Option<&NumberValidation>,
) -> Result<Option<Box<NumberValidation>>, ()> {
    match (a, b) {
        (None, other) | (other, None) => Ok(other.cloned().map(Box::new)),
        (Some(_), Some(_)) => {
            unimplemented!("this is fairly fussy and I don't want to do it")
        }
    }
}

fn merge_so_string(
    a: Option<&StringValidation>,
    b: Option<&StringValidation>,
) -> Result<Option<Box<StringValidation>>, ()> {
    match (a, b) {
        (None, other) | (other, None) => Ok(other.cloned().map(Box::new)),
        (Some(_), Some(_)) => {
            unimplemented!("this is fairly fussy and I don't want to do it")
        }
    }
}

fn merge_so_array(
    a: Option<&ArrayValidation>,
    b: Option<&ArrayValidation>,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<Option<Box<ArrayValidation>>, ()> {
    match (a, b) {
        (None, other) | (other, None) => Ok(other.cloned().map(Box::new)),
        (Some(aa), Some(bb)) => {
            let max_items = choose_value(aa.max_items, bb.max_items, Ord::min);
            let min_items = choose_value(aa.min_items, bb.min_items, Ord::max);
            let unique_items =
                choose_value(aa.unique_items, bb.unique_items, std::ops::BitOr::bitor);

            // We can only contain one thing; we can't resolve the need to
            // contain two different things.
            let contains = match (aa.contains.as_deref(), bb.contains.as_deref()) {
                (None, other) | (other, None) => other.cloned().map(Box::new),

                // We could probably do a more complex "equivalency" check e.g.
                // that would follow references.
                (Some(aa_contains), Some(bb_contains)) if aa_contains == bb_contains => {
                    Some(Box::new(aa_contains.clone()))
                }

                (Some(_), Some(_)) => return Err(()),
            };

            // If min > max the schema is unsatisfiable.
            if let (Some(min), Some(max)) = (min_items, max_items) {
                if min > max {
                    return Err(());
                }
            }

            // The items and additional_items fields need to be considered
            // together.
            //
            // - If items is a singleton, additional_items is ignored and all
            //   items in the array must obey the items schema.
            //
            // - If items is an array of size N, the Ith < N item must conform
            //   to the Ith schema. Subsequent items must conform to
            //   additional_items (so can be whatever if it is None =
            //   Schema::Bool(true))
            //
            // - If items is None (i.e. absent) additional_items is ignored and
            //   any value is permitted in any position of the array.
            //
            // Note that if there is a maximum array length specified and the
            // items schema array is at least that long, additional_items is
            // irrelevant so we omit it. This case appears several times below.

            let (items, additional_items) = match (
                (&aa.items, &aa.additional_items),
                (&bb.items, &bb.additional_items),
            ) {
                // Both items are none; items and additional_items are None.
                ((None, _), (None, _)) => (None, None),

                // A None and a single-item; we can use the single item and
                // additional_items are irrelevant.
                ((None, _), (Some(SingleOrVec::Single(item)), _))
                | ((Some(SingleOrVec::Single(item)), _), (None, _)) => {
                    (Some(SingleOrVec::Single(item.clone())), None)
                }

                // A None and a array of schemas; we can take the array,
                // modifying it only in consideration of the maximum length (if
                // it is specified).
                ((None, _), (Some(SingleOrVec::Vec(items)), additional_items))
                | ((Some(SingleOrVec::Vec(items)), additional_items), (None, _)) => {
                    match (max_items, items.len()) {
                        (Some(max), len) if len >= max as usize => (
                            Some(SingleOrVec::Vec(
                                items.iter().take(max as usize).cloned().collect(),
                            )),
                            None,
                        ),
                        _ => (
                            Some(SingleOrVec::Vec(items.clone())),
                            additional_items.clone(),
                        ),
                    }
                }

                // Two single schemas, just merge them; additional_items would
                // be irrelevant.
                (
                    (Some(SingleOrVec::Single(aa_single)), _),
                    (Some(SingleOrVec::Single(bb_single)), _),
                ) => (
                    Some(SingleOrVec::Single(Box::new(try_merge_schema(
                        aa_single, bb_single, defs,
                    )?))),
                    None,
                ),

                // A single item and an array of schemas. We merge the
                // singleton with the array and additional_items as needed.
                (
                    (Some(SingleOrVec::Single(single)), _),
                    (Some(SingleOrVec::Vec(items)), additional_items),
                )
                | (
                    (Some(SingleOrVec::Vec(items)), additional_items),
                    (Some(SingleOrVec::Single(single)), _),
                ) => match (max_items, items.len()) {
                    (Some(max), len) if len >= max as usize => (
                        Some(SingleOrVec::Vec(
                            items
                                .iter()
                                .take(max as usize)
                                .map(|item_schema| try_merge_schema(item_schema, single, defs))
                                .collect::<Result<_, _>>()?,
                        )),
                        None,
                    ),
                    _ => {
                        let items = items
                            .iter()
                            .map(|item_schema| try_merge_schema(item_schema, single, defs))
                            .collect::<Result<_, _>>()?;
                        let additional_items = additional_items.as_deref().map_or_else(
                            || Ok(single.as_ref().clone()),
                            |additional_schema| try_merge_schema(additional_schema, single, defs),
                        )?;
                        (
                            Some(SingleOrVec::Vec(items)),
                            Some(Box::new(additional_items)),
                        )
                    }
                },

                // We need to pairwise merge schemas--as many as the longer
                // of the two items arrays, limited by the max size of the
                // array if one is specified. To do this we create
                // iterators over the items followed by a repetition of the
                // additional_items schema. We zip these together, merge, and
                // limit them as appropriate.
                (
                    (Some(SingleOrVec::Vec(aa_items)), aa_additional_items),
                    (Some(SingleOrVec::Vec(bb_items)), bb_additional_items),
                ) => {
                    let items_len = aa_items.len().max(bb_items.len());

                    // Note that one of these .chain(repeat(..)) statements is
                    // always irrelevant because we will always .take(..) a
                    // quantity less than or equal to the longest of the two
                    // schema arrays; we just do them both and don't sweat it.
                    let aa_items_iter = aa_items
                        .iter()
                        .map(Some)
                        .chain(repeat(aa_additional_items.as_deref()));
                    let bb_items_iter = bb_items
                        .iter()
                        .map(Some)
                        .chain(repeat(bb_additional_items.as_deref()));
                    let items_iter =
                        aa_items_iter
                            .zip(bb_items_iter)
                            .map(|schemas| match schemas {
                                (None, None) => unreachable!(),
                                (None, Some(item)) => Ok(item.clone()),
                                (Some(item), None) => Ok(item.clone()),
                                (Some(aa_item), Some(bb_item)) => {
                                    try_merge_schema(aa_item, bb_item, defs)
                                }
                            });

                    match max_items {
                        Some(max) if items_len >= max as usize => {
                            let items = items_iter.take(max as usize).collect::<Result<_, _>>()?;
                            (Some(SingleOrVec::Vec(items)), None)
                        }

                        _ => {
                            let items = items_iter.take(items_len).collect::<Result<_, _>>()?;
                            let additional_items = merge_additional_items(
                                aa_additional_items.as_deref(),
                                bb_additional_items.as_deref(),
                                defs,
                            )
                            .map(Box::new);

                            (Some(SingleOrVec::Vec(items)), additional_items)
                        }
                    }
                }
            };

            Ok(Some(Box::new(ArrayValidation {
                items,
                additional_items,
                max_items,
                min_items,
                unique_items,
                contains,
            })))
        }
    }
}

/// Prefer Some over None and the result of `prefer` if both are Some.
fn choose_value<T, F>(a: Option<T>, b: Option<T>, prefer: F) -> Option<T>
where
    F: FnOnce(T, T) -> T,
{
    match (a, b) {
        (None, other) | (other, None) => other,
        (Some(aa), Some(bb)) => Some(prefer(aa, bb)),
    }
}

fn merge_so_object(
    a: Option<&ObjectValidation>,
    b: Option<&ObjectValidation>,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<Option<Box<ObjectValidation>>, ()> {
    match (a, b) {
        (None, other) | (other, None) => Ok(other.cloned().map(Box::new)),
        (Some(aa), Some(bb)) => {
            let a_props = aa.properties.iter().filter_map(|(name, a_schema)| {
                let resolved_schema = if let Some(b_schema) = bb.properties.get(name) {
                    try_merge_schema(a_schema, b_schema, defs)
                } else {
                    filter_prop(name, a_schema, bb)
                };

                // TODO I'm going to copy/paste this so move it to a
                // subroutine.
                match resolved_schema {
                    // If a required field is incompatible with the
                    // other schema, this object is unsatisfiable.
                    Err(()) if aa.required.contains(name) => Some(Err(())),

                    // We can ignore incompatible, non-required fields.
                    Err(()) => None,

                    // Compatible schema; proceed.
                    Ok(schema) => Some(Ok((name.clone(), schema))),
                }
            });

            let b_props = bb.properties.iter().filter_map(|(name, b_schema)| {
                if aa.properties.contains_key(name) {
                    // We handled the intersection above.
                    None
                } else {
                    // TODO I'm going to copy/paste this so move it to a
                    // subroutine.
                    match filter_prop(name, b_schema, aa) {
                        // If a required field is incompatible with the
                        // other schema, this object is unsatisfiable.
                        Err(()) if bb.required.contains(name) => Some(Err(())),

                        // We can ignore incompatible, non-required fields.
                        Err(()) => None,

                        // Compatible schema; proceed.
                        Ok(schema) => Some(Ok((name.clone(), schema))),
                    }
                }
            });

            let properties = a_props.chain(b_props).collect::<Result<_, ()>>()?;

            let required = aa.required.union(&bb.required).cloned().collect();
            let additional_properties = merge_additional_properties(
                aa.additional_properties.as_deref(),
                bb.additional_properties.as_deref(),
            );

            let object_validation = ObjectValidation {
                required,
                properties,
                additional_properties,
                ..Default::default()
            };
            Ok(Some(object_validation.into()))
        }
    }
}

// TODO this is starting to feel redundant...
fn merge_additional_properties(a: Option<&Schema>, b: Option<&Schema>) -> Option<Box<Schema>> {
    match (a, b) {
        (Some(Schema::Bool(true)), other)
        | (None, other)
        | (other, Some(Schema::Bool(true)))
        | (other, None) => other.cloned().map(Box::new),

        (Some(Schema::Bool(false)), _) | (_, Some(Schema::Bool(false))) => None,

        (Some(aa @ Schema::Object(_)), Some(bb @ Schema::Object(_))) => Some(Box::new(
            SchemaObject {
                subschemas: Some(Box::new(SubschemaValidation {
                    // TODO it would be a good idea to merge these now rather than
                    // deferring that since the schemas might be unresolvable i.e.
                    // they might have no intersection. However, a non-true/false/
                    // absent additionalProperties within an allOf is an uncommon
                    // pattern so this is likely good enough for the moment.
                    all_of: Some(vec![aa.clone(), bb.clone()]),
                    ..Default::default()
                })),
                ..Default::default()
            }
            .into(),
        )),
    }
}

fn filter_prop(
    name: &str,
    prop_schema: &Schema,
    object_schema: &ObjectValidation,
) -> Result<Schema, ()> {
    // We're only considering properties we *know* do not appear in the other
    // object's schema.
    assert!(!object_schema.properties.contains_key(name));

    // TODO We should do a simple check here to validating the name against
    // propertyNames if that schema is specified.
    assert!(object_schema.property_names.is_none());

    // TODO We should first check patternProperties, but that's such a pain in
    // the neck and so weird that I can't be bothered right now (until we hit
    // some examples in the wild). A match here would exempt the property from
    // the check below against additionalProperties.
    assert!(object_schema.pattern_properties.is_empty());

    merge_additional(object_schema.additional_properties.as_deref(), prop_schema)
}

fn merge_additional(additional: Option<&Schema>, prop_schema: &Schema) -> Result<Schema, ()> {
    match additional {
        // Anything is fine.
        Some(Schema::Bool(true)) | None => Ok(prop_schema.clone()),
        // Nothing is fine.
        Some(Schema::Bool(false)) => Err(()),

        // Some things might be fine.
        Some(additional_schema) => Ok(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                // TODO it would be a good idea to merge these now rather than
                // deferring that since the schemas might be unresolvable i.e.
                // they might have no intersection. However, a non-true/false/
                // absent additionalProperties within an allOf is an uncommon
                // pattern so this is likely good enough for the moment.
                all_of: Some(vec![additional_schema.clone(), prop_schema.clone()]),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use schemars::schema::InstanceType;
    use serde_json::json;

    use crate::merge::merge_so_instance_type;

    use super::try_merge_schema;

    #[test]
    fn test_simple_merge() {
        let a = json!({
            "type": "object",
            "properties": {
                "result": {
                    "type": "string"
                }
            }
        });
        let b = json!({
            "required": ["result", "msg"],
            "properties": {
                "result": {
                    "enum": ["success"]
                },
                "msg": {
                    "type": "string"
                }
            }
        });
        let ab = json!({
            "type": "object",
            "required": ["result", "msg"],
            "properties": {
                "result": {
                    "type": "string",
                    "enum": ["success"]
                },
                "msg": {
                    "type": "string"
                }
            }
        });

        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();
        let ab = serde_json::from_value(ab).unwrap();

        let merged = try_merge_schema(&a, &b, &BTreeMap::default()).unwrap();

        assert_eq!(merged, ab);
    }

    #[test]
    fn test_nop_merge() {
        let a = json!({
                "type": "object",
                "required": [
                  "avatar_url",
                  "events_url",
                  "followers_url",
                  "following_url",
                  "gists_url",
                  "gravatar_id",
                  "html_url",
                  "id",
                  "login",
                  "node_id",
                  "organizations_url",
                  "received_events_url",
                  "repos_url",
                  "site_admin",
                  "starred_url",
                  "subscriptions_url",
                  "type",
                  "url"
                ],
                "properties": {
                  "avatar_url": {
                    "type": "string",
                    "format": "uri"
                  },
                  "email": {
                    "type": [
                      "string",
                      "null"
                    ]
                  },
                  "events_url": {
                    "type": "string",
                    "format": "uri-template"
                  },
                  "followers_url": {
                    "type": "string",
                    "format": "uri"
                  },
                  "following_url": {
                    "type": "string",
                    "format": "uri-template"
                  },
                  "gists_url": {
                    "type": "string",
                    "format": "uri-template"
                  },
                  "gravatar_id": {
                    "type": "string"
                  },
                  "html_url": {
                    "type": "string",
                    "format": "uri"
                  },
                  "id": {
                    "type": "integer"
                  },
                  "login": {
                    "type": "string"
                  },
                  "name": {
                    "type": "string"
                  },
                  "node_id": {
                    "type": "string"
                  },
                  "organizations_url": {
                    "type": "string",
                    "format": "uri"
                  },
                  "received_events_url": {
                    "type": "string",
                    "format": "uri"
                  },
                  "repos_url": {
                    "type": "string",
                    "format": "uri"
                  },
                  "site_admin": {
                    "type": "boolean"
                  },
                  "starred_url": {
                    "type": "string",
                    "format": "uri-template"
                  },
                  "subscriptions_url": {
                    "type": "string",
                    "format": "uri"
                  },
                  "type": {
                    "type": "string",
                    "enum": [
                      "Bot",
                      "User",
                      "Organization"
                    ]
                  },
                  "url": {
                    "type": "string",
                    "format": "uri"
                  }
                },
                "additionalProperties": false
              }

        );
        let b = json!({});

        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();

        let merged = try_merge_schema(&a, &b, &BTreeMap::default()).unwrap();

        assert_eq!(merged, a);
    }

    #[test]
    fn test_merge_instance_types() {
        // Simple cases
        assert_eq!(merge_so_instance_type(None, None), Ok(None));

        assert_eq!(
            merge_so_instance_type(None, Some(&InstanceType::Integer.into())),
            Ok(Some(InstanceType::Integer.into())),
        );
        assert_eq!(
            merge_so_instance_type(Some(&InstanceType::Null.into()), None),
            Ok(Some(InstanceType::Null.into())),
        );

        // Containment
        assert_eq!(
            merge_so_instance_type(
                Some(&vec![InstanceType::Integer, InstanceType::Number].into()),
                Some(&InstanceType::Integer.into())
            ),
            Ok(Some(InstanceType::Integer.into())),
        );
        assert_eq!(
            merge_so_instance_type(
                Some(&vec![InstanceType::Integer, InstanceType::Number].into()),
                Some(&InstanceType::Null.into())
            ),
            Err(()),
        );
        assert_eq!(
            merge_so_instance_type(
                Some(&vec![InstanceType::Integer, InstanceType::Number].into()),
                Some(&vec![InstanceType::Integer, InstanceType::Null].into()),
            ),
            Ok(Some(InstanceType::Integer.into())),
        );
        assert_eq!(
            merge_so_instance_type(
                Some(
                    &vec![
                        InstanceType::Object,
                        InstanceType::Integer,
                        InstanceType::Number
                    ]
                    .into()
                ),
                Some(
                    &vec![
                        InstanceType::Object,
                        InstanceType::Integer,
                        InstanceType::Null
                    ]
                    .into()
                ),
            ),
            Ok(Some(
                vec![InstanceType::Object, InstanceType::Integer,].into()
            )),
        );
        assert_eq!(
            merge_so_instance_type(
                Some(
                    &vec![
                        InstanceType::Object,
                        InstanceType::Integer,
                        InstanceType::Number
                    ]
                    .into()
                ),
                Some(
                    &vec![
                        InstanceType::Array,
                        InstanceType::Boolean,
                        InstanceType::Null
                    ]
                    .into()
                ),
            ),
            Err(()),
        );
    }

    #[test]
    fn test_array_fail() {
        let a = json!({
            "type": "array",
            "items": { "type": "integer" }
        });
        let b = json!({
            "type": "array",
            "items": { "type": "string" }
        });
        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();

        let ab = try_merge_schema(&a, &b, &Default::default());

        assert!(ab.is_err());

        let a = json!({
            "type": "array",
            "items": [{ "type": "integer" }, {"type": "object"}]
        });
        let b = json!({
            "type": "array",
            "items": { "type": "string" }
        });
        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();

        let ab = try_merge_schema(&a, &b, &Default::default());

        assert!(ab.is_err());

        let a = json!({
            "type": "array",
            "items": [{ "type": "integer" }, {"type": "object"}]
        });
        let b = json!({
            "type": "array",
            "items": [{ "type": "integer" }, { "type": "string" }]
        });
        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();

        let ab = try_merge_schema(&a, &b, &Default::default());

        assert!(ab.is_err());

        let a = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" }
            ],
            "maxItems": 4
        });
        let b = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" }
            ],
            "additionalItems": { "type": "string" },
            "maxItems": 3
        });
        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();

        let ab = try_merge_schema(&a, &b, &Default::default());

        assert!(ab.is_err());
    }

    #[test]
    fn test_array_good() {
        let a = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" }
            ],
            "maxItems": 4
        });
        let b = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" }
            ],
            "additionalItems": { "type": "integer" },
            "maxItems": 3
        });
        let ab = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" }
            ],
            "maxItems": 3
        });

        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();
        let ab = serde_json::from_value(ab).unwrap();

        let merged = try_merge_schema(&a, &b, &BTreeMap::default()).unwrap();
        assert_eq!(merged, ab);

        let a = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" }
            ],
            "maxItems": 4
        });
        let b = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" }
            ],
            "additionalItems": true,
        });
        let ab = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" }
            ],
            "additionalItems": true,
            "maxItems": 4
        });

        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();
        let ab = serde_json::from_value(ab).unwrap();

        let merged = try_merge_schema(&a, &b, &BTreeMap::default()).unwrap();
        assert_eq!(merged, ab);
    }
}
