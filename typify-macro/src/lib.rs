use std::path::Path;

use proc_macro::TokenStream;
use quote::quote;
use schemars::schema::Schema;
use syn::LitStr;
use typify_impl::TypeSpace;

#[proc_macro]
pub fn import_types(item: TokenStream) -> TokenStream {
    match do_import_types(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out,
    }
}

fn do_import_types(item: TokenStream) -> Result<TokenStream, syn::Error> {
    let arg = syn::parse::<LitStr>(item)?;
    let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| std::env::current_dir().unwrap(),
        |s| Path::new(&s).to_path_buf(),
    );

    let path = dir.join(arg.value());

    let content = std::fs::read_to_string(path).map_err(|e| {
        syn::Error::new(
            arg.span(),
            format!("couldn't read file {}: {}", arg.value(), e.to_string()),
        )
    })?;

    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::default();
    type_space
        .add_ref_types(schema.definitions)
        .map_err(|e| into_syn_err(e, arg.span()))?;
    let base_type = &schema.schema;
    // Only convert the top-level type if it has a name
    if (|| base_type.metadata.as_ref()?.title.as_ref())().is_some() {
        let _ = type_space
            .add_type(&Schema::Object(schema.schema))
            .map_err(|e| into_syn_err(e, arg.span()))?;
    }

    let type_defs = type_space.iter_types().map(|t| t.definition());

    let file = quote! {
        #(#type_defs)*
    };
    Ok(file.into())
}

fn into_syn_err(e: typify_impl::Error, span: proc_macro2::Span) -> syn::Error {
    syn::Error::new(span, e.to_string())
}
