// Copyright 2022 Oxide Computer Company

use std::collections::{BTreeSet, HashSet};

use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
    SubschemaValidation,
};
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::{Name, TypeSpace};

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

pub(crate) fn metadata_title_and_description(metadata: &Option<Box<Metadata>>) -> Option<String> {
    metadata
        .as_ref()
        .and_then(|metadata| match (&metadata.title, &metadata.description) {
            (Some(t), Some(d)) => Some(format!("{}\n\n{}", t, d)),
            (Some(t), None) => Some(t.clone()),
            (None, Some(d)) => Some(d.clone()),
            (None, None) => None,
        })
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

        // Iterate over subschemas.
        (
            other,
            Schema::Object(SchemaObject {
                metadata: None,
                instance_type: None,
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: Some(subschemas),
                number: None,
                string: None,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            }),
        )
        | (
            Schema::Object(SchemaObject {
                metadata: None,
                instance_type: None,
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: Some(subschemas),
                number: None,
                string: None,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            }),
            other,
        ) => match subschemas.as_ref() {
            // For an allOf, *any* subschema incompatibility means that the
            // schemas are mutually exclusive.
            SubschemaValidation {
                all_of: Some(s),
                any_of: None,
                one_of: None,
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            } => s.iter().any(|sub| schemas_mutually_exclusive(sub, other)),

            // For a oneOf or anyOf, *all* subschemas need to be incompatible.
            SubschemaValidation {
                all_of: None,
                any_of: Some(s),
                one_of: None,
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            }
            | SubschemaValidation {
                all_of: None,
                any_of: None,
                one_of: Some(s),
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            } => s.iter().all(|sub| schemas_mutually_exclusive(sub, b)),

            // For a not, they're mutually exclusive if they *do* match.
            SubschemaValidation {
                all_of: None,
                any_of: None,
                one_of: None,
                not: Some(sub),
                if_schema: None,
                then_schema: None,
                else_schema: None,
            } => !schemas_mutually_exclusive(sub, other),

            // Assume other subschemas are complex to understand and may
            // therefore be compatible.
            _ => false,
        },

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

// See if there are unique, required properties of each that cannot be present
// in the other. In other words, see if there are properties that would
// uniquely identify an objects as validating exclusively with one or the other
// (but not with both).
fn object_schemas_mutually_exclusive(
    a_validation: &ObjectValidation,
    b_validation: &ObjectValidation,
) -> bool {
    let ObjectValidation {
        required: a_required,
        properties: a_properties,
        ..
    } = a_validation;
    let ObjectValidation {
        required: b_required,
        properties: b_properties,
        ..
    } = b_validation;

    // Either set of required properties must not be a subset of the other's
    // properties i.e. if there's a property that *must* be in one of the two
    // objects, and *cannot* be in the other, a property whose presence or
    // absence determines which of the two objects is relevant.
    if !a_required.is_subset(&b_properties.keys().cloned().collect())
        || !b_required.is_subset(&a_properties.keys().cloned().collect())
    {
        true
    } else {
        // Even if all required properties of each is a permitted property of
        // the other, each may have required properties that have fixed values
        // that differ. This can happen in particular for internally or
        // adjacently tagged enums where the properties may be identical but
        // the value of the tag property will be unique.

        // Compute the set that consists of fixed-value properties--a
        // tuple of the property name and the value.
        let aa = a_required
            .iter()
            .filter_map(|name| {
                let t = a_properties.get(name).unwrap();
                constant_string_value(t).map(|s| (name.clone(), s))
            })
            .collect::<HashSet<_>>();
        let bb = b_required
            .iter()
            .filter_map(|name| {
                let t = b_properties.get(name).unwrap();
                constant_string_value(t).map(|s| (name.clone(), s))
            })
            .collect::<HashSet<_>>();

        // True if neither is a subset of the other.
        !aa.is_subset(&bb) && !bb.is_subset(&aa)
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

        // Plain, vanilla arrays.
        (
            ArrayValidation {
                items: Some(SingleOrVec::Single(a_items)),
                additional_items: None,
                max_items: None,
                min_items: None,
                unique_items: None,
                contains: None,
            },
            ArrayValidation {
                items: Some(SingleOrVec::Single(b_items)),
                additional_items: None,
                max_items: None,
                min_items: None,
                unique_items: None,
                contains: None,
            },
        ) => schemas_mutually_exclusive(a_items, b_items),

        (aa, bb) => todo!("{:#?} {:#?}", aa, bb),
    }
}

/// If this schema represents a constant-value string, return that string,
/// otherwise return None.
pub(crate) fn constant_string_value(schema: &Schema) -> Option<&str> {
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
                values.get(0).and_then(|value| value.as_str())
            } else {
                None
            }
        }
        _ => None,
    }
}

pub(crate) fn ref_key(ref_name: &String) -> &str {
    match ref_name.rfind('/') {
        Some(idx) => &ref_name[idx + 1..],
        None => ref_name,
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
            reference: Some(ref_name),
            extensions: _,
        }) => definitions.get(ref_key(ref_name)).unwrap(),
        Schema::Object(SchemaObject {
            reference: None, ..
        }) => schema,
        // TODO Not sure what this would mean...
        _ => todo!(),
    }
}

pub(crate) fn schema_is_named(schema: &Schema) -> Option<String> {
    let raw_name = match schema {
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
            let idx = reference.rfind('/')?;
            Some(reference[idx + 1..].to_string())
        }
        Schema::Object(SchemaObject {
            metadata: Some(metadata),
            ..
        }) if metadata.as_ref().title.is_some() => Some(metadata.as_ref().title.as_ref()?.clone()),
        Schema::Object(SchemaObject {
            metadata: _,
            instance_type: _,
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: Some(subschemas),
            number: None,
            string: None,
            array: None,
            object: None,
            reference: None,
            extensions: _,
        }) => match subschemas.as_ref() {
            SubschemaValidation {
                all_of: Some(subschemas),
                any_of: None,
                one_of: None,
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            }
            | SubschemaValidation {
                all_of: None,
                any_of: Some(subschemas),
                one_of: None,
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            }
            | SubschemaValidation {
                all_of: None,
                any_of: None,
                one_of: Some(subschemas),
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            } if subschemas.len() == 1 => schema_is_named(subschemas.first()?),
            _ => None,
        },
        _ => None,
    }?;

    Some(sanitize(&raw_name, Case::Pascal))
}

pub(crate) enum Case {
    Pascal,
    Snake,
}

pub(crate) fn sanitize(input: &str, case: Case) -> String {
    use heck::{ToPascalCase, ToSnakeCase};
    let to_case = match case {
        Case::Pascal => str::to_pascal_case,
        Case::Snake => str::to_snake_case,
    };

    // If every case was special then none of them would be.
    let out = match input {
        "+1" => "plus1".to_string(),
        "-1" => "minus1".to_string(),
        _ => to_case(&input.replace("'", "").replace(|c| !is_xid_continue(c), "-")),
    };

    let out = match out.chars().next() {
        None => to_case("x"),
        Some(c) if is_xid_start(c) => out,
        Some(_) => format!("_{}", out),
    };

    // Make sure the string is a valid Rust identifier.
    if syn::parse_str::<syn::Ident>(&out).is_ok() {
        out
    } else {
        format!("{}_", out)
    }
}

pub(crate) fn recase(input: &str, case: Case) -> (String, Option<String>) {
    let new = sanitize(input, case);
    let rename = if new == input {
        None
    } else {
        Some(input.to_string())
    };
    (new, rename)
}

pub(crate) fn get_type_name(type_name: &Name, metadata: &Option<Box<Metadata>>) -> Option<String> {
    let name = match (type_name, metadata_title(metadata)) {
        (Name::Required(name), _) => name.clone(),
        (Name::Suggested(name), None) => name.clone(),
        (_, Some(name)) => name,
        (Name::Unknown, None) => None?,
    };

    Some(sanitize(&name, Case::Pascal))
}

pub(crate) fn type_patch(type_space: &TypeSpace, type_name: String) -> (String, BTreeSet<String>) {
    match type_space.settings.patch.get(&type_name) {
        None => (type_name, Default::default()),

        Some(patch) => {
            let name = patch.rename.clone().unwrap_or(type_name);
            let derives = patch.derives.iter().cloned().collect();

            (name, derives)
        }
    }
}

pub(crate) fn none_or_single<T>(test: &Option<SingleOrVec<T>>, value: &T) -> bool
where
    T: Eq,
{
    match test {
        None => true,
        Some(SingleOrVec::Single(single)) if single.as_ref() == value => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use schemars::{schema_for, JsonSchema};

    use crate::util::{sanitize, schemas_mutually_exclusive, Case};

    #[test]
    fn test_non_exclusive_structs() {
        #![allow(dead_code)]

        #[derive(JsonSchema)]
        struct A {
            a: Option<()>,
            b: (),
        }

        #[derive(JsonSchema)]
        struct B {
            a: (),
            b: Option<()>,
        }

        let a = schema_for!(A).schema.into();
        let b = schema_for!(B).schema.into();

        assert!(!schemas_mutually_exclusive(&a, &b));
        assert!(!schemas_mutually_exclusive(&b, &a));
    }

    #[test]
    fn test_unique_prop_structs() {
        #![allow(dead_code)]

        #[derive(JsonSchema)]
        struct A {
            a: Option<()>,
            b: (),
        }

        #[derive(JsonSchema)]
        struct B {
            a: (),
            b: Option<()>,
            c: (),
        }

        let a = schema_for!(A).schema.into();
        let b = schema_for!(B).schema.into();

        assert!(schemas_mutually_exclusive(&a, &b));
        assert!(schemas_mutually_exclusive(&b, &a));
    }

    #[test]
    fn test_exclusive_structs() {
        #![allow(dead_code)]

        #[derive(JsonSchema)]
        struct A {
            a: Option<()>,
            b: (),
            aa: (),
        }

        #[derive(JsonSchema)]
        struct B {
            a: (),
            b: Option<()>,
            bb: (),
        }

        let a = schema_for!(A).schema.into();
        let b = schema_for!(B).schema.into();

        assert!(schemas_mutually_exclusive(&a, &b));
        assert!(schemas_mutually_exclusive(&b, &a));
    }

    #[test]
    fn test_exclusive_one_empty_struct() {
        #![allow(dead_code)]

        #[derive(JsonSchema)]
        struct A {}

        #[derive(JsonSchema)]
        struct B {
            a: (),
            b: Option<()>,
            bb: (),
        }

        let a = schema_for!(A).schema.into();
        let b = schema_for!(B).schema.into();

        assert!(schemas_mutually_exclusive(&a, &b));
        assert!(schemas_mutually_exclusive(&b, &a));
    }

    #[test]
    fn test_exclusive_simple_arrays() {
        let a = schema_for!(Vec<u32>).schema.into();
        let b = schema_for!(Vec<f32>).schema.into();

        assert!(schemas_mutually_exclusive(&a, &b));
        assert!(schemas_mutually_exclusive(&b, &a));
    }

    #[test]
    fn test_sanitize() {
        assert_eq!(sanitize("type", Case::Snake), "type_");
        assert_eq!(sanitize("ref", Case::Snake), "ref_");
        assert_eq!(sanitize("+1", Case::Snake), "plus1");
        assert_eq!(sanitize("-1", Case::Snake), "minus1");
        assert_eq!(sanitize("@timestamp", Case::Pascal), "Timestamp");
        assert_eq!(sanitize("won't and can't", Case::Pascal), "WontAndCant");
        assert_eq!(
            sanitize(
                "urn:ietf:params:scim:schemas:extension:gluu:2.0:user_",
                Case::Pascal
            ),
            "UrnIetfParamsScimSchemasExtensionGluu20User"
        );
        assert_eq!(sanitize("Ipv6Net", Case::Snake), "ipv6_net");
        assert_eq!(sanitize("V6", Case::Pascal), "V6");
    }
}
