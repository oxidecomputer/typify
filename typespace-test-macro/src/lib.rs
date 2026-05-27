use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, ItemFn, LitStr, Token,
};

struct MacroArgs {
    filename: LitStr,
    output_expr: Expr,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let filename: LitStr = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let output_expr: Expr = input.parse()?;
        Ok(MacroArgs {
            filename,
            output_expr,
        })
    }
}

fn pretty_tokens(output_expr: &Expr) -> proc_macro2::TokenStream {
    quote! {
        {
            let __output_tokens = #output_expr;
            let __file: ::syn::File = ::syn::parse2(__output_tokens)
                .expect("failed to parse rendered output as Rust file");
            ::prettyplease::unparse(&__file)
        }
    }
}

fn expand_missing_file(filename: &str, output_expr: &Expr) -> proc_macro2::TokenStream {
    let pretty = pretty_tokens(output_expr);
    quote! {
        {
            let __snapshot_path = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(#filename);
            let __content: ::std::string::String = #pretty;
            if let Some(parent) = __snapshot_path.parent() {
                ::std::fs::create_dir_all(parent).ok();
            }
            ::std::fs::write(&__snapshot_path, &__content)
                .expect("failed to write snapshot");
            let _ = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #filename));
            panic!(
                "snapshot file created, run tests again: {}",
                __snapshot_path.display()
            );
        }
    }
}

fn expand_inner(
    filename: &str,
    output_expr: &Expr,
    file_tokens: proc_macro2::TokenStream,
    body_stmts: &[syn::Stmt],
) -> proc_macro2::TokenStream {
    let pretty = pretty_tokens(output_expr);
    quote! {
        {
            let _ = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #filename));

            let __snapshot_path = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(#filename);
            let __content: ::std::string::String = #pretty;
            let __needs_update = match ::std::fs::read_to_string(&__snapshot_path) {
                Ok(ref existing) => existing != &__content,
                Err(_) => true,
            };
            if __needs_update {
                if let Some(parent) = __snapshot_path.parent() {
                    ::std::fs::create_dir_all(parent).ok();
                }
                ::std::fs::write(&__snapshot_path, __content)
                    .expect("failed to write snapshot");
                panic!(
                    "snapshot updated, run tests again: {}",
                    __snapshot_path.display()
                );
            }
            mod import {
                use super::*;
                #file_tokens
            }
            #( #body_stmts )*
        }
    }
}

/// Attribute macro for snapshot-testing rendered Rust code.
///
/// Usage:
/// ```ignore
/// #[check_and_include("tests/output/test_unit_struct.rs", ts.render())]
/// fn inner() {
///     let value = import::MyUnitStruct;
///     assert_eq!(serde_json::to_string(&value).unwrap(), "\"<<+>>\"");
/// }
/// ```
///
/// The annotated function is replaced by an inline block that:
/// 1. Evaluates the expression, pretty-prints it, and compares against the snapshot file;
///    updates + panics if different.
/// 2. Embeds the snapshot file content as `mod import { ... }` (read at macro expansion time).
/// 3. Runs the original function body.
///
/// The annotated function must have no parameters.
#[proc_macro_attribute]
pub fn check_and_include(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);
    let func = parse_macro_input!(item as ItemFn);

    if !func.sig.inputs.is_empty() {
        return syn::Error::new_spanned(
            &func.sig.inputs,
            "check_and_include: function must have no parameters",
        )
        .to_compile_error()
        .into();
    }

    let filename_str = args.filename.value();
    let output_expr = &args.output_expr;
    let body_stmts = &func.block.stmts;

    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => dir,
        Err(_) => {
            return syn::Error::new_spanned(
                &args.filename,
                "CARGO_MANIFEST_DIR is not set; cannot resolve snapshot path",
            )
            .to_compile_error()
            .into();
        }
    };

    let snapshot_path = std::path::Path::new(&manifest_dir).join(&filename_str);

    let file_tokens: proc_macro2::TokenStream = match std::fs::read_to_string(&snapshot_path) {
        Ok(content) => match proc_macro2::TokenStream::from_str(&content) {
            Ok(ts) => ts,
            Err(_) => {
                return expand_missing_file(&filename_str, output_expr).into();
            }
        },
        Err(_) => {
            let _ = std::fs::write(&snapshot_path, "");
            return expand_missing_file(&filename_str, output_expr).into();
        }
    };

    expand_inner(&filename_str, output_expr, file_tokens, body_stmts).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_expansion_missing_file() {
        let output_expr: Expr = parse_quote! { ts.render() };
        let expanded = expand_missing_file("tests/output/missing.rs", &output_expr);
        let wrapped: syn::File = parse_quote! { fn wrapper() { #expanded } };
        let out = prettyplease::unparse(&wrapped);
        expectorate::assert_contents("tests/output/test_expansion_missing_file.rs", &out);
    }

    #[test]
    fn test_expansion() {
        let output_expr: Expr = parse_quote! { ts.render() };
        let file_tokens: proc_macro2::TokenStream = parse_quote! {
            pub struct MyType(pub String);
        };
        let body_stmts: Vec<syn::Stmt> = parse_quote! {
            let value = import::MyType("hello".to_string());
            assert_eq!(value.0, "hello");
        };

        let expanded = expand_inner(
            "tests/output/my_type.rs",
            &output_expr,
            file_tokens,
            &body_stmts,
        );

        let wrapped: syn::File = parse_quote! { fn wrapper() { #expanded } };
        let out = prettyplease::unparse(&wrapped);
        expectorate::assert_contents("tests/output/test_expansion.rs", &out);
    }
}
