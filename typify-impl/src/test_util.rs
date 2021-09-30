use schema::Schema;
use schemars::{schema_for, JsonSchema};
use syn::{
    parse2, punctuated::Punctuated, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
    FieldsUnnamed, Type, TypePath, TypeTuple, Variant,
};

use crate::{Name, TypeSpace};

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

fn validate_output_impl<T: JsonSchema + Schema>(ignore_variant_names: bool) {
    let schema = schema_for!(T);

    let mut type_space = TypeSpace::new(&schema.definitions).unwrap();
    let (ty, _) = type_space
        .convert_schema_object(Name::Unknown, &schema.schema)
        .unwrap();

    let output = ty.output(&type_space);

    let expected = T::schema();
    let actual = parse2::<DeriveInput>(output).unwrap();

    expected.syn_cmp(&actual, ignore_variant_names).unwrap()
}

pub(crate) trait SynCompare {
    fn syn_cmp(&self, other: &Self, ignore_variant_names: bool) -> Result<(), String>;
}

impl SynCompare for DeriveInput {
    fn syn_cmp(&self, other: &Self, ignore_variant_names: bool) -> Result<(), String> {
        self.ident.syn_cmp(&other.ident, false)?;

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

impl SynCompare for syn::Ident {
    fn syn_cmp(&self, other: &Self, _: bool) -> Result<(), String> {
        if self != other {
            Err(format!(
                "idents differ: {} {}",
                self.to_string(),
                other.to_string()
            ))
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
                "unexpected or mistmatched type pair: {:?} {:?}",
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
