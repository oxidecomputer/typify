// Copyright 2022 Oxide Computer Company

use std::{collections::HashMap, path::Path};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use schemars::schema::Schema;
use serde::Deserialize;
use serde_tokenstream::ParseWrapper;
use syn::LitStr;
use typify_impl::{TypeSpace, TypeSpacePatch, TypeSpaceSettings};

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
/// - `patch`: optional map of type to an object with the optional members
///   `rename` and `derives` for type-specific renames and derive macros.
///   
/// - `struct_builder`: optional boolean; (if true) generates a `::builder()`
///   method for each generated struct that can be used to specify each
///   property and construct the struct
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
    patch: HashMap<ParseWrapper<syn::Type>, MacroPatch>,
    #[serde(default)]
    convert: serde_tokenstream::OrderedMap<
        schemars::schema::SchemaObject,
        (ParseWrapper<syn::Path>, Vec<ParseWrapper<syn::Path>>),
    >,
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
            s.with_derive(derive.to_token_stream().to_string());
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
        convert
            .into_iter()
            .for_each(|(schema, (type_name, impls))| {
                settings.with_conversion(
                    schema,
                    type_name.to_token_stream(),
                    impls.into_iter().map(|x| x.to_token_stream()),
                );
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
        .add_ref_types(root_schema.definitions)
        .map_err(|e| into_syn_err(e, schema.span()))?;
    let base_type = &root_schema.schema;
    // Only convert the top-level type if it has a name
    if (|| base_type.metadata.as_ref()?.title.as_ref())().is_some() {
        let _ = type_space
            .add_type(&Schema::Object(root_schema.schema))
            .map_err(|e| into_syn_err(e, schema.span()))?;
    }

    let types = type_space.to_stream();
    let path_str = path.to_string_lossy();
    let output = quote! {
        #types

        // Force a rebuild when the given file is modified.
        const _: &str = include_str!(#path_str);
    };

    Ok(output.into())
}

fn into_syn_err(e: typify_impl::Error, span: proc_macro2::Span) -> syn::Error {
    syn::Error::new(span, e.to_string())
}
