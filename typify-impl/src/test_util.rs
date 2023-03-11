// Copyright 2022 Oxide Computer Company

use std::{any::type_name, collections::HashSet};

use proc_macro2::TokenStream;
use quote::ToTokens;
use rustfmt_wrapper::rustfmt;
use schema::Schema;
use schemars::{schema_for, JsonSchema};
use syn::{
    parse2, punctuated::Punctuated, Attribute, DataEnum, DataStruct, DeriveInput, Field, Fields,
    FieldsNamed, FieldsUnnamed, File, Type, TypePath, TypeTuple, Variant,
};

use crate::{output::OutputSpace, Name, TypeId, TypeSpace};

pub(crate) fn get_type<T: JsonSchema>() -> (TypeSpace, TypeId) {
    let schema = schema_for!(T);

    let type_name = type_name::<T>();
    let name = type_name
        .rsplit_once("::")
        .map_or(type_name, |split| split.1)
        .to_string();

    let mut type_space = TypeSpace::default();

    // Convert all references
    type_space
        .add_ref_types(schema.definitions.clone())
        .unwrap();

    // In some situations, `schema_for!(T)` may actually give us two copies
    // of the type: one in the definitions and one in the schema. This will
    // occur in particular for cyclic types i.e. those for which the type
    // itself is a reference.
    //
    // If we have converted the type already, use that, otherwise convert
    // schema object
    let type_id = if let Some(already_type_id) = type_space.ref_to_id.get(&name) {
        already_type_id.clone()
    } else {
        type_space
            .add_type_with_name(&schema.schema.into(), Some(name.to_string()))
            .unwrap()
    };

    (type_space, type_id)
}

/// Ingest a type, spit it back out, and make sure it matches where we started.
#[track_caller]
pub(crate) fn validate_output<T: JsonSchema + Schema>() {
    validate_output_impl::<T>(false)
}

/// Same as `validate_output` but ignores differences of the top-level enum's
/// variant names which are lost in the case of `#[serde(untagged)]`
#[track_caller]
pub(crate) fn validate_output_for_untagged_enm<T: JsonSchema + Schema>() {
    validate_output_impl::<T>(true)
}

#[track_caller]
fn validate_output_impl<T: JsonSchema + Schema>(ignore_variant_names: bool) {
    let (type_space, type_id) = get_type::<T>();
    let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

    let mut output = OutputSpace::default();
    type_entry.output(&type_space, &mut output);
    let output = output.into_stream();

    let expected = T::schema();
    // We may generate more than one item for a given schema. For example, we
    // may generate `impl`s with convenient functions. We do the somewhat
    // arcane dance here of parsing the full output, extracting the first item,
    // converting it **back** to tokens, and then parsing it again as
    // DeriveInput.
    let file = parse2::<File>(output.clone()).unwrap();
    assert!(!file.items.is_empty(), "{}", output.to_string());
    let actual = parse2::<DeriveInput>(file.items.first().unwrap().to_token_stream()).unwrap();

    // Make sure they match.
    if let Err(err) = expected.syn_cmp(&actual, ignore_variant_names) {
        println!("{}", serde_json::to_string_pretty(&schema_for!(T)).unwrap());
        println!("{}", rustfmt(output.to_string()).unwrap());
        panic!("{}", err);
    }
}

#[macro_export]
macro_rules! validate_builtin {
    ($t:ty) => {
        crate::test_util::validate_builtin_impl::<$t>(stringify!($t))
    };
}

#[track_caller]
pub(crate) fn validate_builtin_impl<T: JsonSchema>(name: &str) {
    let schema = schema_for!(T);

    let mut type_space = TypeSpace::default();
    type_space
        .add_ref_types(schema.definitions.clone())
        .unwrap();
    let (ty, _) = type_space
        .convert_schema_object(Name::Unknown, &schema.schema)
        .unwrap();

    let output = ty.type_ident(&type_space, &None);

    let actual = syn::parse2::<syn::Type>(output.clone()).unwrap();
    let expected = syn::parse_str::<syn::Type>(name).unwrap();

    // Make sure they match.
    if let Err(err) = expected.syn_cmp(&actual, false) {
        println!("{:#?}", schema);
        println!("actual: {}", output);
        println!("expected: {}", name);
        panic!("{}", err);
    }
}

pub(crate) trait SynCompare {
    fn syn_cmp(&self, other: &Self, ignore_variant_names: bool) -> Result<(), String>;
}

impl SynCompare for DeriveInput {
    fn syn_cmp(&self, other: &Self, ignore_variant_names: bool) -> Result<(), String> {
        self.ident.syn_cmp(&other.ident, false)?;

        // Just compare the attributes we're interested in
        compare_attributes(&self.attrs, &other.attrs)?;

        match (&self.data, &other.data) {
            (syn::Data::Struct(a), syn::Data::Struct(b)) => a.syn_cmp(b, ignore_variant_names),
            (syn::Data::Enum(a), syn::Data::Enum(b)) => a.syn_cmp(b, ignore_variant_names),
            (syn::Data::Union(_), syn::Data::Union(_)) => {
                Err("unions are not supported".to_string())
            }
            _ => Err("mismatched data".to_string()),
        }
    }
}

fn compare_attributes(attrs_a: &[Attribute], attrs_b: &[Attribute]) -> Result<(), String> {
    let serde_options_a = get_serde(attrs_a);
    let serde_options_b = get_serde(attrs_b);

    if serde_options_a == serde_options_b {
        Ok(())
    } else {
        Err(format!(
            "different serde options: {:?} {:?}",
            serde_options_a, serde_options_b
        ))
    }
}

fn get_serde(attrs: &[Attribute]) -> HashSet<String> {
    attrs
        .iter()
        .filter_map(|attr| {
            let name = attr.path.segments.first()?.ident.to_string();
            if name == "serde" {
                let mut iter = attr.tokens.clone().into_iter();
                if let Some(proc_macro2::TokenTree::Group(group)) = iter.next() {
                    // Serde options have a single item.
                    assert!(iter.next().is_none());
                    // Return the list of discrete serde options
                    return Some(
                        group
                            .stream()
                            .into_iter()
                            .collect::<Vec<_>>()
                            // Split into comma-delimited groups.
                            .split(|token| matches!(token, proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ','))
                            // Join the tokens into a string.
                            .map(|tokens| {
                                tokens.iter().cloned().collect::<TokenStream>().to_string()
                            })
                            // Remove rename statements because there are many
                            // ways to get to the same place.
                            .filter(|s| !s.starts_with("rename"))
                            .collect::<Vec<_>>(),
                    );
                }
            }
            None
        })
        .flatten()
        .collect()
}

impl SynCompare for syn::Ident {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        if self != other {
            Err(format!("idents differ: {} {}", self, other))
        } else {
            Ok(())
        }
    }
}

impl SynCompare for DataStruct {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        self.fields.syn_cmp(&other.fields, false)
    }
}

impl SynCompare for DataEnum {
    fn syn_cmp(&self, other: &Self, ignore_variant_names: bool) -> Result<(), String> {
        self.variants.syn_cmp(&other.variants, ignore_variant_names)
    }
}

impl<T, P> SynCompare for Punctuated<T, P>
where
    T: SynCompare,
{
    fn syn_cmp(&self, other: &Self, ignore_variant_names: bool) -> Result<(), String> {
        if self.len() != other.len() {
            return Err(format!(
                "lengths don't match: {:?} != {:?}",
                self.len(),
                other.len()
            ));
        }
        self.iter()
            .zip(other.iter())
            .try_for_each(|(a, b)| a.syn_cmp(b, ignore_variant_names))
    }
}

impl<T> SynCompare for Option<T>
where
    T: SynCompare,
{
    fn syn_cmp(&self, other: &Self, ignore_variant_names: bool) -> Result<(), String> {
        match (self, other) {
            (None, None) => Ok(()),
            (Some(a), Some(b)) => a.syn_cmp(b, ignore_variant_names),
            _ => Err("options don't match".to_string()),
        }
    }
}

impl SynCompare for Variant {
    fn syn_cmp(&self, other: &Self, ignore_variant_names: bool) -> Result<(), String> {
        if !ignore_variant_names {
            self.ident.syn_cmp(&other.ident, false)?;
        }
        self.fields.syn_cmp(&other.fields, false)
    }
}

impl SynCompare for Fields {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        match (self, other) {
            (Fields::Named(a), Fields::Named(b)) => a.syn_cmp(b, false),
            (Fields::Unnamed(a), Fields::Unnamed(b)) => a.syn_cmp(b, false),
            (Fields::Unit, Fields::Unit) => Ok(()),
            _ => Err("mismatched field types".to_string()),
        }
    }
}

impl SynCompare for FieldsNamed {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        self.named.syn_cmp(&other.named, false)
    }
}

impl SynCompare for FieldsUnnamed {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        self.unnamed.syn_cmp(&other.unnamed, false)
    }
}

impl SynCompare for Field {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        self.ident.syn_cmp(&other.ident, false)?;
        self.ty.syn_cmp(&other.ty, false)?;
        Ok(())
    }
}

impl SynCompare for Type {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        match (self, other) {
            (Type::Tuple(a), Type::Tuple(b)) => a.syn_cmp(b, false),
            (Type::Path(a), Type::Path(b)) => a.syn_cmp(b, false),
            _ => Err(format!(
                "unexpected or mismatched type pair: {:?} {:?}",
                self, other
            )),
        }
    }
}

impl SynCompare for TypeTuple {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        self.elems.syn_cmp(&other.elems, false)
    }
}

impl SynCompare for TypePath {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        assert!(self.qself.is_none());
        assert!(other.qself.is_none());

        if self.path != other.path {
            Err(format!(
                "paths did not match {:?} {:?}",
                self.path, other.path
            ))
        } else {
            Ok(())
        }
    }
}
