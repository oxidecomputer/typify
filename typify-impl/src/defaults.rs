// Copyright 2026 Oxide Computer Company

use std::collections::BTreeMap;

use crate::{
    convert::STD_NUM_NONZERO_PREFIX, type_entry::TypeEntry, Error, Result, TypeId, TypeSpace,
};

use typespace::build::{
    EnumTagType, EnumVariant, StructProperty, StructPropertySerde, StructPropertyState, Type,
    VariantDetails,
};

impl TypeEntry {
    /// Check that the given [`Value`] is a valid instance of this type
    ///
    /// The return value indicates whether the default is the "intrinsic",
    /// typical default for the given type, can be handled by generic function,
    /// or requires a bespoke function to generate the value.
    ///
    /// [`Value`]: serde_json::Value
    pub(crate) fn validate_value(
        &self,
        type_space: &TypeSpace,
        default: &serde_json::Value,
    ) -> Result<()> {
        match self.as_type() {
            Type::Enum(type_enum) => {
                let variants = type_enum.get_variants();
                match type_enum.get_tag_type().expect("built enum has a tag type") {
                    EnumTagType::External => {
                        validate_default_for_external_enum(type_space, variants, default)
                            .ok_or_else(Error::invalid_value)
                    }
                    EnumTagType::Internal { tag } => {
                        validate_default_for_internal_enum(type_space, variants, default, tag)
                            .ok_or_else(Error::invalid_value)
                    }
                    EnumTagType::Adjacent { tag, content } => validate_default_for_adjacent_enum(
                        type_space, variants, default, tag, content,
                    )
                    .ok_or_else(Error::invalid_value),
                    EnumTagType::Untagged => {
                        validate_default_for_untagged_enum(type_space, variants, default)
                            .ok_or_else(Error::invalid_value)
                    }
                }
            }
            Type::Struct(type_struct) => {
                validate_default_struct_props(type_struct.get_properties(), type_space, default)
                    .ok_or_else(Error::invalid_value)
            }

            Type::NewtypeStruct(type_newtype) => {
                let inner = type_newtype.get_inner();
                // Validate the inner type, but irrespective of the result,
                // we'll need a custom function to make a default of the outer
                // newtype.
                let _ = validate_type_id(inner, type_space, default)?;
                Ok(())
            }
            Type::Option(type_id) => {
                if let serde_json::Value::Null = default {
                    Ok(())
                } else {
                    // Make sure the default is valid for the sub-type.
                    let _ = validate_type_id(type_id, type_space, default)?;
                    Ok(())
                }
            }
            Type::Box(type_id) => validate_type_id(type_id, type_space, default),

            Type::Vec(type_id) => {
                if let serde_json::Value::Array(v) = default {
                    if v.is_empty() {
                        Ok(())
                    } else {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        for value in v {
                            let _ = type_entry.validate_value(type_space, value)?;
                        }
                        Ok(())
                    }
                } else {
                    Err(Error::invalid_value())
                }
            }
            Type::Map(key_id, value_id) => {
                if let serde_json::Value::Object(m) = default {
                    if m.is_empty() {
                        Ok(())
                    } else {
                        let key_ty = type_space.id_to_entry.get(key_id).unwrap();
                        let value_ty = type_space.id_to_entry.get(value_id).unwrap();
                        for (key, value) in m {
                            let _ = key_ty.validate_value(
                                type_space,
                                &serde_json::Value::String(key.clone()),
                            )?;
                            let _ = value_ty.validate_value(type_space, value)?;
                        }
                        Ok(())
                    }
                } else {
                    Err(Error::invalid_value())
                }
            }
            Type::Set(type_id) => {
                if let serde_json::Value::Array(v) = default {
                    if v.is_empty() {
                        Ok(())
                    } else {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        for (i, value) in v.iter().enumerate() {
                            // Sets can't contain duplicates; also Value isn't
                            // Ord so O(n^2) it is!
                            for other in &v[(i + 1)..] {
                                if value == other {
                                    return Err(Error::invalid_value());
                                }
                            }
                            let _ = type_entry.validate_value(type_space, value)?;
                        }
                        Ok(())
                    }
                } else {
                    Err(Error::invalid_value())
                }
            }
            Type::Tuple(ids) => {
                validate_default_tuple(ids, type_space, default).ok_or_else(Error::invalid_value)
            }

            Type::Array(type_id, length) => {
                let Some(arr) = default.as_array() else {
                    return Err(Error::invalid_value());
                };
                if arr.len() != *length {
                    return Err(Error::invalid_value());
                }

                let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                for value in arr {
                    let _ = type_entry.validate_value(type_space, value)?;
                }
                Ok(())
            }
            Type::Unit => {
                if let serde_json::Value::Null = default {
                    Ok(())
                } else {
                    Err(Error::invalid_value())
                }
            }
            Type::Native(_) => {
                // This is tricky. There's not a lot we can do--particularly if
                // and when we start to consider arbitrary types as "built-in"
                // (e.g. if schemars tags types with an extension to denote
                // their rust type or if the user can supply a list of type
                // names to treat as built-in). So we just do no checking and
                // will fail an `unwrap()` in the generated code if this Value
                // is not valid for this built-in type.
                Ok(())
            }
            Type::JsonValue => Ok(()),
            Type::Boolean => match default {
                serde_json::Value::Bool(false) => Ok(()),
                serde_json::Value::Bool(true) => Ok(()),
                _ => Err(Error::invalid_value()),
            },
            // Note that min and max values are handled already by the
            // conversion routines since we have those close at hand.
            Type::Integer(itype) => match (default.as_u64(), default.as_i64()) {
                (None, None) => Err(Error::invalid_value()),
                (Some(0), _) => Ok(()),
                (_, Some(0)) => unreachable!(),
                (Some(_), _) => {
                    if itype.starts_with(STD_NUM_NONZERO_PREFIX) {
                        Ok(())
                    } else {
                        Ok(())
                    }
                }
                (_, Some(_)) => Ok(()),
            },
            Type::Float(_) => {
                if let Some(value) = default.as_f64() {
                    if value == 0.0 {
                        Ok(())
                    } else {
                        Ok(())
                    }
                } else {
                    Err(Error::invalid_value())
                }
            }
            Type::String => {
                if let Some("") = default.as_str() {
                    Ok(())
                } else {
                    Ok(())
                }
            }

            // typify never constructs unit structs, tuple structs, or
            // type aliases; Type is non_exhaustive besides.
            _ => unreachable!("unexpected typespace type variant"),
        }
    }
}

pub(crate) fn validate_default_for_external_enum(
    type_space: &TypeSpace,
    variants: &[EnumVariant<TypeId>],
    default: &serde_json::Value,
) -> Option<()> {
    if let Some(simple_name) = default.as_str() {
        let variant = variants
            .iter()
            .find(|variant| simple_name == variant.json_name())?;
        matches!(variant.details(), VariantDetails::Unit).then(|| ())?;

        Some(())
    } else {
        let map = default.as_object()?;
        if map.len() != 1 {
            return None;
        }

        let (name, value) = map.iter().next()?;

        let variant = variants
            .iter()
            .find(|variant| name == variant.json_name())?;

        match variant.details() {
            VariantDetails::Unit => None,
            VariantDetails::Item(type_id) => validate_type_id(type_id, type_space, value).ok(),
            VariantDetails::Tuple(tup) => validate_default_tuple(tup, type_space, value),
            VariantDetails::Struct(props) => {
                validate_default_struct_props(props, type_space, value)
            }
        }
    }
}

pub(crate) fn validate_default_for_internal_enum(
    type_space: &TypeSpace,
    variants: &[EnumVariant<TypeId>],
    default: &serde_json::Value,
    tag: &str,
) -> Option<()> {
    let map = default.as_object()?;
    let name = map.get(tag).and_then(serde_json::Value::as_str)?;
    let variant = variants
        .iter()
        .find(|variant| name == variant.json_name())?;

    match variant.details() {
        VariantDetails::Unit => Some(()),
        VariantDetails::Struct(props) => {
            // Make an object without the tag.
            let inner_default = serde_json::Value::Object(
                map.clone()
                    .into_iter()
                    .filter(|(name, _)| name != tag)
                    .collect(),
            );

            validate_default_struct_props(props, type_space, &inner_default)
        }

        VariantDetails::Item(_) | VariantDetails::Tuple(_) => unreachable!(),
    }
}

pub(crate) fn validate_default_for_adjacent_enum(
    type_space: &TypeSpace,
    variants: &[EnumVariant<TypeId>],
    default: &serde_json::Value,
    tag: &str,
    content: &str,
) -> Option<()> {
    let map = default.as_object()?;

    let (tag_value, content_value) = match (
        map.len(),
        map.get(tag).and_then(serde_json::Value::as_str),
        map.get(content),
    ) {
        (1, Some(tag_value), None) => (tag_value, None),
        (2, Some(tag_value), content_value @ Some(_)) => (tag_value, content_value),
        _ => return None,
    };

    let variant = variants
        .iter()
        .find(|variant| tag_value == variant.json_name())?;

    match (variant.details(), content_value) {
        (VariantDetails::Unit, None) => Some(()),
        (VariantDetails::Tuple(tup), Some(content_value)) => {
            validate_default_tuple(tup, type_space, content_value)
        }
        (VariantDetails::Struct(props), Some(content_value)) => {
            validate_default_struct_props(props, type_space, content_value)
        }
        _ => None,
    }
}

pub(crate) fn validate_default_for_untagged_enum(
    type_space: &TypeSpace,
    variants: &[EnumVariant<TypeId>],
    default: &serde_json::Value,
) -> Option<()> {
    variants.iter().find_map(|variant| {
        // The name of the variant is not meaningful; we just need to see
        // if any of the variants are valid with the given default.
        match variant.details() {
            VariantDetails::Unit => {
                default.as_null()?;
                Some(())
            }
            VariantDetails::Item(type_id) => validate_type_id(type_id, type_space, default).ok(),
            VariantDetails::Tuple(tup) => validate_default_tuple(tup, type_space, default),
            VariantDetails::Struct(props) => {
                validate_default_struct_props(props, type_space, default)
            }
        }
    })
}

fn validate_type_id(
    type_id: &TypeId,
    type_space: &TypeSpace,
    default: &serde_json::Value,
) -> Result<()> {
    let type_entry = type_space.id_to_entry.get(type_id).unwrap();
    type_entry.validate_value(type_space, default)
}

fn validate_default_tuple(
    types: &[TypeId],
    type_space: &TypeSpace,
    default: &serde_json::Value,
) -> Option<()> {
    let arr = default.as_array()?;
    if arr.len() != types.len() {
        return None;
    }

    types
        .iter()
        .zip(arr.iter())
        .all(|(type_id, value)| validate_type_id(type_id, type_space, value).is_ok())
        .then_some(())
}

fn validate_default_struct_props(
    properties: &[StructProperty<TypeId>],
    type_space: &TypeSpace,
    default: &serde_json::Value,
) -> Option<()> {
    let map = default.as_object()?;

    // Gather up all properties including those of flattened struct properties:
    // a tuple of (name: Option<String>, type_id: TypeId, required: bool). We
    // partition these into the named_properties which we then put into a map
    // with the property name as the key, and unnamed_properties which consists
    // of properties from flattened maps which have types but not names.
    let (named_properties, unnamed_properties): (Vec<_>, Vec<_>) = properties
        .iter()
        .flat_map(|property| all_props(property, type_space))
        .partition(|(name, _, _)| name.is_some());

    // These are the direct properties of this struct as well as the properties
    // of any nested, flatted struct.
    let named_properties = named_properties
        .into_iter()
        .map(|(name, type_id, required)| (name.unwrap(), (type_id, required)))
        .collect::<BTreeMap<_, _>>();
    // These are just the types for any flattened map (either within this
    // struct or nested within another flattened struct).
    let unnamed_properties = unnamed_properties
        .into_iter()
        .map(|(_, type_id, _)| type_id)
        .collect::<Vec<_>>();

    // Make sure that every value in the map validates properly.
    map.iter().try_for_each(|(name, default_value)| {
        // If there's a matching, named property, the value needs to validate.
        // Otherwise it needs to validate against the schema of one of the
        // unnamed properties i.e. it must be a valid value type for a nested,
        // flatted map.
        if let Some((type_id, _)) = named_properties.get(name) {
            validate_type_id(type_id, type_space, default_value)
                .ok()
                .map(|_| ())
        } else {
            unnamed_properties
                .iter()
                .any(|type_id| validate_type_id(type_id, type_space, default_value).is_ok())
                .then_some(())
        }
    })?;

    // Make sure that every required field is present in the map.
    named_properties
        .iter()
        .filter(|(_, (_, required))| *required)
        .try_for_each(|(name, _)| map.get(name.as_str()).map(|_| ()))?;

    Some(())
}

fn all_props<'a>(
    property: &'a StructProperty<TypeId>,
    type_space: &'a TypeSpace,
) -> Vec<(Option<String>, &'a TypeId, bool)> {
    // The JSON name of the property; typespace stores the Rust name and
    // an optional serde rename, so the JSON name is the rename when
    // present and the Rust name otherwise.
    let maybe_name = match property.json_name() {
        StructPropertySerde::None => Some(property.rust_name().to_string()),
        StructPropertySerde::Rename(rename) => Some(rename.clone()),
        StructPropertySerde::Flatten => None,
    };

    if let Some(name) = maybe_name {
        let required = matches!(property.state(), StructPropertyState::Required);

        vec![(Some(name), property.type_id(), required)]
    } else {
        // The type must be a struct, an option for a struct, or a map.
        let type_entry = type_space.id_to_entry.get(property.type_id()).unwrap();

        let (properties, all_required) = match type_entry.as_type() {
            Type::Struct(type_struct) => {
                let optional = matches!(property.state(), StructPropertyState::Optional);
                (type_struct.get_properties(), !optional)
            }
            Type::Option(type_id) => {
                let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                if let Type::Struct(type_struct) = type_entry.as_type() {
                    (type_struct.get_properties(), false)
                } else {
                    unreachable!()
                }
            }

            // TODO Rather than an option, this should probably be something
            // that lets us say "explicit name" or "type to validate against"
            Type::Map(_, value_id) => return vec![(None, value_id, false)],
            _ => unreachable!(),
        };

        properties
            .iter()
            .flat_map(|property| all_props(property, type_space))
            .map(|(name, type_id, required)| (name, type_id, required && all_required))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use schemars::JsonSchema;
    use serde_json::json;
    use uuid::Uuid;

    use crate::{test_util::get_type, type_entry::TypeEntry};

    #[test]
    fn test_default_option() {
        let (type_space, type_id) = get_type::<Option<u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(type_entry
            .validate_value(&type_space, &json!("forty-two"))
            .is_err());
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(null)),
            Ok(())
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(42)),
            Ok(())
        ));
    }

    #[test]
    fn test_default_box() {
        let (type_space, type_id) = get_type::<Option<u32>>();

        let type_entry = TypeEntry::from(typespace::build::Type::Box(type_id));

        assert!(type_entry
            .validate_value(&type_space, &json!("forty-two"))
            .is_err());
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(null)),
            Ok(())
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(42)),
            Ok(())
        ));
    }

    #[test]
    fn test_default_array() {
        let (type_space, type_id) = get_type::<Vec<u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(type_entry
            .validate_value(&type_space, &json!([null]))
            .is_err());
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([])),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([1, 2, 5])),
            Ok(()),
        ));
    }

    #[test]
    fn test_default_map() {
        let (type_space, type_id) = get_type::<HashMap<String, u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(type_entry.validate_value(&type_space, &json!([])).is_err());
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!({})),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!({"a": 1, "b": 2})),
            Ok(()),
        ));
    }

    #[test]
    fn test_default_tuple() {
        let (type_space, type_id) = get_type::<(u32, u32, String)>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(type_entry
            .validate_value(&type_space, &json!([1, 2, "three", 4]))
            .is_err());
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([1, 2, "three"])),
            Ok(()),
        ));
    }

    #[test]
    fn test_default_builtin() {
        let (type_space, type_id) = get_type::<Uuid>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("not-a-uuid")),
            Ok(())
        ));
    }

    #[test]
    fn test_default_bool() {
        let (type_space, type_id) = get_type::<bool>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(false)),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(true)),
            Ok(()),
        ));
    }

    #[test]
    fn test_default_numbers_and_string() {
        let (type_space, type_id) = get_type::<u32>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(type_entry
            .validate_value(&type_space, &json!(true))
            .is_err());
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(0)),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(42)),
            Ok(()),
        ));

        let (type_space, type_id) = get_type::<String>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("")),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("howdy")),
            Ok(()),
        ));
    }

    #[test]
    fn test_struct_simple() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        struct Test {
            a: String,
            b: u32,
            c: Option<String>,
            d: Option<f64>,
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!(
                    {
                        "a": "aaaa",
                        "b": 7,
                        "c": "cccc"
                    }
                )
            ),
            Ok(()),
        ));
        assert!(type_entry
            .validate_value(
                &type_space,
                &json!(
                    {
                        "a": "aaaa",
                        "c": "cccc",
                        "d": 7
                    }
                )
            )
            .is_err());
        assert!(type_entry
            .validate_value(
                &type_space,
                &json!(
                    {
                        "a": "aaaa",
                        "b": 7,
                        "d": {}
                    }
                )
            )
            .is_err());
    }

    #[test]
    fn test_enum_external() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("A")),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "B": ["xx", "yy"]
                })
            ),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "C": { "cc": "xx", "dd": "yy" }
                })
            ),
            Ok(()),
        ));
        assert!(type_entry
            .validate_value(&type_space, &json!({ "A": null }))
            .is_err());
        assert!(type_entry.validate_value(&type_space, &json!("B")).is_err());
    }

    #[test]
    fn test_enum_internal() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(tag = "tag")]
        enum Test {
            A,
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "A"
                })
            ),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "C",
                    "cc": "xx",
                    "dd": "yy"
                })
            ),
            Ok(()),
        ));
        assert!(type_entry
            .validate_value(
                &type_space,
                &json!({
                    "not-tag": "A"
                })
            )
            .is_err());
        assert!(type_entry
            .validate_value(
                &type_space,
                &json!({
                    "tag": "B",
                    "cc": "where's D?"
                })
            )
            .is_err());
    }

    #[test]
    fn test_enum_adjacent() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(tag = "tag", content = "content")]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "A"
                })
            ),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "B",
                    "content": ["xx", "yy"]
                })
            ),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "C",
                    "content": { "cc": "xx", "dd": "yy" }
                })
            ),
            Ok(()),
        ));
        assert!(type_entry.validate_value(&type_space, &json!("A")).is_err());
        assert!(type_entry
            .validate_value(
                &type_space,
                &json!({
                    "tag": "A",
                    "content": null,
                })
            )
            .is_err());
    }
    #[test]
    fn test_enum_untagged() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(untagged)]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(null)),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(["xx", "yy"])),
            Ok(()),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!( { "cc": "xx", "dd": "yy" })),
            Ok(()),
        ));
        assert!(type_entry.validate_value(&type_space, &json!({})).is_err());
    }
}
