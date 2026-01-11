use proc_macro2::TokenStream;
use quote::quote;

pub fn value_tokens(value: &serde_json::Value) -> TokenStream {
    match value {
        serde_json::Value::Null => quote! {
            ::serde_json::Value::Null
        },
        serde_json::Value::Bool(b) => todo!(),
        serde_json::Value::Number(number) => {
            if let Some(n) = number.as_i64() {
                quote! {
                    ::serde_json::Value::Number(::serde_json::Number::from(#n))
                }
            } else if let Some(n) = number.as_u64() {
                quote! {
                    ::serde_json::Value::Number(::serde_json::Number::from(#n))
                }
            } else if let Some(n) = number.as_f64() {
                quote! {
                    ::serde_json::Value::Number(::serde_json::Number::from_f64(#n))
                }
            } else {
                panic!("Invalid number")
            }
        }
        serde_json::Value::String(s) => quote! {
            ::serde_json::Value::String(#s.to_string())
        },
        serde_json::Value::Array(values) => {
            let elems = values.iter().map(value_tokens);
            quote! {
                ::serde_json::Value::Array(vec![#(#elems),*])
            }
        }
        serde_json::Value::Object(map) => {
            let entries = map.iter().map(|(k, v)| {
                let value = value_tokens(v);
                quote! {
                    (#k.to_string(), #value)
                }
            });
            quote! {
                ::serde_json::Value::Object({
                    ::serde_json::Map::from_iter([#(#entries),*])
                })
            }
        }
    }
}
