use std::collections::BTreeSet;

use convert_case::{Case, Casing};
use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
};

use crate::Name;

pub(crate) fn metadata_description(metadata: &Option<Box<Metadata>>) -> Option<String> {
    metadata
        .as_ref()
        .and_then(|metadata| metadata.description.as_ref().cloned())
}

pub(crate) fn metadata_title(metadata: &Option<Box<Metadata>>) -> Option<String> {
    metadata
        .as_ref()
        .and_then(|metadata| metadata.title.as_ref().cloned())
}

pub(crate) fn all_mutually_exclusive(
    subschemas: &[Schema],
    definitions: &schemars::Map<String, Schema>,
) -> bool {
    let len = subschemas.len();
    // Consider all pairs
    (0..len - 1)
        .flat_map(|ii| (ii + 1..len).map(move |jj| (ii, jj)))
        .all(|(ii, jj)| {
            let a = resolve(&subschemas[ii], definitions);
            let b = resolve(&subschemas[jj], definitions);
            schemas_mutually_exclusive(a, b)
        })
}

/// This function needs to necessarily be conservative. We'd much prefer a
/// false negative than a false positive.
fn schemas_mutually_exclusive(a: &Schema, b: &Schema) -> bool {
    match (a, b) {
        // If either matches nothing then they are exclusive.
        (Schema::Bool(false), _) => true,
        (_, Schema::Bool(false)) => true,

        // If either matches anything then they are not exclusive.
        (Schema::Bool(true), _) => false,
        (_, Schema::Bool(true)) => false,

        // Neither is a Schema::Bool; we need to look at the instance types.
        (Schema::Object(a), Schema::Object(b)) => {
            match (&a.instance_type, &b.instance_type) {
                // If either is None, assume we're dealing with a more complex
                // type and that they are not exclusive.
                (None, _) => false,
                (_, None) => false,

                // If each schema has a single type and they aren't the same
                // then the types must be mutually exclusive.
                (Some(SingleOrVec::Single(a_single)), Some(SingleOrVec::Single(b_single)))
                    if a_single != b_single =>
                {
                    true
                }

                // For two objects we need to check required properties and
                // additional properties to see if there exists an object that
                // could successfully be validated by either schema.
                (Some(SingleOrVec::Single(a_single)), Some(SingleOrVec::Single(b_single)))
                    if a_single == b_single && a_single.as_ref() == &InstanceType::Object =>
                {
                    if let (
                        SchemaObject {
                            metadata: _,
                            instance_type: _,
                            format: None,
                            enum_values: None,
                            const_value: None,
                            subschemas: None,
                            number: None,
                            string: None,
                            array: None,
                            object: Some(a_validation),
                            reference: None,
                            extensions: _,
                        },
                        SchemaObject {
                            metadata: _,
                            instance_type: _,
                            format: None,
                            enum_values: None,
                            const_value: None,
                            subschemas: None,
                            number: None,
                            string: None,
                            array: None,
                            object: Some(b_validation),
                            reference: None,
                            extensions: _,
                        },
                    ) = (a, b)
                    {
                        object_schemas_mutually_exclusive(a_validation, b_validation)
                    } else {
                        todo!()
                    }
                }

                // For two objects we need to check required properties and
                // additional properties to see if there exists an object that
                // could successfully be validated by either schema.
                (Some(SingleOrVec::Single(a_single)), Some(SingleOrVec::Single(b_single)))
                    if a_single == b_single && a_single.as_ref() == &InstanceType::Array =>
                {
                    if let (
                        SchemaObject {
                            metadata: _,
                            instance_type: _,
                            format: None,
                            enum_values: None,
                            const_value: None,
                            subschemas: None,
                            number: None,
                            string: None,
                            array: Some(a_validation),
                            object: None,
                            reference: None,
                            extensions: _,
                        },
                        SchemaObject {
                            metadata: _,
                            instance_type: _,
                            format: None,
                            enum_values: None,
                            const_value: None,
                            subschemas: None,
                            number: None,
                            string: None,
                            array: Some(b_validation),
                            object: None,
                            reference: None,
                            extensions: _,
                        },
                    ) = (a, b)
                    {
                        array_schemas_mutually_exclusive(a_validation, b_validation)
                    } else {
                        todo!()
                    }
                }

                // For other simple types, check if the single type is the same
                // or not.
                (Some(SingleOrVec::Single(a_single)), Some(SingleOrVec::Single(b_single))) => {
                    a_single != b_single
                }

                // For two schemas with lists of instance types, make sure that
                // all pairs differ.
                (Some(SingleOrVec::Vec(a_vec)), Some(SingleOrVec::Vec(b_vec))) => a_vec
                    .iter()
                    .all(|instance_type| !b_vec.contains(instance_type)),

                // If one is a single type and the other is a vec, it will
                // suffice for now to check that the single item is different
                // than all the items in the vec.
                (Some(SingleOrVec::Single(single)), Some(SingleOrVec::Vec(vec)))
                | (Some(SingleOrVec::Vec(vec)), Some(SingleOrVec::Single(single))) => {
                    !vec.contains(single)
                }
            }
        }
    }
}

fn object_schemas_mutually_exclusive(
    a_validation: &ObjectValidation,
    b_validation: &ObjectValidation,
) -> bool {
    match (a_validation, b_validation) {
        (
            ObjectValidation {
                required: a_required,
                additional_properties: Some(a_additional),
                ..
            },
            ObjectValidation {
                required: b_required,
                additional_properties: Some(b_additional),
                ..
            },
            // Both objects must disallow additional fields
            // (this is stricter than absolutely necessary,
            // but is a reasonable simplification)
        ) if a_additional.as_ref() == &Schema::Bool(false)
            && b_additional.as_ref() == &Schema::Bool(false) =>
        {
            // TODO seems odd that this matches

            // Neither set of required properties must be a
            // subset of the other i.e. both must have
            // unique, required properties.
            !a_required.is_subset(b_required) && !b_required.is_subset(a_required)
        }
        (
            ObjectValidation {
                properties: a_properties,
                required: a_required,
                // TODO I believe this should actually be
                // Bool(false) as above
                additional_properties: None,
                ..
            },
            ObjectValidation {
                properties: b_properties,
                required: b_required,
                additional_properties: None,
                ..
            },
        ) => {
            // The object schemas are mutually exclusive if
            // each has one or more unique, required
            // properties.
            if !a_required.is_subset(b_required) && !b_required.is_subset(a_required) {
                true
            } else {
                // Even if one of the sets of required
                // properties is a subset of the other (or
                // if they are identical), each may have
                // properties that have fixed values that
                // differ. This can happen in particular
                // for internally or adjacently tagged
                // enums where the properties may be
                // identical but the value of the tag
                // property will be unique.

                // Compute the set that consists of
                // fixed-value properties--a tuple of
                // the property name and the value.
                let aa = a_required
                    .iter()
                    .filter_map(|name| {
                        let t = a_properties.get(name).unwrap();
                        constant_string_value(t).map(|s| (name.clone(), s))
                    })
                    .collect::<BTreeSet<_>>();
                let bb = b_required
                    .iter()
                    .filter_map(|name| {
                        let t = b_properties.get(name).unwrap();
                        constant_string_value(t).map(|s| (name.clone(), s))
                    })
                    .collect::<BTreeSet<_>>();

                // True if neither is a subset of the other.
                !aa.is_subset(&bb) && !bb.is_subset(&aa)
            }
        }

        _ => todo!(),
    }
}

fn array_schemas_mutually_exclusive(
    a_validation: &ArrayValidation,
    b_validation: &ArrayValidation,
) -> bool {
    match (a_validation, b_validation) {
        // If one is an array with a single item type and the other is a tuple
        // of a fixed size with fixed item types, we could only see a conflict
        // if the single item was compatible with *all* types of the tuple.
        // It's therefore sufficient to see if it's exclusive with *any* of the
        // types of the tuple.
        (
            ArrayValidation {
                items: Some(SingleOrVec::Single(single)),
                additional_items: None,
                ..
            },
            ArrayValidation {
                items: Some(SingleOrVec::Vec(vec)),
                additional_items: None,
                max_items: Some(max_items),
                min_items: Some(min_items),
                unique_items: None,
                contains: None,
            },
        )
        | (
            ArrayValidation {
                items: Some(SingleOrVec::Vec(vec)),
                additional_items: None,
                max_items: Some(max_items),
                min_items: Some(min_items),
                unique_items: None,
                contains: None,
            },
            ArrayValidation {
                items: Some(SingleOrVec::Single(single)),
                additional_items: None,
                ..
            },
        ) if max_items == min_items && *max_items as usize == vec.len() => vec
            .iter()
            .any(|schema| schemas_mutually_exclusive(schema, single)),

        // Tuples with different numbers of elements are incompatible.
        (
            ArrayValidation {
                items: Some(SingleOrVec::Vec(a_items)),
                additional_items: None,
                max_items: Some(a_max_items),
                min_items: Some(a_min_items),
                unique_items: None,
                contains: None,
            },
            ArrayValidation {
                items: Some(SingleOrVec::Vec(b_items)),
                additional_items: None,
                max_items: Some(b_max_items),
                min_items: Some(b_min_items),
                unique_items: None,
                contains: None,
            },
        ) if a_max_items == a_min_items
            && *a_max_items as usize == a_items.len()
            && b_max_items == b_min_items
            && *b_max_items as usize == b_items.len() =>
        {
            a_max_items != b_max_items
        }

        (aa, bb) => todo!("{:#?} {:#?}", aa, bb),
    }
}

/// If this schema represents a constant-value string, return that string,
/// otherwise return None.
pub(crate) fn constant_string_value(schema: &Schema) -> Option<String> {
    match schema {
        // Strings must be simple enumerations.
        Schema::Object(SchemaObject {
            metadata: _,
            instance_type: Some(SingleOrVec::Single(single)),
            format: None,
            enum_values: Some(values),
            const_value: None,
            subschemas: None,
            number: None,
            string: None,
            array: None,
            object: None,
            reference: None,
            extensions: _,
        }) if single.as_ref() == &InstanceType::String => {
            if values.len() == 1 {
                values
                    .get(0)
                    .and_then(|value| value.as_str().map(ToString::to_string))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn resolve<'a>(schema: &'a Schema, definitions: &'a schemars::Map<String, Schema>) -> &'a Schema {
    match schema {
        Schema::Bool(_) => schema,
        Schema::Object(SchemaObject {
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
            reference: Some(reference),
            extensions: _,
        }) => {
            const PREFIX: &str = "#/definitions/";
            assert!(reference.starts_with(PREFIX));
            let type_name = &reference[PREFIX.len()..];

            definitions.get(type_name).unwrap()
        }
        Schema::Object(SchemaObject {
            reference: None, ..
        }) => schema,
        // TODO Not sure what this would mean...
        _ => todo!(),
    }
}

fn sanitize(input: String) -> String {
    let out = input.replace("$", "-").replace("'", "");
    match out.as_str() {
        "ref" => "rref".to_string(),
        "type" => "ttype".to_string(),
        "self" => "sself".to_string(),
        _ => out,
    }
}

pub(crate) fn recase(input: String, case: Case) -> (String, Option<String>) {
    let new = sanitize(input.clone()).to_case(case);
    let rename = if new == input { None } else { Some(input) };
    assert_ne!(new, "ref");
    (new, rename)
}

pub(crate) fn get_type_name(
    type_name: &Name,
    metadata: &Option<Box<Metadata>>,
    case: Case,
) -> Option<String> {
    let name = match (type_name, metadata_title(metadata)) {
        (Name::Required(name), _) => name.clone(),
        (Name::Suggested(name), None) => name.clone(),
        (_, Some(name)) => name,
        (Name::Unknown, None) => None?,
    };

    let name = sanitize(name);
    Some(name.to_case(case))
}
