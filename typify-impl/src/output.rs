// Copyright 2024 Oxide Computer Company

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

/// A generated item from a [`crate::TypeSpace`].
#[derive(Clone, Debug)]
pub struct TypeOutputItem {
    /// The logical section where this item belongs.
    pub section: TypeOutputSection,
    /// A deterministic key used to order and group generated items.
    pub order_key: String,
    /// The generated Rust tokens for this item.
    pub tokens: TokenStream,
}

/// The logical section for a generated type output item.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeOutputSection {
    /// Error support items.
    Error,
    /// Items emitted directly in the type module.
    Crate,
    /// Struct builder items.
    Builder,
    /// Shared default helper functions.
    Defaults,
}
impl TypeOutputSection {
    /// The Rust module name for this section when emitted as separate files.
    pub fn module_name(self) -> Option<&'static str> {
        match self {
            Self::Error => Some("error"),
            Self::Crate => None,
            Self::Builder => Some("builder"),
            Self::Defaults => Some("defaults"),
        }
    }

    /// The Rust file name for this section when emitted as separate files.
    pub fn file_name(self) -> &'static str {
        match self {
            Self::Error => "error.rs",
            Self::Crate => "mod.rs",
            Self::Builder => "builder.rs",
            Self::Defaults => "defaults.rs",
        }
    }

    /// The documentation to place on this section's module declaration.
    pub fn description(self) -> Option<&'static str> {
        match self {
            Self::Error => Some("Error types."),
            Self::Crate => None,
            Self::Builder => Some("Types for composing complex structures."),
            Self::Defaults => Some("Generation of default values for serde."),
        }
    }
}

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
            .or_default()
            .extend(stream);
    }

    pub fn into_items(self) -> Vec<TypeOutputItem> {
        self.items
            .into_iter()
            .map(|((location, order_key), tokens)| TypeOutputItem {
                section: location.into(),
                order_key,
                tokens,
            })
            .collect()
    }

    pub fn into_stream(self) -> TokenStream {
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
        }
    }
}

impl From<OutputSpaceMod> for TypeOutputSection {
    fn from(value: OutputSpaceMod) -> Self {
        match value {
            OutputSpaceMod::Error => Self::Error,
            OutputSpaceMod::Crate => Self::Crate,
            OutputSpaceMod::Builder => Self::Builder,
            OutputSpaceMod::Defaults => Self::Defaults,
        }
    }
}

impl From<TypeOutputSection> for OutputSpaceMod {
    fn from(value: TypeOutputSection) -> Self {
        match value {
            TypeOutputSection::Error => Self::Error,
            TypeOutputSection::Crate => Self::Crate,
            TypeOutputSection::Builder => Self::Builder,
            TypeOutputSection::Defaults => Self::Defaults,
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
