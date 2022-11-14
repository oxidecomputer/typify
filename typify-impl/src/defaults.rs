// Copyright 2022 Oxide Computer Company

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    type_entry::{
        DefaultKind, EnumTagType, StructProperty, StructPropertyRename, StructPropertyState,
        TypeEntry, TypeEntryDetails, TypeEntryEnum, TypeEntryNewtype, TypeEntryStruct, Variant,
        VariantDetails, WrappedValue,
    },
    util::{sanitize, Case},
    DefaultImpl, Error, Result, TypeId, TypeSpace,
};

// Implementations for "stock" default functions so we don't litter the
// namespace with many that are effectively identical.
impl From<&DefaultImpl> for TokenStream {
    fn from(default: &DefaultImpl) -> Self {
        match default {
            DefaultImpl::Boolean => quote! {
                pub(super) fn default_bool<const V: bool>() -> bool {
                    V
                }
            },
            DefaultImpl::I64 => quote! {
                pub(super) fn default_i64<T, const V: i64>() -> T
                where
                    T: std::convert::TryFrom<i64>,
                    <T as std::convert::TryFrom<i64>>::Error: std::fmt::Debug,
                {
                    T::try_from(V).unwrap()
                }
            },
            DefaultImpl::U64 => quote! {
                pub(super) fn default_u64<T, const V: u64>() -> T
                where
                    T: std::convert::TryFrom<u64>,
                    <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
                {
                    T::try_from(V).unwrap()
                }
            },
        }
    }
}

impl TypeEntry {
    pub(crate) fn check_defaults(&self, type_space: &mut TypeSpace) -> Result<()> {
        // Check the "whole-type" default.
        match &self.details {
            TypeEntryDetails::Enum(TypeEntryEnum {
                default: Some(WrappedValue(default)),
                ..
            })
            | TypeEntryDetails::Struct(TypeEntryStruct {
                default: Some(WrappedValue(default)),
                ..
            })
            | TypeEntryDetails::Newtype(TypeEntryNewtype {
                default: Some(WrappedValue(default)),
                ..
            }) => {
                if let DefaultKind::Generic(default_fn) =
                    self.validate_value(type_space, default)?
                {
                    type_space.defaults.insert(default_fn);
                }
            }

            _ => (),
        }

        // Check default values for struct properties or those of struct-type
        // enum variants.
        match &self.details {
            TypeEntryDetails::Struct(TypeEntryStruct { properties, .. }) => {
                properties
                    .iter()
                    .try_for_each(|prop| Self::check_property_defaults(prop, type_space))?;
            }

            TypeEntryDetails::Enum(TypeEntryEnum { variants, .. }) => {
                variants.iter().try_for_each(|variant| {
                    if let VariantDetails::Struct(properties) = &variant.details {
                        properties
                            .iter()
                            .try_for_each(|prop| Self::check_property_defaults(prop, type_space))
                    } else {
                        Ok(())
                    }
                })?;
            }

            _ => (),
        };

        Ok(())
    }

    fn check_property_defaults(
        property: &StructProperty,
        type_space: &mut TypeSpace,
    ) -> Result<()> {
        if let StructProperty {
            state: StructPropertyState::Default(WrappedValue(prop_default)),
            type_id,
            ..
        } = property
        {
            let type_entry = type_space.id_to_entry.get(type_id).unwrap();
            if let DefaultKind::Generic(default_fn) =
                type_entry.validate_value(type_space, prop_default)?
            {
                type_space.defaults.insert(default_fn);
            }
        }
        Ok(())
    }

    /// Check that the given [`Value`] is a valid instance of this type
    ///
    /// The return value indicates whether the default is the "intrinsic",
    /// typical default for the given type, can be handled by generic function,
    /// or requires a bespoke function to generate the value. This contains
    /// additional validation logic compared with [`value()`] but is able to skip the parts where we actually emit code.
    ///
    /// [`Value`]: serde_json::Value
    pub(crate) fn validate_value(
        &self,
        type_space: &TypeSpace,
        default: &serde_json::Value,
    ) -> Result<DefaultKind> {
        match &self.details {
            TypeEntryDetails::Enum(TypeEntryEnum {
                tag_type, variants, ..
            }) => match tag_type {
                EnumTagType::External => {
                    validate_default_for_external_enum(type_space, variants, default)
                }
                EnumTagType::Internal { tag } => {
                    validate_default_for_internal_enum(type_space, variants, default, tag)
                }
                EnumTagType::Adjacent { tag, content } => {
                    validate_default_for_adjacent_enum(type_space, variants, default, tag, content)
                }
                EnumTagType::Untagged => {
                    validate_default_for_untagged_enum(type_space, variants, default)
                }
            }
            .ok_or(Error::InvalidValue),
            TypeEntryDetails::Struct(TypeEntryStruct { properties, .. }) => {
                validate_default_struct_props(properties, type_space, default)
                    .ok_or(Error::InvalidValue)
            }

            TypeEntryDetails::Newtype(TypeEntryNewtype { type_id, .. }) => type_space
                .id_to_entry
                .get(type_id)
                .unwrap()
                .validate_value(type_space, default),
            TypeEntryDetails::Option(type_id) => {
                if let serde_json::Value::Null = default {
                    Ok(DefaultKind::Intrinsic)
                } else {
                    let ty = type_space.id_to_entry.get(type_id).unwrap();
                    // Make sure the default is valid for the sub-type.
                    let _ = ty.validate_value(type_space, default)?;
                    Ok(DefaultKind::Specific)
                }
            }
            TypeEntryDetails::Box(type_id) => type_space
                .id_to_entry
                .get(type_id)
                .unwrap()
                .validate_value(type_space, default),

            TypeEntryDetails::Array(type_id) => {
                if let serde_json::Value::Array(v) = default {
                    if v.is_empty() {
                        Ok(DefaultKind::Intrinsic)
                    } else {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        for value in v {
                            let _ = type_entry.validate_value(type_space, value)?;
                        }
                        Ok(DefaultKind::Specific)
                    }
                } else {
                    Err(Error::InvalidValue)
                }
            }
            TypeEntryDetails::Map(type_id) => {
                if let serde_json::Value::Object(m) = default {
                    if m.is_empty() {
                        Ok(DefaultKind::Intrinsic)
                    } else {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        for (_, value) in m {
                            let _ = type_entry.validate_value(type_space, value)?;
                        }
                        Ok(DefaultKind::Specific)
                    }
                } else {
                    Err(Error::InvalidValue)
                }
            }
            TypeEntryDetails::Set(type_id) => {
                if let serde_json::Value::Array(v) = default {
                    if v.is_empty() {
                        Ok(DefaultKind::Intrinsic)
                    } else {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        for (i, value) in v.iter().enumerate() {
                            // Sets can't contain duplicates; also Value isn't
                            // Ord so O(n^2) it is!
                            for other in &v[(i + 1)..] {
                                if value == other {
                                    return Err(Error::InvalidValue);
                                }
                            }
                            let _ = type_entry.validate_value(type_space, value)?;
                        }
                        Ok(DefaultKind::Specific)
                    }
                } else {
                    Err(Error::InvalidValue)
                }
            }
            TypeEntryDetails::Tuple(ids) => {
                validate_default_tuple(ids, type_space, default).ok_or(Error::InvalidValue)
            }
            TypeEntryDetails::Unit => {
                if let serde_json::Value::Null = default {
                    Ok(DefaultKind::Intrinsic)
                } else {
                    Err(Error::InvalidValue)
                }
            }
            TypeEntryDetails::BuiltIn(_) => {
                // This is tricky. There's not a lot we can do--particularly if
                // and when we start to consider arbitrary types as "built-in"
                // (e.g. if schemars tags types with an extension to denote
                // their rust type or if the user can supply a list of type
                // names to treat as built-in). So we just do no checking and
                // will fail an `unwrap()` in the code emitted by `value()` if
                // this Value is not valid for this built-in type.
                Ok(DefaultKind::Specific)
            }
            TypeEntryDetails::Boolean => match default {
                serde_json::Value::Bool(false) => Ok(DefaultKind::Intrinsic),
                serde_json::Value::Bool(true) => Ok(DefaultKind::Generic(DefaultImpl::Boolean)),
                _ => Err(Error::InvalidValue),
            },
            // Note that min and max values are handled already by the
            // conversion routines since we have those close at hand.
            TypeEntryDetails::Integer(_) => match (default.as_u64(), default.as_i64()) {
                (None, None) => Err(Error::InvalidValue),
                (Some(0), _) => Ok(DefaultKind::Intrinsic),
                (_, Some(0)) => unreachable!(),
                (Some(_), _) => Ok(DefaultKind::Generic(DefaultImpl::U64)),
                (_, Some(_)) => Ok(DefaultKind::Generic(DefaultImpl::I64)),
            },
            TypeEntryDetails::Float(_) => {
                if let Some(value) = default.as_f64() {
                    if value == 0.0 {
                        Ok(DefaultKind::Intrinsic)
                    } else {
                        Ok(DefaultKind::Generic(DefaultImpl::I64))
                    }
                } else {
                    Err(Error::InvalidValue)
                }
            }
            TypeEntryDetails::String => {
                if let Some("") = default.as_str() {
                    Ok(DefaultKind::Intrinsic)
                } else {
                    Ok(DefaultKind::Specific)
                }
            }

            TypeEntryDetails::Reference(_) => unreachable!(),
        }
    }

    /// Return a string representing the function that can be called to produce
    /// the value for the given default. If there is no such built-in function,
    /// the .1 will be Some with a TokenStream for a function that can produce
    /// that value.
    pub(crate) fn default_fn(
        &self,
        default: &serde_json::Value,
        type_space: &TypeSpace,
        type_name: &str,
        prop_name: &str,
    ) -> (String, Option<TokenStream>) {
        let maybe_builtin = match &self.details {
            // This can only be covered by the intrinsic default
            TypeEntryDetails::Unit => unreachable!(),
            TypeEntryDetails::Boolean => Some("defaults::default_bool::<true>".to_string()),
            TypeEntryDetails::Integer(name) => {
                if let Some(value) = default.as_u64() {
                    Some(format!("defaults::default_u64::<{}, {}>", name, value))
                } else if let Some(value) = default.as_i64() {
                    Some(format!("defaults::default_i64::<{}, {}>", name, value))
                } else {
                    panic!()
                }
            }
            _ => None,
        };

        if let Some(fn_name) = maybe_builtin {
            (fn_name, None)
        } else {
            let n = self.type_ident(type_space, &Some("super".to_string()));
            let value = self.output_value(type_space, default).unwrap();
            let fn_name = sanitize(&format!("{}_{}", type_name, prop_name), Case::Snake);
            let fn_ident = format_ident!("{}", fn_name);
            let def = quote! {
                pub(super) fn #fn_ident() -> #n {
                    #value
                }
            };
            (format!("defaults::{}", fn_name), Some(def))
        }
    }
}

pub(crate) fn validate_default_for_external_enum(
    type_space: &TypeSpace,
    variants: &[Variant],
    default: &serde_json::Value,
) -> Option<DefaultKind> {
    if let Some(simple_name) = default.as_str() {
        let variant = variants
            .iter()
            .find(|variant| simple_name == variant.rename.as_ref().unwrap_or(&variant.name))?;
        matches!(&variant.details, VariantDetails::Simple).then(|| ())?;

        Some(DefaultKind::Specific)
    } else {
        let map = default.as_object()?;
        (map.len() == 1).then(|| ())?;

        let (name, value) = map.iter().next()?;

        let variant = variants
            .iter()
            .find(|variant| name == variant.rename.as_ref().unwrap_or(&variant.name))?;

        match &variant.details {
            VariantDetails::Simple => None,
            VariantDetails::Tuple(tup) => validate_default_tuple(tup, type_space, value),
            VariantDetails::Struct(props) => {
                validate_default_struct_props(props, type_space, value)
            }
        }
    }
}

pub(crate) fn validate_default_for_internal_enum(
    type_space: &TypeSpace,
    variants: &[Variant],
    default: &serde_json::Value,
    tag: &str,
) -> Option<DefaultKind> {
    let map = default.as_object()?;
    let name = map.get(tag).and_then(serde_json::Value::as_str)?;
    let variant = variants
        .iter()
        .find(|variant| name == variant.rename.as_ref().unwrap_or(&variant.name))?;

    match &variant.details {
        VariantDetails::Simple => Some(DefaultKind::Specific),
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
        VariantDetails::Tuple(_) => unreachable!(),
    }
}

pub(crate) fn validate_default_for_adjacent_enum(
    type_space: &TypeSpace,
    variants: &[Variant],
    default: &serde_json::Value,
    tag: &str,
    content: &str,
) -> Option<DefaultKind> {
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
        .find(|variant| tag_value == variant.rename.as_ref().unwrap_or(&variant.name))?;

    match (&variant.details, content_value) {
        (VariantDetails::Simple, None) => Some(DefaultKind::Specific),
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
    variants: &[Variant],
    default: &serde_json::Value,
) -> Option<DefaultKind> {
    variants.iter().find_map(|variant| {
        // The name of the variant is not meaningful; we just need to see
        // if any of the variants are valid with the given default.
        match &variant.details {
            VariantDetails::Simple => {
                default.as_null()?;
                Some(DefaultKind::Specific)
            }
            VariantDetails::Tuple(tup) => validate_default_tuple(tup, type_space, default),
            VariantDetails::Struct(props) => {
                validate_default_struct_props(props, type_space, default)
            }
        }
    })
}

fn validate_default_tuple(
    types: &[TypeId],
    type_space: &TypeSpace,
    default: &serde_json::Value,
) -> Option<DefaultKind> {
    let arr = default.as_array()?;
    (arr.len() == types.len()).then_some(())?;

    types
        .iter()
        .zip(arr.iter())
        .all(|(type_id, value)| {
            type_space
                .id_to_entry
                .get(type_id)
                .unwrap()
                .validate_value(type_space, value)
                .is_ok()
        })
        .then_some(DefaultKind::Specific)
}

fn validate_default_struct_props(
    properties: &[StructProperty],
    type_space: &TypeSpace,
    default: &serde_json::Value,
) -> Option<DefaultKind> {
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
            let type_entry = type_space.id_to_entry.get(type_id).unwrap();
            type_entry
                .validate_value(type_space, default_value)
                .ok()
                .map(|_| ())
        } else {
            unnamed_properties
                .iter()
                .any(|type_id| {
                    let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                    type_entry.validate_value(type_space, default_value).is_ok()
                })
                .then_some(())
        }
    })?;

    // Make sure that every required field is present in the map.
    named_properties
        .iter()
        .filter(|(_, (_, required))| *required)
        .try_for_each(|(name, _)| map.get(*name).map(|_| ()))?;

    Some(DefaultKind::Specific)
}

fn all_props<'a>(
    property: &'a StructProperty,
    type_space: &'a TypeSpace,
) -> Vec<(Option<&'a String>, &'a TypeId, bool)> {
    let maybe_name = match &property.rename {
        StructPropertyRename::None => Some(&property.name),
        StructPropertyRename::Rename(rename) => Some(rename),
        StructPropertyRename::Flatten => None,
    };

    if let Some(name) = maybe_name {
        let required = match &property.state {
            StructPropertyState::Required => true,
            StructPropertyState::Optional | StructPropertyState::Default(_) => false,
        };

        vec![(Some(name), &property.type_id, required)]
    } else {
        // The type must be a struct, an option for a struct, or a map.
        let type_entry = type_space.id_to_entry.get(&property.type_id).unwrap();

        let (properties, all_required) = match &type_entry.details {
            TypeEntryDetails::Struct(TypeEntryStruct { properties, .. }) => {
                let optional = matches!(&property.state, StructPropertyState::Optional);
                (properties, !optional)
            }
            TypeEntryDetails::Option(type_id) => {
                let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                if let TypeEntryDetails::Struct(TypeEntryStruct { properties, .. }) =
                    &type_entry.details
                {
                    (properties, false)
                } else {
                    unreachable!()
                }
            }

            TypeEntryDetails::Map(type_id) => return vec![(None, type_id, false)],
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

    use crate::{
        test_util::get_type,
        type_entry::{DefaultKind, TypeEntry},
        DefaultImpl,
    };

    #[test]
    fn test_default_option() {
        let (type_space, type_id) = get_type::<Option<u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("forty-two")),
            Err(_)
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(null)),
            Ok(DefaultKind::Intrinsic)
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(42)),
            Ok(DefaultKind::Specific)
        ));
    }

    #[test]
    fn test_default_box() {
        let (type_space, type_id) = get_type::<Option<u32>>();

        let type_entry = TypeEntry {
            details: crate::type_entry::TypeEntryDetails::Box(type_id),
            derives: Default::default(),
        };

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("forty-two")),
            Err(_)
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(null)),
            Ok(DefaultKind::Intrinsic)
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(42)),
            Ok(DefaultKind::Specific)
        ));
    }

    #[test]
    fn test_default_array() {
        let (type_space, type_id) = get_type::<Vec<u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([null])),
            Err(_),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([])),
            Ok(DefaultKind::Intrinsic),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([1, 2, 5])),
            Ok(DefaultKind::Specific),
        ));
    }

    #[test]
    fn test_default_map() {
        let (type_space, type_id) = get_type::<HashMap<String, u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([])),
            Err(_),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!({})),
            Ok(DefaultKind::Intrinsic),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!({"a": 1, "b": 2})),
            Ok(DefaultKind::Specific),
        ));
    }

    #[test]
    fn test_default_tuple() {
        let (type_space, type_id) = get_type::<(u32, u32, String)>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([1, 2, "three", 4])),
            Err(_),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!([1, 2, "three"])),
            Ok(DefaultKind::Specific),
        ));
    }

    #[test]
    fn test_default_builtin() {
        let (type_space, type_id) = get_type::<Uuid>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("not-a-uuid")),
            Ok(DefaultKind::Specific)
        ));
    }

    #[test]
    fn test_default_bool() {
        let (type_space, type_id) = get_type::<bool>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(false)),
            Ok(DefaultKind::Intrinsic),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(true)),
            Ok(DefaultKind::Generic(DefaultImpl::Boolean)),
        ));
    }

    #[test]
    fn test_default_numbers_and_string() {
        let (type_space, type_id) = get_type::<u32>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(true)),
            Err(_),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(0)),
            Ok(DefaultKind::Intrinsic),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(42)),
            Ok(DefaultKind::Generic(DefaultImpl::U64)),
        ));

        let (type_space, type_id) = get_type::<String>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("")),
            Ok(DefaultKind::Intrinsic),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("howdy")),
            Ok(DefaultKind::Specific),
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
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!(
                    {
                        "a": "aaaa",
                        "c": "cccc",
                        "d": 7
                    }
                )
            ),
            Err(_),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!(
                    {
                        "a": "aaaa",
                        "b": 7,
                        "d": {}
                    }
                )
            ),
            Err(_),
        ));
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
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "B": ["xx", "yy"]
                })
            ),
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "C": { "cc": "xx", "dd": "yy" }
                })
            ),
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!({ "A": null })),
            Err(_),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("B")),
            Err(_),
        ));
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
            Ok(DefaultKind::Specific),
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
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "targ": "A"
                })
            ),
            Err(_),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "B",
                    "cc": "where's D?"
                })
            ),
            Err(_),
        ));
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
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "B",
                    "content": ["xx", "yy"]
                })
            ),
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "C",
                    "content": { "cc": "xx", "dd": "yy" }
                })
            ),
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!("A")),
            Err(_),
        ));
        assert!(matches!(
            type_entry.validate_value(
                &type_space,
                &json!({
                    "tag": "A",
                    "content": null,
                })
            ),
            Err(_),
        ));
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
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!(["xx", "yy"])),
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!( { "cc": "xx", "dd": "yy" })),
            Ok(DefaultKind::Specific),
        ));
        assert!(matches!(
            type_entry.validate_value(&type_space, &json!({})),
            Err(_),
        ));
    }
}
