// Copyright 2024 Oxide Computer Company

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Default)]
pub struct OutputSpace {
    items: BTreeMap<(OutputSpaceMod, String), TokenStream>,
    /// Set when at least one `double_option` property has been emitted, so the
    /// shared `double_option` deserialize helper module is generated exactly
    /// once (regardless of how many properties reference it).
    needs_double_option: bool,
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
            .or_default()
            .extend(stream);
    }

    /// Record that the generated code needs the `double_option` deserialize
    /// helper module (see [`OutputSpace::into_stream`]).
    pub fn require_double_option(&mut self) {
        self.needs_double_option = true;
    }

    pub fn into_stream(self) -> TokenStream {
        // Emitted (once) when any property uses the `double_option` setting.
        // The `default` + `skip_serializing_if` attributes on the field already
        // serialize the three states correctly; this helper is only needed on
        // deserialize, where stock serde would otherwise collapse a present
        // `null` into the outer `None`. Wrapping the inner `Option<T>` in `Some`
        // recovers `Some(None)` for an explicit `null`.
        let double_option = self.needs_double_option.then(|| {
            quote! {
                /// Deserialization helper for `Option<Option<T>>` fields that
                /// must distinguish an absent field from an explicit `null`.
                pub mod double_option {
                    pub fn deserialize<'de, T, D>(
                        deserializer: D,
                    ) -> ::std::result::Result<::std::option::Option<::std::option::Option<T>>, D::Error>
                    where
                        T: ::serde::Deserialize<'de>,
                        D: ::serde::Deserializer<'de>,
                    {
                        ::serde::Deserialize::deserialize(deserializer).map(::std::option::Option::Some)
                    }
                }
            }
        });

        let mods = self
            .items
            .into_iter()
            .map(|((location, _), item)| (location, item))
            .fold(
                BTreeMap::<_, TokenStream>::new(),
                |mut map, (location, item)| {
                    map.entry(location).or_default().extend(item);
                    map
                },
            );

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
            #double_option
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
