// Copyright 2024 Oxide Computer Company

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Default)]
pub struct OutputSpace {
    items: BTreeMap<(OutputSpaceMod, String), TokenStream>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputSpaceMod {
    Error,
    Crate,
    Builder,
    Defaults,
}

impl OutputSpace {
    pub fn add_item(
        &mut self,
        location: OutputSpaceMod,
        order_hint: impl ToString,
        stream: TokenStream,
    ) {
        self.items
            .entry((location, order_hint.to_string()))
            .or_insert_with(TokenStream::new)
            .extend(stream);
    }

    pub fn into_stream(self) -> TokenStream {
        let mods = self
            .items
            .into_iter()
            .map(|((location, _), item)| (location, item))
            .fold(BTreeMap::new(), |mut map, (location, item)| {
                map.entry(location)
                    .or_insert_with(TokenStream::new)
                    .extend(item);
                map
            });

        let mod_streams = mods.into_iter().map(|(location, items)| match location {
            OutputSpaceMod::Crate => quote! {
                #items
            },
            OutputSpaceMod::Builder => quote! {
                /// Types for composing complex structures.
                pub mod builder {
                    #items
                }
            },
            OutputSpaceMod::Defaults => quote! {
                /// Generation of default values for serde.
                pub mod defaults {
                    #items
                }
            },
            OutputSpaceMod::Error => quote! {
                /// Error types.
                pub mod error {
                    #items
                }
            },
        });

        quote! {
            #(#mod_streams)*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OutputSpace, OutputSpaceMod};

    use quote::quote;

    #[test]
    fn test_order() {
        let mut output = OutputSpace::default();
        output.add_item(
            OutputSpaceMod::Crate,
            "a",
            quote! {
                struct A;
            },
        );
        output.add_item(
            OutputSpaceMod::Crate,
            "b",
            quote! {
                struct B;
            },
        );
        output.add_item(
            OutputSpaceMod::Crate,
            "a",
            quote! {
                impl A {
                    fn new() -> Self { Self }
                }
            },
        );

        assert_eq!(
            output.into_stream().to_string(),
            quote! {
                struct A;
                impl A {
                    fn new() -> Self { Self }
                }
                struct B;
            }
            .to_string()
        );
    }
}
