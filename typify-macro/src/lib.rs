use std::path::Path;

use proc_macro::TokenStream;
use syn::LitStr;
use typify_impl::TypeSpace;

#[proc_macro]
pub fn import_types(item: TokenStream) -> TokenStream {
    match syn::parse::<LitStr>(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(arg) => {
            let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
                |_| std::env::current_dir().unwrap(),
                |s| Path::new(&s).to_path_buf(),
            );

            let path = dir.join(arg.value());

            let content = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    return syn::Error::new(
                        arg.span(),
                        format!("couldn't read file {}: {}", arg.value(), e.to_string()),
                    )
                    .to_compile_error()
                    .into();
                }
            };

            let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

            println!("arg {:#?}", schema);

            let mut type_space = TypeSpace::new(&schema.definitions).unwrap();
            let x = type_space
                .convert_schema_object(Some("myname"), &schema.schema)
                .unwrap();

            println!("{:#?}", x.0);
            let ret = x.0.output(&type_space);
            println!("{}", ret.to_string());

            ret.into()
        }
    }
}
