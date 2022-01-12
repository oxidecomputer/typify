// Copyright 2021 Oxide Computer Company

use std::path::Path;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use schemars::schema::Schema;
use serde::Deserialize;
use serde_tokenstream::ParseWrapper;
use syn::LitStr;
use typify_impl::TypeSpace;

/// Import types from a schema file. This may be invoked with simply a pathname
/// for a JSON Schema file (relative to `$CARGO_MANIFEST_DIR`), or it may be
/// invoked with a structured form:
/// ```
/// # use typify_macro::import_types;
/// # use serde::{Deserialize,Serialize};
/// import_types!(
///     schema = "../example.json",
///     derives = [schemars::JsonSchema],
/// );
/// ```
#[proc_macro]
pub fn import_types(item: TokenStream) -> TokenStream {
    match do_import_types(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out,
    }
}

#[derive(Deserialize)]
struct Settings {
    schema: ParseWrapper<LitStr>,
    #[serde(default)]
    derives: Vec<ParseWrapper<syn::Path>>,
}

fn do_import_types(item: TokenStream) -> Result<TokenStream, syn::Error> {
    // Allow the caller to give us either a simple string or a compound object.
    let (schema, derives) = if let Ok(ll) = syn::parse::<LitStr>(item.clone()) {
        (ll, Vec::new())
    } else {
        let Settings { schema, derives } = serde_tokenstream::from_tokenstream(&item.into())?;
        (
            schema.into_inner(),
            derives.into_iter().map(ParseWrapper::into_inner).collect(),
        )
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
                format!("couldn't read file {}: {}", schema.value(), e.to_string()),
            )
        })?)
        .unwrap();

    let mut type_space = TypeSpace::default();
    derives
        .iter()
        .cloned()
        .for_each(|derive| type_space.add_derive(derive.to_token_stream()));
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
