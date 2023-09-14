// Copyright 2023 Oxide Computer Company

use std::{
    collections::{BTreeMap, BTreeSet},
    iter::repeat,
};

use schemars::{
    schema::{
        ArrayValidation, InstanceType, NumberValidation, ObjectValidation, Schema, SchemaObject,
        SingleOrVec, StringValidation, SubschemaValidation,
    },
    Map,
};

/// Merge two schemas returning the resulting schema. If the two schemas are
/// incompatible (i.e. if there is no data that can satisfy them both
/// simultaneously) then this returns `Schema::Bool(false)`.
pub fn merge_schema(a: &Schema, b: &Schema) -> Schema {
    match try_merge_schema(a, b) {
        Ok(schema) => schema,
        // An error indicates that there is no value that satisfies both
        // schemas.
        Err(()) => Schema::Bool(false),
    }
}

fn merge_maybe_schema(a: Option<&Schema>, b: Option<&Schema>) -> Option<Schema> {
    match (a, b) {
        (None, other) | (other, None) => other.cloned(),
        (Some(aa), Some(bb)) => Some(merge_schema(aa, bb)),
    }
}

pub fn try_merge_schema(a: &Schema, b: &Schema) -> Result<Schema, ()> {
    match (a, b) {
        (Schema::Bool(false), _) | (_, Schema::Bool(false)) => Ok(Schema::Bool(false)),
        (Schema::Bool(true), other) | (other, Schema::Bool(true)) => Ok(other.clone()),
        (Schema::Object(aa), Schema::Object(bb)) => merge_schema_object(aa, bb),
    }
}

fn merge_schema_object(a: &SchemaObject, b: &SchemaObject) -> Result<Schema, ()> {
    // let SchemaObject {
    //     metadata,
    //     instance_type,
    //     format,
    //     enum_values,
    //     const_value,
    //     subschemas,
    //     number,
    //     string,
    //     array,
    //     object,
    //     reference,
    //     extensions,
    // } = aa;

    let instance_type = merge_so_instance_type(a.instance_type.as_ref(), b.instance_type.as_ref())?;
    let format = merge_so_format(a.format.as_ref(), b.format.as_ref())?;

    // TODO enum_values and const_value need validation against the other schema

    // I can imagine how we might handle this, but it seems like both a pain in
    // the neck and an odd construct given that the schemas were merging likely
    // came from a subschema construct.
    assert!(a.subschemas.is_none());
    assert!(b.subschemas.is_none());

    let number = merge_so_number(a.number.as_deref(), b.number.as_deref())?;
    let string = merge_so_string(a.string.as_deref(), b.string.as_deref())?;
    let array = merge_so_array(a.array.as_deref(), b.array.as_deref())?;
    let object = merge_so_object(a.object.as_deref(), b.object.as_deref())?;

    // We could clean up this schema to eliminate data irrelevant to the
    // instance type, but logic in the conversion path should already handle
    // that.
    Ok((SchemaObject {
        instance_type,
        format,
        number,
        string,
        array,
        object,
        ..Default::default()
    })
    .into())
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
        (Some(aa), Some(bb)) => {
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
        (Some(aa), Some(bb)) => {
            unimplemented!("this is fairly fussy and I don't want to do it")
        }
    }
}

fn merge_so_array(
    a: Option<&ArrayValidation>,
    b: Option<&ArrayValidation>,
) -> Result<Option<Box<ArrayValidation>>, ()> {
    match (a, b) {
        (None, other) | (other, None) => Ok(other.cloned().map(Box::new)),
        (Some(aa), Some(bb)) => {
            let ArrayValidation {
                items,
                additional_items,
                max_items,
                min_items,
                unique_items,
                contains,
            } = aa;

            let max_items = choose_value(aa.max_items, bb.max_items, Ord::min);
            let min_items = choose_value(aa.min_items, bb.min_items, Ord::max);

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

            let (items, additional_items) = match (
                (&aa.items, &aa.additional_items),
                (&bb.items, &bb.additional_items),
            ) {
                // Both items are none; items and additional_items are None.
                ((None, _), (None, _)) => (None, None),
                ((None, _), (Some(SingleOrVec::Single(item)), _))
                | ((Some(SingleOrVec::Single(item)), _), (None, _)) => {
                    (Some(SingleOrVec::Single(item.clone())), None)
                }

                ((None, _), (Some(SingleOrVec::Vec(items)), additional_items))
                | ((Some(SingleOrVec::Vec(items)), additional_items), (None, _)) => {
                    match (max_items, items.len()) {
                        // If the number of item schemas is at least as large
                        // as the maximum number if items then we don't need
                        // any additional_items.
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

                // Two single schemas, just merge them.
                (
                    (Some(SingleOrVec::Single(aa_single)), _),
                    (Some(SingleOrVec::Single(bb_single)), _),
                ) => (
                    Some(SingleOrVec::Single(Box::new(merge_schema(
                        aa_single, bb_single,
                    )))),
                    None,
                ),

                // A single item and an array if schemas.
                (
                    (Some(SingleOrVec::Single(single)), _),
                    (Some(SingleOrVec::Vec(items)), additional_items),
                )
                | (
                    (Some(SingleOrVec::Vec(items)), additional_items),
                    (Some(SingleOrVec::Single(single)), _),
                ) => {
                    match (max_items, items.len()) {
                        // If the number of item schemas is at least as large
                        // as the maximum number if items then we don't need
                        // any additional_items.
                        (Some(max), len) if len >= max as usize => (
                            Some(SingleOrVec::Vec(
                                items
                                    .iter()
                                    .take(max as usize)
                                    .map(|item_schema| merge_schema(item_schema, single))
                                    .collect(),
                            )),
                            None,
                        ),
                        _ => {
                            let items = items
                                .iter()
                                .map(|item_schema| merge_schema(item_schema, single))
                                .collect();
                            let additional_items = additional_items.as_deref().map_or_else(
                                || single.as_ref().clone(),
                                |additional_schema| merge_schema(additional_schema, single),
                            );
                            (
                                Some(SingleOrVec::Vec(items)),
                                Some(Box::new(additional_items)),
                            )
                        }
                    }
                }

                (
                    (Some(SingleOrVec::Vec(aa_items)), aa_additional_items),
                    (Some(SingleOrVec::Vec(bb_items)), bb_additional_items),
                ) => {
                    let items_len = aa_items.len().max(bb_items.len());

                    match max_items {
                        Some(max) if items_len <= max as usize => {
                            todo!();
                            todo!()
                        }

                        _ => {
                            let aa_items_iter = aa_items
                                .iter()
                                .map(Some)
                                .chain(repeat(aa_additional_items.as_deref()));
                            let bb_items_iter = bb_items
                                .iter()
                                .map(Some)
                                .chain(repeat(bb_additional_items.as_deref()));

                            let items = aa_items_iter
                                .zip(bb_items_iter)
                                .take(items_len)
                                .map(|schemas| match schemas {
                                    (None, None) => unreachable!(),
                                    (None, Some(item)) => item.clone(),
                                    (Some(item), None) => item.clone(),
                                    (Some(aa_item), Some(bb_item)) => {
                                        merge_schema(aa_item, bb_item)
                                    }
                                })
                                .collect();
                            let additional_items = merge_maybe_schema(
                                aa_additional_items.as_deref(),
                                bb_additional_items.as_deref(),
                            )
                            .map(Box::new);

                            (Some(SingleOrVec::Vec(items)), additional_items)
                        }
                    }
                }

                // (None, other) => (other.clone(), None),
                // (Some(_), None) => todo!(),
                // (Some(_), Some(_)) => todo!(),
                _ => todo!(),
            };

            let xxx = ArrayValidation {
                items,
                additional_items,
                max_items,
                min_items,
                unique_items: todo!(),
                contains: todo!(),
            };

            unimplemented!("this is fairly fussy and I don't want to do it")
        }
    }
}

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
) -> Result<Option<Box<ObjectValidation>>, ()> {
    match (a, b) {
        (None, other) | (other, None) => Ok(other.cloned().map(Box::new)),
        (Some(aa), Some(bb)) => {
            let ObjectValidation {
                max_properties,
                min_properties,
                required,
                properties,
                pattern_properties,
                additional_properties,
                property_names,
            } = aa;

            let a_props = aa.properties.iter().filter_map(|(name, a_schema)| {
                let resolved_schema = if let Some(b_schema) = bb.properties.get(name) {
                    try_merge_schema(a_schema, b_schema)
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
    use schemars::schema::InstanceType;
    use serde_json::json;

    use crate::merge::merge_so_instance_type;

    use super::try_merge_schema;

    #[test]
    fn xxx() {
        // let a = json!({
        //     "type": "object",
        //     "properties": {
        //         "result": {
        //             "type": "string"
        //         }
        //     }
        // });
        // let b = json!({
        //     "required": ["result", "msg"],
        //     "properties": {
        //         "result": {
        //             "enum": ["success"]
        //         },
        //         "msg": {
        //             "type": "string"
        //         }
        //     }
        // });
        let a = json!({
            "type": "object",
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
        let b = json!({
            "additionalProperties":false,
            "properties": {
                "result":{},
                "msg":{}
            }
        });

        let a = serde_json::from_value(a).unwrap();
        let b = serde_json::from_value(b).unwrap();

        let x = try_merge_schema(&a, &b);

        println!("{}", serde_json::to_string_pretty(&x.unwrap()).unwrap());
        panic!();
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
}
