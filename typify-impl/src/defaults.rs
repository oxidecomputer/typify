// Copyright 2022 Oxide Computer Company

use std::collections::BTreeMap;

use convert_case::Case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    type_entry::{
        DefaultValue, EnumTagType, StructProperty, StructPropertyRename, StructPropertyState,
        TypeEntry, TypeEntryDetails, TypeEntryEnum, TypeEntryNewtype, TypeEntryStruct,
        ValidDefault, Variant, VariantDetails,
    },
    util::sanitize,
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
                default: Some(DefaultValue(default)),
                ..
            })
            | TypeEntryDetails::Struct(TypeEntryStruct {
                default: Some(DefaultValue(default)),
                ..
            })
            | TypeEntryDetails::Newtype(TypeEntryNewtype {
                default: Some(DefaultValue(default)),
                ..
            }) => {
                if let ValidDefault::Generic(default_fn) =
                    self.validate_default(default, type_space)?
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
            state: StructPropertyState::Default(DefaultValue(prop_default)),
            type_id,
            ..
        } = property
        {
            let type_entry = type_space.id_to_entry.get(type_id).unwrap();
            if let ValidDefault::Generic(default_fn) =
                type_entry.validate_default(prop_default, type_space)?
            {
                type_space.defaults.insert(default_fn);
            }
        }
        Ok(())
    }

    pub(crate) fn validate_default(
        &self,
        default: &serde_json::Value,
        type_space: &TypeSpace,
    ) -> Result<ValidDefault> {
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
            .ok_or(Error::InvalidDefaultValue),
            TypeEntryDetails::Struct(TypeEntryStruct { properties, .. }) => {
                validate_default_struct_props(properties, type_space, default)
                    .ok_or(Error::InvalidDefaultValue)
            }

            TypeEntryDetails::Newtype(TypeEntryNewtype { type_id, .. }) => type_space
                .id_to_entry
                .get(type_id)
                .unwrap()
                .validate_default(default, type_space),
            TypeEntryDetails::Option(type_id) => {
                if let serde_json::Value::Null = default {
                    Ok(ValidDefault::Intrinsic)
                } else {
                    let ty = type_space.id_to_entry.get(type_id).unwrap();
                    // Make sure the default is valid for the sub-type.
                    let _ = ty.validate_default(default, type_space)?;
                    Ok(ValidDefault::Specific)
                }
            }
            TypeEntryDetails::Box(type_id) => type_space
                .id_to_entry
                .get(type_id)
                .unwrap()
                .validate_default(default, type_space),

            TypeEntryDetails::Array(type_id) => {
                if let serde_json::Value::Array(v) = default {
                    if v.is_empty() {
                        Ok(ValidDefault::Intrinsic)
                    } else {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        for value in v {
                            let _ = type_entry.validate_default(value, type_space)?;
                        }
                        Ok(ValidDefault::Specific)
                    }
                } else {
                    Err(Error::InvalidDefaultValue)
                }
            }
            TypeEntryDetails::Map(type_id) => {
                if let serde_json::Value::Object(m) = default {
                    if m.is_empty() {
                        Ok(ValidDefault::Intrinsic)
                    } else {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        for (_, value) in m {
                            let _ = type_entry.validate_default(value, type_space)?;
                        }
                        Ok(ValidDefault::Specific)
                    }
                } else {
                    Err(Error::InvalidDefaultValue)
                }
            }
            TypeEntryDetails::Set(type_id) => {
                if let serde_json::Value::Array(v) = default {
                    if v.is_empty() {
                        Ok(ValidDefault::Intrinsic)
                    } else {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        for (i, value) in v.iter().enumerate() {
                            // Sets can't contain duplicates; also Value isn't
                            // Ord so O(n^2) it is!
                            for other in &v[(i + 1)..] {
                                if value == other {
                                    return Err(Error::InvalidDefaultValue);
                                }
                            }
                            let _ = type_entry.validate_default(value, type_space)?;
                        }
                        Ok(ValidDefault::Specific)
                    }
                } else {
                    Err(Error::InvalidDefaultValue)
                }
            }
            TypeEntryDetails::Tuple(ids) => {
                validate_default_tuple(ids, type_space, default).ok_or(Error::InvalidDefaultValue)
            }
            TypeEntryDetails::Unit => {
                if let serde_json::Value::Null = default {
                    Ok(ValidDefault::Intrinsic)
                } else {
                    Err(Error::InvalidDefaultValue)
                }
            }
            TypeEntryDetails::BuiltIn(_) => {
                // TODO Not sure what could be done here...
                Err(Error::InvalidDefaultValue)
            }
            TypeEntryDetails::Boolean => match default {
                serde_json::Value::Bool(false) => Ok(ValidDefault::Intrinsic),
                serde_json::Value::Bool(true) => Ok(ValidDefault::Generic(DefaultImpl::Boolean)),
                _ => Err(Error::InvalidDefaultValue),
            },
            TypeEntryDetails::Integer(_) if default.is_u64() => {
                if let Some(0) = default.as_u64() {
                    Ok(ValidDefault::Intrinsic)
                } else {
                    Ok(ValidDefault::Generic(DefaultImpl::U64))
                }
            }
            TypeEntryDetails::Integer(_) => {
                if let Some(0) = default.as_i64() {
                    Ok(ValidDefault::Intrinsic)
                } else {
                    Ok(ValidDefault::Generic(DefaultImpl::I64))
                }
            }
            TypeEntryDetails::Float(_) => {
                if let Some(value) = default.as_f64() {
                    if value == 0.0 {
                        Ok(ValidDefault::Intrinsic)
                    } else {
                        Ok(ValidDefault::Generic(DefaultImpl::I64))
                    }
                } else {
                    Err(Error::InvalidDefaultValue)
                }
            }
            TypeEntryDetails::String => {
                if let Some("") = default.as_str() {
                    Ok(ValidDefault::Intrinsic)
                } else {
                    Ok(ValidDefault::Generic(DefaultImpl::I64))
                }
            }

            TypeEntryDetails::Reference(_) => todo!(),
        }
    }

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
            TypeEntryDetails::Boolean => Some("defaults::default_bool::<false>".to_string()),
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
            let n = self.type_ident(type_space, false);
            let value = self.value(type_space, default).unwrap();
            let fn_name = sanitize(&format!("{}_{}", type_name, prop_name), Case::Snake);
            let fn_ident = format_ident!("{}", fn_name);
            let def = quote! {
                fn #fn_ident() -> #n { #value }
            };
            (fn_name, Some(def))
        }
    }
}

pub(crate) fn validate_default_for_external_enum(
    type_space: &TypeSpace,
    variants: &[Variant],
    default: &serde_json::Value,
) -> Option<ValidDefault> {
    if let Some(simple_name) = default.as_str() {
        let variant = variants
            .iter()
            .find(|variant| simple_name == variant.rename.as_ref().unwrap_or(&variant.name))?;
        matches!(&variant.details, VariantDetails::Simple).then(|| ())?;

        Some(ValidDefault::Specific)
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
) -> Option<ValidDefault> {
    let map = default.as_object()?;
    let name = map.get(tag).and_then(serde_json::Value::as_str)?;
    let variant = variants
        .iter()
        .find(|variant| name == variant.rename.as_ref().unwrap_or(&variant.name))?;

    match &variant.details {
        VariantDetails::Simple => Some(ValidDefault::Specific),
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
) -> Option<ValidDefault> {
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
        (VariantDetails::Simple, None) => Some(ValidDefault::Specific),
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
) -> Option<ValidDefault> {
    variants.iter().find_map(|variant| {
        // The name of the variant is not meaningful; we just need to see
        // if any of the variants are valid with the given default.
        match &variant.details {
            VariantDetails::Simple => {
                default.as_null()?;
                Some(ValidDefault::Specific)
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
) -> Option<ValidDefault> {
    let arr = default.as_array()?;
    (arr.len() == types.len()).then(|| ())?;

    types
        .iter()
        .zip(arr.iter())
        .all(|(type_id, value)| {
            type_space
                .id_to_entry
                .get(type_id)
                .unwrap()
                .validate_default(value, type_space)
                .is_ok()
        })
        .then(|| ValidDefault::Specific)
}

fn validate_default_struct_props(
    properties: &[StructProperty],
    type_space: &TypeSpace,
    default: &serde_json::Value,
) -> Option<ValidDefault> {
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
                .validate_default(default_value, type_space)
                .ok()
                .map(|_| ())
        } else {
            unnamed_properties
                .iter()
                .any(|type_id| {
                    let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                    type_entry
                        .validate_default(default_value, type_space)
                        .is_ok()
                })
                .then(|| ())
        }
    })?;

    // Make sure that every required field is present in the map.
    named_properties
        .iter()
        .filter(|(_, (_, required))| *required)
        .try_for_each(|(name, _)| map.get(*name).map(|_| ()))?;

    Some(ValidDefault::Specific)
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
