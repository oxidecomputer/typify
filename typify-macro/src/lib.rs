// Copyright 2023 Oxide Computer Company

use std::{collections::HashMap, path::Path};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;
use serde_tokenstream::ParseWrapper;
use syn::LitStr;
use token_utils::TypeAndImpls;
use typify_impl::{TypeSpace, TypeSpacePatch, TypeSpaceSettings};

mod token_utils;

/// Import types from a schema file. This may be invoked with simply a pathname
/// for a JSON Schema file (relative to `$CARGO_MANIFEST_DIR`), or it may be
/// invoked with a structured form:
/// ```
/// use typify_macro::import_types;
/// use serde::{Deserialize,Serialize};
/// import_types!(
///     schema = "../example.json",
///     derives = [schemars::JsonSchema],
/// );
/// ```
///
/// - `schema`: string literal; the JSON schema file
///
/// - `derives`: optional array of derive macro paths; the derive macros to be
///   applied to all generated types
///
/// - `struct_builder`: optional boolean; (if true) generates a `::builder()`
///   method for each generated struct that can be used to specify each
///   property and construct the struct
///
/// - `patch`: optional map from type to an object with the optional members
///   `rename` and `derives`. This may be used to renamed generated types or
///   to apply additional (non-default) derive macros to them.
///
/// - `replace`: optional map from definition name to a replacement type. This
///   may be used to skip generation of the named type and use a existing Rust
///   type.
///   
/// - `convert`: optional map from a JSON schema to a replacement type. This
///   may be used to skip generation of the schema and use an existing Rust
///   type.
#[proc_macro]
pub fn import_types(item: TokenStream) -> TokenStream {
    match do_import_types(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out,
    }
}

#[derive(Deserialize)]
struct MacroSettings {
    schema: ParseWrapper<LitStr>,
    #[serde(default)]
    derives: Vec<ParseWrapper<syn::Path>>,
    #[serde(default)]
    struct_builder: bool,

    #[serde(default)]
    patch: HashMap<ParseWrapper<syn::Ident>, MacroPatch>,
    #[serde(default)]
    replace: HashMap<ParseWrapper<syn::Ident>, ParseWrapper<TypeAndImpls>>,
    #[serde(default)]
    convert:
        serde_tokenstream::OrderedMap<schemars::schema::SchemaObject, ParseWrapper<TypeAndImpls>>,
}

#[derive(Deserialize)]
struct MacroPatch {
    #[serde(default)]
    rename: Option<String>,
    #[serde(default)]
    derives: Vec<ParseWrapper<syn::Path>>,
}

impl From<MacroPatch> for TypeSpacePatch {
    fn from(a: MacroPatch) -> Self {
        let mut s = Self::default();
        a.rename.iter().for_each(|rename| {
            s.with_rename(rename);
        });
        a.derives.iter().for_each(|derive| {
            s.with_derive(derive.to_token_stream());
        });
        s
    }
}

fn do_import_types(item: TokenStream) -> Result<TokenStream, syn::Error> {
    // Allow the caller to give us either a simple string or a compound object.
    let (schema, settings) = if let Ok(ll) = syn::parse::<LitStr>(item.clone()) {
        (ll, TypeSpaceSettings::default())
    } else {
        let MacroSettings {
            schema,
            derives,
            replace,
            patch,
            struct_builder,
            convert,
        } = serde_tokenstream::from_tokenstream(&item.into())?;
        let mut settings = TypeSpaceSettings::default();
        derives.into_iter().for_each(|derive| {
            settings.with_derive(derive.to_token_stream().to_string());
        });
        settings.with_struct_builder(struct_builder);

        patch.into_iter().for_each(|(type_name, patch)| {
            settings.with_patch(type_name.to_token_stream(), &patch.into());
        });
        replace.into_iter().for_each(|(type_name, type_and_impls)| {
            let (replace_type, impls) = type_and_impls.into_inner().into_name_and_impls();
            settings.with_replacement(type_name.to_token_stream(), replace_type, impls.into_iter());
        });
        convert.into_iter().for_each(|(schema, type_and_impls)| {
            let (type_name, impls) = type_and_impls.into_inner().into_name_and_impls();
            settings.with_conversion(schema, type_name, impls);
        });

        (schema.into_inner(), settings)
    };

    let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| std::env::current_dir().unwrap(),
        |s| Path::new(&s).to_path_buf(),
    );

    let path = dir.join(schema.value());

    let root_schema: schemars::schema::RootSchema =
        serde_json::from_reader(std::fs::File::open(&path).map_err(|e| {
            syn::Error::new(
                schema.span(),
                format!("couldn't read file {}: {}", schema.value(), e),
            )
        })?)
        .unwrap();

    let mut type_space = TypeSpace::new(&settings);
    type_space
        .add_root_schema(root_schema)
        .map_err(|e| into_syn_err(e, schema.span()))?;

    let path_str = path.to_string_lossy();
    let output = quote! {
        #type_space

        // Force a rebuild when the given file is modified.
        const _: &str = include_str!(#path_str);
    };

    Ok(output.into())
}

fn into_syn_err(e: typify_impl::Error, span: proc_macro2::Span) -> syn::Error {
    syn::Error::new(span, e.to_string())
}
