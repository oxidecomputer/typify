// Copyright 2021 Oxide Computer Company

use std::path::Path;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use schemars::schema::Schema;
use syn::{parse::Parse, LitStr, Token};
use typify_impl::TypeSpace;

/// Import types by providing a pathname for a JSON Schema file. The path must
/// be relative to `$CARGO_MANIFEST_DIR`.
#[proc_macro]
pub fn import_types(item: TokenStream) -> TokenStream {
    match do_import_types(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out,
    }
}

struct Settings {
    file: LitStr,
    derives: Vec<syn::Path>,
}

impl Parse for Settings {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let file = input.parse()?;
        if !input.is_empty() {
            let _: Token!(,) = input.parse()?;
        }

        let mut derives = Vec::new();
        while !input.is_empty() {
            derives.push(input.parse()?);
            if input.is_empty() {
                break;
            } else {
                let _: Token!(,) = input.parse()?;
            }
        }

        Ok(Self { file, derives })
    }
}

fn do_import_types(item: TokenStream) -> Result<TokenStream, syn::Error> {
    let Settings { file, derives } = syn::parse(item)?;
    let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| std::env::current_dir().unwrap(),
        |s| Path::new(&s).to_path_buf(),
    );

    let path = dir.join(file.value());

    let schema: schemars::schema::RootSchema =
        serde_json::from_reader(std::fs::File::open(&path).map_err(|e| {
            syn::Error::new(
                file.span(),
                format!("couldn't read file {}: {}", file.value(), e.to_string()),
            )
        })?)
        .unwrap();

    let mut type_space = TypeSpace::default();
    derives
        .iter()
        .for_each(|derive| type_space.add_derive(derive.to_token_stream().to_string()));
    type_space
        .add_ref_types(schema.definitions)
        .map_err(|e| into_syn_err(e, file.span()))?;
    let base_type = &schema.schema;
    // Only convert the top-level type if it has a name
    if (|| base_type.metadata.as_ref()?.title.as_ref())().is_some() {
        let _ = type_space
            .add_type(&Schema::Object(schema.schema))
            .map_err(|e| into_syn_err(e, file.span()))?;
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
