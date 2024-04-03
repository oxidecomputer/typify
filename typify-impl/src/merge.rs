// Copyright 2024 Oxide Computer Company

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
    try_merge_all(schemas, defs).unwrap_or(Schema::Bool(false))
}

fn try_merge_all(schemas: &[Schema], defs: &BTreeMap<RefKey, Schema>) -> Result<Schema, ()> {
    debug!(
        "merge all {}",
        serde_json::to_string_pretty(schemas).unwrap(),
    );

    let merged_schema = match schemas {
        [] => panic!("we should not be trying to merge an empty array of schemas"),
        [only] => only.clone(),
        [first, second, rest @ ..] => {
            let mut out = try_merge_schema(first, second, defs)?;
            for schema in rest {
                out = try_merge_schema(&out, schema, defs)?;
            }
            out
        }
    };

    Ok(merged_schema)
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
        _ => merge_additional_properties(a, b, defs),
    }
}

/// Given two additionalProperties schemas that might be None--which is
/// equivalent to Schema::Bool(true)--this returns the appropriate value. We
/// prefer None to `true` for objects since the named properties are th main
/// event.
fn merge_additional_properties(
    a: Option<&Schema>,
    b: Option<&Schema>,
    defs: &BTreeMap<RefKey, Schema>,
) -> Option<Schema> {
    match (a, b) {
        (None, other) | (other, None) => other.cloned(),
        (Some(aa), Some(bb)) => {
            Some(try_merge_schema(aa, bb, defs).unwrap_or_else(|_| Schema::Bool(false)))
        }
    }
}

fn merge_schema(a: &Schema, b: &Schema, defs: &BTreeMap<RefKey, Schema>) -> Schema {
    try_merge_schema(a, b, defs).unwrap_or(Schema::Bool(false))
}

/// Merge two schemas returning the resulting schema. If the two schemas are
/// incompatible (i.e. if there is no data that can satisfy them both
/// simultaneously) then this returns Err.
fn try_merge_schema(a: &Schema, b: &Schema, defs: &BTreeMap<RefKey, Schema>) -> Result<Schema, ()> {
    // dbg!((a,b));
    match (a, b) {
        (Schema::Bool(false), _) | (_, Schema::Bool(false)) => Err(()),
        (Schema::Bool(true), other) | (other, Schema::Bool(true)) => Ok(other.clone()),

        // If we have two references to the same schema, that's easy!
        (
            Schema::Object(SchemaObject {
                reference: Some(a_ref_name),
                ..
            }),
            Schema::Object(SchemaObject {
                reference: Some(b_ref_name),
                ..
            }),
        ) if a_ref_name == b_ref_name => Ok(Schema::Object(SchemaObject {
            reference: Some(a_ref_name.clone()),
            ..Default::default()
        })),

        // Resolve references here before we start to merge the objects.
        //
        // TODO: need to mitigate circular references so we don't go into a
        // spin loop. We can do this by wrapping defs in a structure that
        // remembers what we've already looked up; if we hit a cycle we can
        // consider the proper handling, but it might be to ignore it--a
        // circular allOf chain is a bit hard to reason about.
        (
            ref_schema @ Schema::Object(SchemaObject {
                reference: Some(ref_name),
                ..
            }),
            other,
        )
        | (
            other,
            ref_schema @ Schema::Object(SchemaObject {
                reference: Some(ref_name),
                ..
            }),
        ) => {
            let key = ref_key(ref_name);
            let resolved = defs
                .get(&key)
                .unwrap_or_else(|| panic!("unresolved reference: {}", ref_name));
            let merged_schema = try_merge_schema(resolved, other, defs)?;
            // If we merge a referenced schema with another schema **and**
            // the resulting schema is equivalent to the referenced schema
            // (i.e. the other schema is identical or less permissive) then we
            // just return the reference schema rather than its contents.
            if merged_schema.roughly(resolved) {
                Ok(ref_schema.clone())
            } else {
                Ok(merged_schema)
            }
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

    let enum_values = merge_so_enum_values(
        a.enum_values.as_ref(),
        a.const_value.as_ref(),
        b.enum_values.as_ref(),
        b.const_value.as_ref(),
    )?;

    // We could clean up this schema to eliminate data irrelevant to the
    // instance type, but logic in the conversion path should already handle
    // that.
    let mut merged_schema = SchemaObject {
        metadata: None,
        instance_type,
        format,
        enum_values,
        const_value: None,
        subschemas: None,
        number,
        string,
        array,
        object,
        reference: None,
        extensions: Default::default(),
    };

    // TODO if the merged schema is Default::default() then we should probably
    // take some shortcut here...

    // If we have subschemas for either schema then we merge the body of the
    // two schemas and then do the appropriate merge with subschemas (i.e.
    // potentially twice). This is effectively an `allOf` between the merged
    // "body" schema and the component subschemas.
    merged_schema = try_merge_with_subschemas(merged_schema, a.subschemas.as_deref(), defs)?;
    merged_schema = try_merge_with_subschemas(merged_schema, b.subschemas.as_deref(), defs)?;

    assert_ne!(merged_schema, Schema::Bool(false).into_object());

    // Now that we've finalized the schemas, we take a pass through the
    // enumerated values (if there are any) to weed out any that might be
    // invalid.
    if let Some(enum_values) = merged_schema.enum_values.take() {
        let wrapped_schema = Schema::Object(merged_schema);
        let enum_values = Some(
            enum_values
                .into_iter()
                .filter(|value| schema_value_validate(&wrapped_schema, value, defs).is_ok())
                .collect(),
        );
        let Schema::Object(new_merged_schema) = wrapped_schema else {
            unreachable!()
        };
        merged_schema = new_merged_schema;
        merged_schema.enum_values = enum_values;
    }

    debug!(
        "merged {}\n{}\n|\nv\n{}",
        serde_json::to_string_pretty(a).unwrap(),
        serde_json::to_string_pretty(b).unwrap(),
        serde_json::to_string_pretty(&merged_schema).unwrap(),
    );

    Ok(merged_schema)
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

/// Merge the schema with a subschema validation object. It's important that
/// the return value reduces the complexity of the problem so avoid infinite
/// recursion.
pub(crate) fn try_merge_with_subschemas(
    mut schema_object: SchemaObject,
    maybe_subschemas: Option<&SubschemaValidation>,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<SchemaObject, ()> {
    let Some(SubschemaValidation {
        all_of,
        any_of,
        one_of,
        not,
        if_schema,
        then_schema,
        else_schema,
    }) = maybe_subschemas
    else {
        return Ok(schema_object);
    };

    if if_schema.is_some() || then_schema.is_some() || else_schema.is_some() {
        unimplemented!("if/then/else schemas are not supported");
    }

    if let Some(all_of) = all_of {
        let merged_schema = all_of
            .iter()
            .try_fold(schema_object.into(), |schema, other| {
                try_merge_schema(&schema, other, defs)
            })?;
        assert_ne!(merged_schema, Schema::Bool(false));
        schema_object = merged_schema.into_object();
    }

    if let Some(not) = not {
        schema_object = try_merge_schema_not(schema_object, not.as_ref(), defs)?;
    }

    // TODO: we should be able to handle a combined one_of and any_of... but
    // I don't want to do that now because that would be a very strange
    // construction.
    assert!(any_of.is_none() || one_of.is_none());

    if let Some(any_of) = any_of {
        let merged_subschemas = try_merge_with_each_subschema(&schema_object, any_of, defs);

        match merged_subschemas.len() {
            0 => return Err(()),
            1 => schema_object = merged_subschemas.into_iter().next().unwrap().into_object(),
            _ => {
                schema_object = SchemaObject {
                    metadata: schema_object.metadata,
                    subschemas: Some(Box::new(SubschemaValidation {
                        any_of: Some(merged_subschemas),
                        ..Default::default()
                    })),
                    ..Default::default()
                }
            }
        }
    }

    if let Some(one_of) = one_of {
        let merged_subschemas = try_merge_with_each_subschema(&schema_object, one_of, defs);

        match merged_subschemas.len() {
            0 => return Err(()),
            1 => schema_object = merged_subschemas.into_iter().next().unwrap().into_object(),
            _ => {
                schema_object = SchemaObject {
                    metadata: schema_object.metadata,
                    subschemas: Some(Box::new(SubschemaValidation {
                        one_of: Some(merged_subschemas),
                        ..Default::default()
                    })),
                    ..Default::default()
                }
            }
        }
    }

    Ok(schema_object)
}

fn try_merge_with_each_subschema(
    schema_object: &SchemaObject,
    subschemas: &[Schema],
    defs: &BTreeMap<RefKey, Schema>,
) -> Vec<Schema> {
    let schema = Schema::Object(schema_object.clone());
    // First we do a pairwise merge the schemas; if the result is invalid /
    // unresolvable / never / whatever, we exclude it from the list. If it is
    // valid, *then* we do the join to preserve information (though we probably
    // only need to to *that* if at least one schema contains a ref). This
    // could probably be an opportunity for memoization, but this is an
    // infrequent construction so... whatever for now.
    let joined_schemas = subschemas
        .iter()
        .enumerate()
        .filter_map(|(ii, other)| {
            // Skip if the merged schema is unsatisfiable.
            let merged_schema = try_merge_schema(&schema, other, defs).ok()?;
            // If the merged schema is equivalent to one or other of the
            // individual schemas, use that.
            // TODO is this right? Should we be "subtracting" out other schemas as below?
            if merged_schema.roughly(&schema) {
                Some(schema.clone())
            } else if merged_schema.roughly(other) {
                Some(other.clone())
            } else {
                let not_others = subschemas
                    .iter()
                    .enumerate()
                    .filter(|(jj, _)| *jj != ii)
                    .map(|(_, not_schema)| {
                        Schema::Object(SchemaObject {
                            subschemas: Some(Box::new(SubschemaValidation {
                                not: Some(Box::new(not_schema.clone())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        })
                    });
                let joined_schema = [schema.clone(), other.clone()]
                    .into_iter()
                    .chain(not_others)
                    .collect::<Vec<_>>();
                Some(
                    SchemaObject {
                        subschemas: Some(Box::new(SubschemaValidation {
                            all_of: Some(joined_schema),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }
                    .into(),
                )
            }
        })
        .collect::<Vec<_>>();

    joined_schemas
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
    debug!(
        "try_merge_schema_not {}\n not:{}",
        serde_json::to_string_pretty(&schema_object).unwrap(),
        serde_json::to_string_pretty(not_schema).unwrap(),
    );
    match not_schema {
        // Subtracting everything leaves nothing...
        Schema::Bool(true) => Err(()),
        // ... whereas subtracting nothing leaves everything.
        Schema::Bool(false) => Ok(schema_object),

        Schema::Object(SchemaObject {
            // I don't think there's any significance to the schema metadata
            // with respect to the types we might generate.
            metadata: _,
            // TODO we should should check instance_type and then walk through
            // validation of each type based on the specific validation.
            instance_type: _,
            format: _,
            enum_values: _,
            const_value: _,
            subschemas,
            number: _,
            string: _,
            array: _,
            object,
            // TODO we might want to chase these references but need to take
            // care to handle circular references.
            reference: _,
            extensions: _,
        }) => {
            if let Some(not_object) = object {
                // TODO this is incomplete, but seems sufficient for the
                // schemas we've seen in the wild.
                if let Some(ObjectValidation {
                    required,
                    properties,
                    ..
                }) = schema_object.object.as_deref_mut()
                {
                    // TODO This is completely wrong for arrays of len > 1.
                    // We need to treat required: [x, y] like it's:
                    //   not:
                    //     allOf:
                    //       required: [x]
                    //       required: [y]
                    // Then we can transform them into:
                    //   anyOf:
                    //     not:
                    //       required: [x]
                    //     not:
                    //       required: [y]
                    // Which in turn can become:
                    //   oneOf:
                    //     not:
                    //       required: [x]
                    //     not:
                    //       required: [y]
                    //     not:
                    //       required: [x, y]
                    for not_required in &not_object.required {
                        // A property can't be both required and not required
                        // therefore this schema is unsatisfiable.
                        if required.contains(not_required) {
                            return Err(());
                        }
                        // Set the property's schema to false i.e. that the
                        // presence of any value would be invalid. We ignore
                        // the return value as it doesn't matter if the
                        // property was there previously or not.
                        let _ = properties.insert(not_required.clone(), Schema::Bool(false));
                    }
                }
            }

            if let Some(not_subschemas) = subschemas {
                schema_object = try_merge_with_subschemas_not(schema_object, not_subschemas, defs)?;
            }

            Ok(schema_object)
        }
    }
}

fn try_merge_with_subschemas_not(
    schema_object: SchemaObject,
    not_subschemas: &SubschemaValidation,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<SchemaObject, ()> {
    debug!("try_merge_with_subschemas_not");
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
            not: Some(not),
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => {
            debug!("not not");
            Ok(try_merge_schema(&schema_object.into(), not.as_ref(), defs)?.into_object())
        }

        // TODO this is a kludge
        SubschemaValidation {
            all_of: None,
            any_of: None,
            one_of: Some(_),
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => Ok(schema_object),

        SubschemaValidation {
            all_of: None,
            any_of: None,
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => Ok(schema_object),

        SubschemaValidation {
            all_of: Some(all_of),
            any_of: None,
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => match try_merge_all(all_of, defs) {
            Ok(merged_not_schema) => try_merge_schema_not(schema_object, &merged_not_schema, defs),
            Err(_) => Ok(schema_object),
        },

        _ => todo!(
            "{}\nnot: {}",
            serde_json::to_string_pretty(&schema_object).unwrap(),
            serde_json::to_string_pretty(&not_subschemas).unwrap(),
        ),
    }
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
        (Some(a), Some(b)) if a == b => Ok(Some(Box::new(a.clone()))),
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
        (Some(a), Some(b)) if a == b => Ok(Some(Box::new(a.clone()))),
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
            // together, and the results of merging can affect the max.
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

            let (items, additional_items, max_items) = match (
                (&aa.items, &aa.additional_items),
                (&bb.items, &bb.additional_items),
            ) {
                // Both items are none; items and additional_items are None.
                ((None, _), (None, _)) => (None, None, max_items),

                // A None and a single-item; we can use the single item and
                // additional_items are irrelevant.
                ((None, _), (Some(SingleOrVec::Single(item)), _))
                | ((Some(SingleOrVec::Single(item)), _), (None, _)) => {
                    (Some(SingleOrVec::Single(item.clone())), None, max_items)
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
                            max_items,
                        ),
                        _ => (
                            Some(SingleOrVec::Vec(items.clone())),
                            additional_items.clone(),
                            max_items,
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
                    max_items,
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
                ) => {
                    let (items, allow_additional_items) = merge_items_array(
                        items.iter().zip(repeat(single.as_ref())),
                        min_items,
                        max_items,
                        defs,
                    )?;

                    if allow_additional_items {
                        let additional_items = additional_items.as_deref().map_or_else(
                            || Ok(single.as_ref().clone()),
                            |additional_schema| try_merge_schema(additional_schema, single, defs),
                        )?;
                        (
                            Some(SingleOrVec::Vec(items)),
                            Some(Box::new(additional_items)),
                            max_items,
                        )
                    } else {
                        let len = items.len() as u32;
                        (Some(SingleOrVec::Vec(items)), None, Some(len))
                    }
                }

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
                    // always irrelevant because we will always .take(..) the
                    // shorter of the two (and may consume even fewer). We just
                    // chain them both for simplicity and don't sweat it.
                    let aa_items_iter = aa_items.iter().chain(repeat(
                        aa_additional_items
                            .as_deref()
                            .unwrap_or_else(|| &Schema::Bool(true)),
                    ));
                    let bb_items_iter = bb_items.iter().chain(repeat(
                        bb_additional_items
                            .as_deref()
                            .unwrap_or_else(|| &Schema::Bool(true)),
                    ));
                    let items_iter = aa_items_iter.zip(bb_items_iter).take(items_len);
                    let (items, allow_additional_items) =
                        merge_items_array(items_iter, min_items, max_items, defs)?;
                    if allow_additional_items {
                        let additional_items = merge_additional_items(
                            aa_additional_items.as_deref(),
                            bb_additional_items.as_deref(),
                            defs,
                        );
                        (
                            Some(SingleOrVec::Vec(items)),
                            additional_items.map(Box::new),
                            max_items,
                        )
                    } else {
                        let len = items.len() as u32;
                        (Some(SingleOrVec::Vec(items)), None, Some(len))
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

fn merge_items_array<'a>(
    items_iter: impl Iterator<Item = (&'a Schema, &'a Schema)>,
    min_items: Option<u32>,
    max_items: Option<u32>,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<(Vec<Schema>, bool), ()> {
    let mut items = Vec::new();
    for (a, b) in items_iter {
        match try_merge_schema(a, b, defs) {
            Ok(schema) => {
                items.push(schema);
                if let Some(max) = max_items {
                    if items.len() == max as usize {
                        return Ok((items, false));
                    }
                }
            }
            Err(_) => {
                let len = items.len() as u32;
                if len < min_items.unwrap_or(1) {
                    return Err(());
                }
                return Ok((items, false));
            }
        }
    }

    Ok((items, true))
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
            let required = aa
                .required
                .union(&bb.required)
                .cloned()
                .collect::<BTreeSet<_>>();
            let additional_properties = merge_additional_properties(
                aa.additional_properties.as_deref(),
                bb.additional_properties.as_deref(),
                defs,
            );

            enum AOrB<'a> {
                A(&'a Schema),
                B(&'a Schema),
                Both(&'a Schema, &'a Schema),
            }

            let properties = aa
                .properties
                .iter()
                // First characterize properties as being in a, b, or both.
                .map(|(name, a_schema)| {
                    if let Some(b_schema) = bb.properties.get(name) {
                        (name, AOrB::Both(a_schema, b_schema))
                    } else {
                        (name, AOrB::A(a_schema))
                    }
                })
                .chain(bb.properties.iter().filter_map(|(name, b_schema)| {
                    if aa.properties.contains_key(name) {
                        None
                    } else {
                        Some((name, AOrB::B(b_schema)))
                    }
                }))
                // Then resolve properties against the other full schema or
                // against the schemas for the properties if it appears in
                // both.
                .filter_map(|(name, ab_schema)| {
                    let resolved_schema = match ab_schema {
                        AOrB::A(a_schema) => filter_prop(name, a_schema, bb),
                        AOrB::B(b_schema) => filter_prop(name, b_schema, aa),
                        AOrB::Both(a_schema, b_schema) => merge_schema(a_schema, b_schema, defs),
                    };
                    match resolved_schema {
                        // If a required field is incompatible with the
                        // other schema, this object is unsatisfiable.
                        Schema::Bool(false) if required.contains(name) => Some(Err(())),

                        // For incompatible, non-required fields we need to
                        // exclude the property from any values. If
                        // `additionalProperties` is `false` (i.e. excludes all
                        // other properties) then we can simply omit the
                        // property knowing that it (like all other unnamed
                        // properties) will not be permitted. Otherwise we
                        // include the optional property but with the `false`
                        // schema that means that no value will satisfy that
                        // property--the value would always be None and any
                        // serialization that included the named property would
                        // fail to deserialize.
                        //
                        // If we ever make use of `propertyNames`, it's
                        // conceivable that we might check it or modify it in
                        // this case, but that may be overly complex.
                        Schema::Bool(false) => {
                            if let Some(Schema::Bool(false)) = additional_properties {
                                None
                            } else {
                                Some(Ok((name.clone(), Schema::Bool(false))))
                            }
                        }

                        // Compatible schema; proceed.
                        schema => Some(Ok((name.clone(), schema))),
                    }
                })
                .collect::<Result<schemars::Map<_, _>, _>>()?;

            let max_properties = choose_value(aa.max_properties, bb.max_properties, Ord::min);
            let min_properties = choose_value(aa.min_properties, bb.min_properties, Ord::max);

            if let (Some(min), Some(max)) = (min_properties, max_properties) {
                if min > max {
                    return Err(());
                }
            }

            let object_validation = ObjectValidation {
                required,
                properties,
                additional_properties: additional_properties.map(Box::new),
                max_properties,
                min_properties,
                pattern_properties: Default::default(), // TODO
                property_names: Default::default(),     // TODO
            };
            Ok(Some(object_validation.into()))
        }
    }
}

fn filter_prop(name: &str, prop_schema: &Schema, object_schema: &ObjectValidation) -> Schema {
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
        .unwrap_or(Schema::Bool(false))
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

trait Roughly {
    fn roughly(&self, other: &Self) -> bool;
}

impl Roughly for schemars::schema::Schema {
    fn roughly(&self, other: &Self) -> bool {
        match (self, other) {
            (Schema::Bool(a), Schema::Bool(b)) => a == b,
            (Schema::Bool(false), _) | (_, Schema::Bool(false)) => false,

            (Schema::Bool(true), Schema::Object(other))
            | (Schema::Object(other), Schema::Bool(true)) => matches!(
                other,
                SchemaObject {
                    metadata: _,
                    instance_type: None,
                    format: None,
                    enum_values: None,
                    const_value: None,
                    subschemas: None,
                    number: None,
                    string: None,
                    array: None,
                    object: None,
                    reference: None,
                    extensions: _,
                }
            ),

            (Schema::Object(a), Schema::Object(b)) => {
                a.instance_type == b.instance_type
                    && a.format == b.format
                    && a.enum_values == b.enum_values
                    && a.const_value == b.const_value
                    && roughly_subschemas(a.subschemas.as_deref(), b.subschemas.as_deref())
                    && a.number == b.number
                    && a.string == b.string
                    && roughly_array(a.array.as_deref(), b.array.as_deref())
                    && roughly_object(a.object.as_deref(), b.object.as_deref())
                    && a.reference == b.reference
            }
        }
    }
}

fn roughly_subschemas(a: Option<&SubschemaValidation>, b: Option<&SubschemaValidation>) -> bool {
    match (a, b) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(aa), Some(bb)) => {
            roughly_schema_array(aa.all_of.as_deref(), bb.all_of.as_deref())
                && roughly_schema_array(aa.any_of.as_deref(), bb.any_of.as_deref())
                && roughly_schema_array(aa.one_of.as_deref(), bb.one_of.as_deref())
                && roughly_schema_option(aa.not.as_deref(), bb.not.as_deref())
                && roughly_schema_option(aa.if_schema.as_deref(), bb.if_schema.as_deref())
                && roughly_schema_option(aa.then_schema.as_deref(), bb.then_schema.as_deref())
                && roughly_schema_option(aa.else_schema.as_deref(), bb.else_schema.as_deref())
        }
    }
}

fn roughly_schema_option(a: Option<&Schema>, b: Option<&Schema>) -> bool {
    match (a, b) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(aa), Some(bb)) => aa.roughly(bb),
    }
}

fn roughly_schema_array(a: Option<&[Schema]>, b: Option<&[Schema]>) -> bool {
    match (a, b) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(aa), Some(bb)) => {
            // TODO We'll do it pairwise, but we should be looser..
            aa.len() == bb.len() && aa.iter().zip(bb.iter()).all(|(aaa, bbb)| aaa.roughly(bbb))
        }
    }
}

fn roughly_array(a: Option<&ArrayValidation>, b: Option<&ArrayValidation>) -> bool {
    match (a, b) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(aa), Some(bb)) => match (&aa.items, &bb.items) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(SingleOrVec::Single(_)), Some(SingleOrVec::Vec(_))) => false,
            (Some(SingleOrVec::Vec(_)), Some(SingleOrVec::Single(_))) => false,

            (Some(SingleOrVec::Single(aaa)), Some(SingleOrVec::Single(bbb))) => aaa.roughly(bbb),
            (Some(SingleOrVec::Vec(aaa)), Some(SingleOrVec::Vec(bbb))) => {
                roughly_schema_array(Some(aaa), Some(bbb))
            }
        },
    }
}

fn roughly_object(a: Option<&ObjectValidation>, b: Option<&ObjectValidation>) -> bool {
    match (a, b) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(aa), Some(bb)) => {
            aa.max_properties == bb.max_properties
                && aa.min_properties == bb.min_properties
                && aa.required == bb.required
                && roughly_properties(&aa.properties, &bb.properties)
                && roughly_properties(&aa.pattern_properties, &bb.pattern_properties)
                && roughly_schema_option(
                    aa.additional_properties.as_deref(),
                    bb.additional_properties.as_deref(),
                )
                && roughly_schema_option(aa.property_names.as_deref(), bb.property_names.as_deref())
        }
    }
}

fn roughly_properties(
    a: &schemars::Map<String, Schema>,
    b: &schemars::Map<String, Schema>,
) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b.iter())
            .all(|((aa_name, aa_schema), (bb_name, bb_schema))| {
                aa_name == bb_name && aa_schema.roughly(bb_schema)
            })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use schemars::schema::InstanceType;
    use serde_json::json;

    use crate::{merge::merge_so_instance_type, RefKey};

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
            "items": [{ "type": "string" }, { "type": "object" }]
        });
        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();

        let ab = try_merge_schema(&a, &b, &Default::default());

        assert!(
            ab.is_err(),
            "{}",
            serde_json::to_string_pretty(&ab).unwrap(),
        );

        let a = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" },
                { "type": "integer" }
            ],
            "minItems": 3,
            "maxItems": 4
        });
        let b = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" }
            ],
            "additionalItems": { "type": "string" },
            "maxItems": 100
        });
        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();

        let ab = try_merge_schema(&a, &b, &Default::default());

        assert!(
            ab.is_err(),
            "{}",
            serde_json::to_string_pretty(&ab).unwrap(),
        );
    }

    #[test]
    fn test_array_good1() {
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
        assert_eq!(
            merged,
            ab,
            "{}",
            serde_json::to_string_pretty(&merged).unwrap(),
        )
    }

    #[test]
    fn test_array_good2() {
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
        assert_eq!(
            merged,
            ab,
            "{}",
            serde_json::to_string_pretty(&merged).unwrap(),
        )
    }

    #[test]
    fn test_array_good3() {
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
                { "type": "integer" },
                { "type": "string" }
            ],
            "additionalItems": true,
        });
        let ab = json!({
            "type": "array",
            "items": [
                { "type": "integer" },
                { "type": "integer" }
            ],
            "maxItems": 2
        });

        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();
        let ab = serde_json::from_value(ab).unwrap();

        let merged = try_merge_schema(&a, &b, &BTreeMap::default()).unwrap();
        assert_eq!(
            merged,
            ab,
            "{}",
            serde_json::to_string_pretty(&merged).unwrap(),
        )
    }

    #[test]
    fn test_match_one_of() {
        let a = json!({
            "$ref": "#/definitions/x"
        });
        let b = json!({
            "oneOf": [
                {
                    "$ref": "#/definitions/x"
                },
                {
                    "type": "null"
                }
            ]
        });
        let x = json!({
            "type": "string"
        });
        let ab = json!({
            "$ref": "#/definitions/x"
        });

        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();
        let x: schemars::schema::Schema = serde_json::from_value(x).unwrap();
        let ab = serde_json::from_value(ab).unwrap();

        let merged = try_merge_schema(
            &a,
            &b,
            &[(RefKey::Def("x".to_string()), x)].into_iter().collect(),
        )
        .unwrap();
        assert_eq!(
            merged,
            ab,
            "{}",
            serde_json::to_string_pretty(&merged).unwrap(),
        )
    }

    #[test]
    fn test_all_of_one_of_identity() {
        let a = json!({
            "oneOf": [
                {
                    "$ref": "#/definitions/x"
                },
                {
                    "type": "null"
                }
            ]
        });
        let b = json!({
            "oneOf": [
                {
                    "$ref": "#/definitions/x"
                },
                {
                    "type": "null"
                }
            ]
        });
        let x = json!({
            "title": "x",
            "type": "string"
        });
        let ab = json!({
            "oneOf": [
                {
                    "$ref": "#/definitions/x"
                },
                {
                    "type": "null"
                }
            ]
        });

        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();
        let x: schemars::schema::Schema = serde_json::from_value(x).unwrap();
        let ab = serde_json::from_value(ab).unwrap();

        let merged = try_merge_schema(
            &a,
            &b,
            &[(RefKey::Def("x".to_string()), x)].into_iter().collect(),
        )
        .unwrap();
        assert_eq!(
            merged,
            ab,
            "{}",
            serde_json::to_string_pretty(&merged).unwrap(),
        )
    }
}
