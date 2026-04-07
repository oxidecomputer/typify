mod type_alias;
mod type_common;
mod type_enum;
mod type_native;
mod type_struct;
mod value_tokens;

use log::debug;
use serde::Deserialize;

pub use type_alias::*;
pub use type_common::*;
pub use type_enum::*;
pub use type_native::*;
pub use type_struct::*;

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet, VecDeque};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::{
    namespace::{Name, Namespace},
    schemalet::SchemaRef,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NameBuilder {
    Unset,
    Fixed(String),
    Hints(Vec<NameBuilderHint>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NameBuilderHint {
    Title(String),
    Parent(SchemaRef, String),
}

// 6/25/2025
// I think I need a builder form e.g. of an enum or struct and then the
// finalized form which probably is basically what typify shows today in its
// public interface.

// 7/11/2025
// Thinking through some options on this one. At first I really wanted this to
// be a generic interface that I might be able to use separate from typify. But
// as I got into it, it was kind of a pain in the neck, and hard to keep
// everything straight. So I decided to have it use numeric IDs for the types
// and just map to and from the SchemaRef.
//
// That also kind of sucks because I lose the context of the SchemaRef e.g. if
// I need to report errors. As much as I hate it, I think I should just embed
// SchemaRef everywhere, get all the way through it, and then figure out if I
// can clean up the boundaries.
//
// At a minimum it seems like I need several different forms of a type:
// - Builder -- used to create *de novo* types. It would seem convenient to be
//   able to express these in terms of SchemaRef only. A builder type should be
//   able to (generically) tell you its dependencies. It's not really meant for
//   user interaction beyond that.
// - Internal -- used both before and after finalization; opaque to external
//   consumers. It's where we might incrementally build the thing. (TODO and
//   probably requires a bunch more figuring out)
// - External -- for external consumers of the typify crate e.g. progenitor.
//   This should only work (probably?) for finalized types. But there might be
//   situations where we need to know a little about types before finalization.
//   Something else to consider.

// TODO 7/18/2025
// I wanted to get this started to think through various settings that we might
// eventually want...
/// Modify how types are processed and generated.
///
/// Futures:
///
/// There are traits that may require special handling during type generation:
///
/// - `serde::Serialize` and `serde::Deserialize` -- These traits depend on the
///   shape of the data and, while--as much as possible--generated code makes
///   use of the derived implementations, the serialized form of some generated
///   types may be a little different.
///
/// - `schemars::JsonSchema` -- As with serde traits, JsonSchema depends on the
///   shape of data and may be customized in some circumstances. In addition,
///   typify supports multiple version of `schemars` so additional
///   configuration may be required to specify the version or to customize the
///   crate name e.g. if one were to support multiple versions simultaneously.
///
/// - `std::clone::Clone` -- XXX
/// - `std::fmt::Display` -- XXX
/// - `std::default::Default` -- XXX
/// XXX
///
/// - Eq, Cmp and anything else that's not implemented by floating-point types.
///
/// Null vs Optional
///
/// Most of the time we want to do what serde does and not distinguish between
/// these, but some users may want to be able to adjust this both globally and
/// on a per-type basis... [8/29/2025: done]
#[derive(Debug, Default, Deserialize)]
pub struct TypespaceSettings {
    /// When set to `FullyQualified`, (the default), types in the `std` crate's
    /// prelude are fully qualified. For example, the `Option` type is rendered
    /// as `::std::option::Option`. When set to `Unqualified`, these types
    /// appear in their more typical, auto-imported form. The latter is useful
    /// if one intends to use type generation as a starting point for
    /// manually-edited code. Note that this is relevant only to types in the
    /// `std` crate's prelude such as `Option`, `Vec`, and `String`; types such
    /// as `std::collections::BTreeMap` are always fully qualified since they
    /// are not in the prelude.
    #[serde(default)]
    std: TypespaceSettingsStd,

    /// Specify the modeling of values that may be either `null` or optional
    /// (i.e. absent). The default is `ConflateAsAbsent`, which models `null`
    /// and `optional` as equivalent by using the `std::option::Option<T>` type
    /// and skipping serialization of `None` values. While imprecise, this is
    /// typical of Rust code.
    #[serde(default)]
    optional_nullable: TypespaceSettingsOptionalNullable,

    map_type: Option<()>,
    set_type: Option<()>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TypespaceSettingsStd {
    #[default]
    FullyQualified,
    Unqualified,
}

/// Specify the modeling of values that may be either 'null' or 'optional'
/// (i.e. absent).
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TypespaceSettingsOptionalNullable {
    /// Model `null` and `optional` as equivalent by using the
    /// `std::option::Option<T>` type. Skip serialization of `None` values.
    /// This is the default.
    #[default]
    ConflateAsAbsent,

    /// Model `null` and `optional` as equivalent by using the
    /// `std::option::Option<T>` type. `None` values are serialized as `null`.
    ConflateAsNull,

    /// Use a "double `Option`" of the form
    /// `std::option::Option<std::option::Option<T>>`. A `None` indicates that
    /// the value is absent; `Some(None)` indicates that the value is present
    /// and null; and `Some(Some(_))` indicates that the value is present
    /// and non-null.
    DoubleOption,

    /// Use a custom type `Opt` where `Opt: std::default::Default +
    /// serde::Deserialize + serde::Serialize`. The `Default` implementation
    /// specifies the value for a field when absent; the `Deserialize`
    /// implementation produces a value otherwise (null or a non-null value of
    /// T). In addition, `Opt` must implement `is_absent(&self) -> bool` which
    /// is used with the serde `skip_serializing_if` attribute to omit the
    /// field.
    CustomType(String),
}

/// Enumeration of traits for which Typify has particular awareness.
/// XXX write more docs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypespaceTrait {
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    FromStr,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
}

impl TypespaceTrait {
    pub(crate) fn render(&self, settings: &TypespaceSettings) -> proc_macro2::TokenStream {
        if settings.std == TypespaceSettingsStd::FullyQualified {
            match self {
                TypespaceTrait::Clone => quote! { ::std::clone::Clone },
                TypespaceTrait::Debug => quote! { ::std::fmt::Debug },
                TypespaceTrait::Serialize => quote! { ::serde::Serialize },
                TypespaceTrait::Deserialize => quote! { ::serde::Deserialize },
                TypespaceTrait::JsonSchema => quote! { ::schemars::JsonSchema },
                TypespaceTrait::Ord => quote! { ::std::cmp::Ord },
                TypespaceTrait::PartialOrd => quote! { ::std::cmp::PartialOrd },
                TypespaceTrait::Eq => quote! { ::std::cmp::Eq },
                TypespaceTrait::PartialEq => quote! { ::std::cmp::PartialEq },
                TypespaceTrait::Hash => quote! { ::std::hash::Hash },
                TypespaceTrait::Display => quote! { ::std::fmt::Display },
                TypespaceTrait::FromStr => quote! { ::std::str::FromStr },
            }
        } else {
            match self {
                TypespaceTrait::Clone => quote! { Clone },
                TypespaceTrait::Debug => quote! { Debug },
                TypespaceTrait::Serialize => quote! { ::serde::Serialize },
                TypespaceTrait::Deserialize => quote! { ::serde::Deserialize },
                TypespaceTrait::JsonSchema => quote! { ::schemars::JsonSchema },
                TypespaceTrait::Ord => quote! { Ord },
                TypespaceTrait::PartialOrd => quote! { PartialOrd },
                TypespaceTrait::Eq => quote! { Eq },
                TypespaceTrait::PartialEq => quote! { PartialEq },
                TypespaceTrait::Hash => quote! { Hash },
                TypespaceTrait::Display => quote! { Display },
                TypespaceTrait::FromStr => quote! { FromStr },
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypespaceTraitSet(BTreeSet<TypespaceTrait>);

impl FromIterator<TypespaceTrait> for TypespaceTraitSet {
    fn from_iter<T: IntoIterator<Item = TypespaceTrait>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for TypespaceTraitSet {
    type Item = TypespaceTrait;
    type IntoIter = std::collections::btree_set::IntoIter<TypespaceTrait>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TypespaceTraitSet {
    pub fn empty() -> Self {
        Self(Default::default())
    }

    pub fn contains(&self, tt: &TypespaceTrait) -> bool {
        self.0.contains(tt)
    }
    pub fn add(&mut self, tt: TypespaceTrait) {
        self.0.insert(tt);
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = &TypespaceTrait> {
        self.0.iter()
    }

    pub fn difference<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = &'a TypespaceTrait> + 'a {
        self.0.difference(&other.0)
    }
}

pub struct Typespace {
    settings: TypespaceSettings,
    types: BTreeMap<SchemaRef, Type>,
}

impl Typespace {
    pub fn render(&self) -> TokenStream {
        let types = self.types.iter().map(|(id, typ)| {
            debug!("rendering {id}");
            match typ {
                Type::Enum(type_enum) => type_enum.render(self),
                Type::Struct(type_struct) => type_struct.render(self),
                Type::UnitStruct(unit_struct_info) => unit_struct_info.render(),
                Type::TupleStruct(type_tuple_struct) => type_tuple_struct.render(self),
                Type::NewtypeStruct(newtype_info) => newtype_info.render(self),
                Type::TypeAlias(alias_info) => alias_info.render(self),

                // There's no type definition, so no code is required.
                Type::Native(_)
                | Type::Option(_)
                | Type::Box(_)
                | Type::Vec(_)
                | Type::Map(_, _)
                | Type::Set(_)
                | Type::Array(_, _)
                | Type::Tuple(_)
                | Type::Unit
                | Type::Boolean
                | Type::Integer(_)
                | Type::Float(_)
                | Type::String
                | Type::JsonValue => Default::default(),
            }
        });

        quote! {
            #( #types )*
        }
    }

    pub(crate) fn render_ident(&self, id: &SchemaRef) -> TokenStream {
        let ty = self.types.get(id).unwrap();
        match ty {
            Type::Enum(TypeEnum { common, .. })
            | Type::Struct(TypeStruct { common, .. })
            | Type::UnitStruct(TypeUnitStruct { common, .. })
            | Type::TupleStruct(TypeTupleStruct { common, .. })
            | Type::NewtypeStruct(TypeNewtypeStruct { common, .. })
            | Type::TypeAlias(TypeTypeAlias { common, .. }) => {
                let name = common.built.as_ref().unwrap().name.to_string();
                let name_ident = format_ident!("{name}");
                name_ident.into_token_stream()
            }

            Type::Native(TypeNative {
                name, parameters, ..
            }) => {
                let name_ident = syn::parse_str::<syn::TypePath>(name).unwrap();
                let parameters = (!parameters.is_empty()).then(|| {
                    let parameter_idents = parameters
                        .iter()
                        .map(|param_id| self.render_ident(param_id));
                    quote! {
                        < #( #parameter_idents ),* >
                    }
                });
                quote! {
                    #name_ident #parameters
                }
            }

            Type::Array(schema_ref, n) => {
                let inner_ident = self.render_ident(schema_ref);
                quote! {
                    [#inner_ident; #n]
                }
            }
            Type::Tuple(schema_refs) => {
                let inner_idents = schema_refs.iter().map(|id| self.render_ident(id));
                quote! {
                    ( #( #inner_idents ),* )
                }
            }

            Type::Option(option_id) => {
                let option_type = match &self.settings.std {
                    TypespaceSettingsStd::FullyQualified => quote! { ::std::option::Option },
                    TypespaceSettingsStd::Unqualified => quote! { Option },
                };
                let option_ident = self.render_ident(option_id);
                quote! {
                    #option_type<#option_ident>
                }
            }
            Type::Box(boxed_id) => {
                let box_type = match &self.settings.std {
                    TypespaceSettingsStd::FullyQualified => quote! { ::std::boxed::Box },
                    TypespaceSettingsStd::Unqualified => quote! { Box },
                };
                let boxed_ident = self.render_ident(boxed_id);
                quote! {
                    #box_type<#boxed_ident>
                }
            }
            Type::Set(inner_id) => {
                // TODO 3/25/2026
                // Replace with set type
                let vec_type = match &self.settings.std {
                    TypespaceSettingsStd::FullyQualified => quote! { ::std::vec::Vec },
                    TypespaceSettingsStd::Unqualified => quote! { Vec },
                };
                let inner_ident = self.render_ident(inner_id);
                quote! {
                    #vec_type<#inner_ident>
                }
            }
            Type::Vec(inner_id) => {
                // TODO 3/25/2026
                // Make configurable?
                let vec_type = match &self.settings.std {
                    TypespaceSettingsStd::FullyQualified => quote! { ::std::vec::Vec },
                    TypespaceSettingsStd::Unqualified => quote! { Vec },
                };
                let inner_ident = self.render_ident(inner_id);
                quote! {
                    #vec_type<#inner_ident>
                }
            }
            Type::Map(key_id, value_id) => {
                // TODO 3/25/2026
                // Configurable like typify 1
                let key_ident = self.render_ident(key_id);
                let value_ident = self.render_ident(value_id);
                quote! {
                    ::std::collections::BTreeMap<#key_ident, #value_ident>
                }
            }
            Type::Boolean => quote! { bool },
            Type::Integer(name) | Type::Float(name) => syn::parse_str::<syn::TypePath>(name)
                .unwrap()
                .to_token_stream(),
            Type::String => match &self.settings.std {
                TypespaceSettingsStd::FullyQualified => quote! { ::std::string::String },
                TypespaceSettingsStd::Unqualified => quote! { String },
            },
            Type::JsonValue => quote! { ::serde_json::Value },
            Type::Unit => quote! { () },
        }
    }

    pub(crate) fn render_struct_property(
        &self,
        StructProperty {
            rust_name,
            json_name,
            state,
            description,
            type_id,
        }: &StructProperty,
        vis_pub: bool,
    ) -> TokenStream {
        let description = description.as_ref().map(|text| {
            quote! {
                #[doc = #text]
            }
        });

        let mut serde_options = Vec::new();

        match json_name {
            StructPropertySerde::None => {}
            StructPropertySerde::Rename(s) => {
                serde_options.push(quote! {
                    rename = #s
                });
            }
            StructPropertySerde::Flatten => {
                serde_options.push(quote! {
                    #[serde(flatten)]
                });
            }
        };

        let ty = self.types.get(type_id).unwrap();

        // If the type is itself an Option (i.e. may be null), let's save the
        // alternative (i.e. non-null) type, which we may use i.e. if the field
        // may be absent and the consumer has specified a custom type for that
        // situation. In other cases, we need to know if the type is an Option--
        // even if we don't need to know the identity of the inner type.
        let maybe_option_type = if let Type::Option(id) = ty {
            Some(id)
        } else {
            None
        };

        let ty_ident = self.render_ident(type_id);

        let std_opt_type = match &self.settings.std {
            TypespaceSettingsStd::FullyQualified => quote! { ::std::option::Option },
            TypespaceSettingsStd::Unqualified => quote! { Option },
        };
        let std_opt_is_none = format!("{std_opt_type}::is_none");

        let ty_ident = match (state, maybe_option_type) {
            // A required field needs no serde annotations.
            (StructPropertyState::Required, None) => ty_ident,

            // A required field that is an Option<T> needs a custom
            // deserializer so that the field is mandatory, but may be null;
            // without this attribute, the default handling is to permit
            // either.
            (StructPropertyState::Required, Some(_)) => {
                let opt_deserialize = format!("{std_opt_type}::deserialize");
                // TODO schemars schema_with?
                serde_options.push(quote! { deserialize_with = #opt_deserialize });
                ty_ident
            }

            // An optional field that is not an Option<T> may not be null; we
            // use the json::serde::deserialize_some function to enforce this.
            (StructPropertyState::Optional, None) => {
                serde_options.push(quote! { default });
                serde_options.push(quote! {
                    deserialize_with = "::json_serde::deserialize_some"
                });
                serde_options.push(quote! { skip_serializing_if = #std_opt_is_none });
                // TODO schemars schema_with

                quote! {
                    #std_opt_type<#ty_ident>
                }
            }

            // An optional field that is also an Option<T> may be the type
            // value, null, or absent. Customizable settings determine the
            // handling of this.
            (StructPropertyState::Optional, Some(inner_id)) => {
                match &self.settings.optional_nullable {
                    TypespaceSettingsOptionalNullable::ConflateAsAbsent => {
                        serde_options.push(quote! { skip_serializing_if = #std_opt_is_none });
                        ty_ident
                    }
                    TypespaceSettingsOptionalNullable::ConflateAsNull => {
                        // We always serialize--including `None` as `null`--so
                        // no serde options are necessary.
                        ty_ident
                    }
                    TypespaceSettingsOptionalNullable::DoubleOption => {
                        serde_options.push(quote! { default });
                        serde_options.push(quote! {
                            deserialize_with = "::json_serde::deserialize_some"
                        });
                        serde_options.push(quote! {
                            skip_serializing_if = #std_opt_is_none
                        });

                        quote! {
                            #std_opt_type<#ty_ident>
                        }
                    }
                    TypespaceSettingsOptionalNullable::CustomType(custom_type_name) => {
                        let custom_type_path =
                            syn::parse_str::<syn::TypePath>(custom_type_name).unwrap();
                        serde_options.push(quote! { default });
                        let custom_is_absent = format!("{}::is_absent", custom_type_name);
                        serde_options.push(quote! { skip_serializing_if = #custom_is_absent });

                        let inner_ident = self.render_ident(inner_id);

                        quote! {
                            #custom_type_path<#inner_ident>
                        }
                    }
                }
            }
            (StructPropertyState::Default, _) => {
                serde_options.push(quote! { default });
                match ty {
                    Type::Enum(_) => todo!(),
                    Type::Struct(_) => todo!(),
                    Type::UnitStruct(_) => todo!(),
                    Type::TupleStruct(_) => todo!(),
                    Type::NewtypeStruct(_) => todo!(),
                    Type::TypeAlias(_) => todo!(),

                    Type::Native(_) => todo!(),
                    Type::Option(_schema_ref) => {
                        // This case is basically meaningless, but it's also
                        // fine. Note that #[serde(default)] is a no-op for
                        // Option<T>.
                        serde_options.push(quote! { skip_serializing_if = #std_opt_is_none });
                    }
                    Type::Box(_schema_ref) => todo!(),

                    Type::Vec(_) | Type::Map(_, _) | Type::Set(_) | Type::String => {
                        let is_empty = format!("{ty_ident}::is_empty");
                        serde_options.push(quote! { skip_serializing_if = #is_empty });
                    }

                    Type::Array(_schema_ref, _) => todo!(),
                    Type::Tuple(_schema_refs) => todo!(),
                    Type::Unit => {
                        // This is a weird one
                        todo!()
                    }
                    Type::Boolean => todo!(),
                    Type::Integer(_) => todo!(),
                    Type::Float(_) => todo!(),
                    Type::JsonValue => todo!(),
                }
                ty_ident
            }
            (StructPropertyState::DefaultValue(_json_value), _) => {
                // TODO 1/12/2026
                // We want to construct a value by walking both the type of the
                // property and the value in tandem.
                // Alternatively we could emit the JSON value as a string or as
                // serde_json::Value structure and then deserialize from it.

                serde_options.push(quote! { default = "xxx" });
                ty_ident
            }
        };

        let serde = (!serde_options.is_empty()).then(|| {
            quote! {
                #[serde(
                    #( #serde_options ),*
                )]
            }
        });
        let vis_pub = vis_pub.then(|| quote! { pub });

        quote! {
            #description
            #serde
            #vis_pub #rust_name: #ty_ident
        }
    }
}

pub struct TypespaceBuilder {
    types: BTreeMap<SchemaRef, Type>,
}

impl Default for TypespaceBuilder {
    fn default() -> Self {
        Self {
            types: Default::default(),
        }
    }
}

impl TypespaceBuilder {
    pub fn insert(&mut self, id: SchemaRef, typ: Type) {
        match self.types.entry(id) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(typ);
            }
            Entry::Occupied(occupied_entry) => {
                // TODO 1/12/2026
                // Not fully sure what I intended for this case. It seems like
                // if we're getting duplicate entries ... something is way off.
                let _key = occupied_entry.key();

                println!("existing {:#?}", occupied_entry.get());
                println!("new {:#?}", typ);

                todo!("{}", occupied_entry.key());
            }
        }
    }

    pub fn contains_type(&self, id: &SchemaRef) -> bool {
        self.types.contains_key(id)
    }

    pub fn finalize(self, settings: TypespaceSettings) -> Result<Typespace, ()> {
        // Basic steps:
        // 1. Construct the parent and child adjacency lists
        // 2. Figure out names for all types that need them
        // 3. Break containment cycles with Box types
        // 4. Propagate trait impls
        // 5. Type-specific finalization

        let Self { mut types } = self;

        // TODO 7/2/2025
        // It's all graphs. Think about everything as a graph traversal.

        let id_to_children = types
            .iter()
            .map(|(id, typ)| (id, typ.children()))
            .collect::<BTreeMap<_, _>>();

        // Build forward and backward adjacency lists.
        let mut id_to_parents = BTreeMap::<_, Vec<_>>::new();

        for (id, children) in &id_to_children {
            for child_id in children {
                // Ensure that all referenced types exist
                assert!(types.contains_key(child_id));

                id_to_parents.entry(child_id.clone()).or_default().push(id);
            }
        }

        // Figure out names for the types that need names.
        let mut work = VecDeque::new();

        for (id, typ) in &types {
            // If it's not a named type, continue
            if !typ.is_named() {
                continue;
            }

            for (child_id, child_sigil) in typ.children_with_context() {
                work.push_back((id.clone(), child_id, child_sigil))
            }
        }

        let mut name_hints = BTreeMap::<_, Vec<NameBuilderHint>>::new();

        println!("NAMING");
        while let Some((parent_id, child_id, child_sigil)) = work.pop_front() {
            println!("parent: {parent_id} child: {child_id} child_sigil: {child_sigil}");
            let child_typ = types.get(&child_id).unwrap();
            println!("child named? {}", child_typ.is_named());

            if child_typ.is_named() {
                name_hints
                    .entry(child_id)
                    .or_default()
                    .push(NameBuilderHint::Parent(parent_id.clone(), child_sigil));
            } else {
                for (grandchild_id, grandchild_sigil) in child_typ.children_with_context() {
                    work.push_back((
                        parent_id.clone(),
                        grandchild_id,
                        format!("{child_sigil}-=-{grandchild_sigil}"),
                    ))
                }
            }
        }

        println!("HINTS {:#?}", name_hints);

        types.iter_mut().for_each(|(id, typ)| {
            if let Some(hints) = name_hints.remove(id) {
                typ.add_name_hints(hints);
                println!("{} {:#?}", id, typ);
            }
        });

        let mut namespace = Namespace::<SchemaRef>::default();

        println!("NAMES");
        for (id, typ) in &mut types {
            let is_named = typ.is_named();
            if let Some(name) = typ.get_name_mut() {
                println!("{id} is_named: {}", is_named);
                println!("{:#?}", name);
                println!("{:#?}", typ.children_with_context());
                println!();
            }
        }

        for (id, typ) in &mut types {
            debug!("naming id {id}");
            if let Some(common) = typ.get_common_mut() {
                let name = match &common.name {
                    NameBuilder::Unset => unreachable!(),
                    NameBuilder::Fixed(s) => {
                        let nn = namespace.make_name(id.clone());
                        nn.set_name(s);
                        nn
                    }
                    NameBuilder::Hints(hints) => {
                        let nn = namespace.make_name(id.clone());

                        for hint in hints {
                            match hint {
                                NameBuilderHint::Title(_) => todo!(),
                                NameBuilderHint::Parent(id, s) => {
                                    nn.derive_name(id, s);
                                }
                            }
                        }
                        nn
                    }
                };
                common.built = Some(TypeCommonBuilt {
                    name,
                    traits: TypespaceTraitSet::empty(),
                });
            }

            println!("{:#?}", typ);
        }

        // TODO 7/1/2025
        // Let's do names first.

        // TODO Make sure that all referenced schemas are present.
        // TODO break cycles
        // TODO resolve names
        // TODO propagate trait impls

        // TODO 11/13/2025
        // Update: we've done names, checked references, and broken cycles.
        // Still need to propagate traits.

        let _n2 = namespace.finalize().unwrap();
        break_cycles(&mut types);

        push_traits(&mut types)?;

        Ok(Typespace { settings, types })
    }
}

fn break_cycles(types: &mut BTreeMap<SchemaRef, Type>) {
    enum Node {
        Start {
            type_id: SchemaRef,
        },
        Processing {
            type_id: SchemaRef,
            children_ids: Vec<SchemaRef>,
        },
    }

    let mut visited = BTreeSet::<SchemaRef>::new();

    let type_ids = types.keys().cloned().collect::<Vec<_>>();

    for type_id in type_ids {
        if visited.contains(&type_id) {
            continue;
        }

        let mut active = BTreeSet::<SchemaRef>::new();
        let mut stack = Vec::<Node>::new();

        active.insert(type_id.clone());
        stack.push(Node::Start { type_id });

        while let Some(top) = stack.last_mut() {
            match top {
                // Skip right to the end since we've already seen this type.
                Node::Start { type_id } if visited.contains(type_id) => {
                    assert!(active.contains(type_id));

                    let type_id = type_id.clone();
                    *top = Node::Processing {
                        type_id,
                        children_ids: Vec::new(),
                    };
                }

                // Break any immediate cycles and queue up this type for
                // descent into its child types.
                Node::Start { type_id } => {
                    assert!(active.contains(type_id));

                    visited.insert(type_id.clone());

                    // Determine which child types form cycles--and
                    // therefore need to be snipped--and the rest--into
                    // which we should descend. We make this its own block
                    // to clarify the lifetime of the exclusive reference
                    // to the type. We don't really *need* to have an
                    // exclusive reference here, but there's no point in
                    // writing `get_child_ids` again for shared references.
                    let (snip, descend) = {
                        let typ = types.get_mut(type_id).unwrap();

                        let child_ids = typ
                            .contained_children_mut()
                            .into_iter()
                            .map(|child_id| child_id.clone());

                        // If the child type is in active then we've found
                        // a cycle (otherwise we'll descend).
                        child_ids.partition::<Vec<_>, _>(|child_id| active.contains(child_id))
                    };

                    // Note that while `snip` might contain duplicates,
                    // `id_to_box` is idempotent insofar as the same input
                    // TypeId will result in the same output TypeId. Ergo
                    // the resulting pairs from which we construct the
                    // mapping would contain exact duplicates; it would not
                    // contain two values associated with the same key.
                    let replace = snip
                        .into_iter()
                        .map(|type_id| {
                            let box_id = SchemaRef::Box(Box::new(type_id.clone()));
                            let box_typ = Type::Box(type_id.clone());
                            types.insert(box_id.clone(), box_typ);

                            (type_id, box_id)
                        })
                        .collect::<BTreeMap<SchemaRef, SchemaRef>>();

                    // Break any cycles by reassigning the child type to a box.
                    let typ = types.get_mut(type_id).unwrap();

                    let child_ids = typ.contained_children_mut();
                    // let type_entry = self.id_to_entry.get_mut(type_id).unwrap();
                    // let child_ids = get_child_ids(type_entry);
                    for child_id in child_ids {
                        if let Some(replace_id) = replace.get(child_id) {
                            *child_id = replace_id.clone();
                        }
                    }

                    // Descend into child types.
                    let node = Node::Processing {
                        type_id: type_id.clone(),
                        children_ids: descend,
                    };
                    *top = node;
                }

                // If there are children left, push the next child onto the
                // stack. If there are none left, pop this type.
                Node::Processing {
                    type_id,
                    children_ids,
                } => {
                    if let Some(type_id) = children_ids.pop() {
                        // Descend into the next child node.
                        active.insert(type_id.clone());
                        stack.push(Node::Start { type_id });
                    } else {
                        // All done; remove the item from the active list
                        // and stack.
                        active.remove(type_id);
                        let _ = stack.pop();
                    }
                }
            }
        }
    }
}

fn push_traits(types: &mut BTreeMap<SchemaRef, Type>) -> Result<(), ()> {
    // First, look through all types to determine what traits are required of
    // various children.
    let mut work = types
        .iter()
        .filter_map(|(_, ty)| match ty {
            // TODO 3/31/2026
            // need to check map settings
            Type::Map(key_schema_ref, _) => Some((
                key_schema_ref.clone(),
                [
                    TypespaceTrait::Eq,
                    TypespaceTrait::PartialEq,
                    TypespaceTrait::Ord,
                    TypespaceTrait::PartialOrd,
                ]
                .into_iter()
                .collect::<TypespaceTraitSet>(),
            )),
            // TODO 3/31/2026
            // This is going to depend on what specific type we're using for a
            // set.
            // Type::Set(_) => todo!(),
            _ => None,
        })
        .collect::<VecDeque<_>>();

    // In each iteration, we need to assert the set of required traits to the
    // current type. If the current type is generated, that means adding the
    // traits and pushing children. If the type is **not** generated (native or
    // otherwise external to our control), we need to check that is implements
    // (or is capable of implementing) the required traits; if it doesn't (or
    // can't), we'll produce an error. We don't stop on the first failure, but
    // want to identify as many, distinct failures as is reasonable and as
    // would be useful for a consumer.
    while let Some((schema_ref, traits)) = work.pop_front() {
        let ty = types.get_mut(&schema_ref).unwrap();

        let common_built = match ty {
            Type::NewtypeStruct(TypeNewtypeStruct { common, .. })
            | Type::Enum(TypeEnum { common, .. })
            | Type::Struct(TypeStruct { common, .. }) => Some(common.built.as_mut().unwrap()),
            Type::UnitStruct(_) => todo!(),
            Type::TupleStruct(_) => todo!(),
            Type::TypeAlias(_) => todo!(),

            _ => None,
        };

        if let Some(common) = common_built {
            let built_traits = &mut common.traits;
            // Collect the traits that this type doesn't already have.
            let mut new_traits = TypespaceTraitSet::empty();

            for trait_name in traits {
                if !built_traits.contains(&trait_name) {
                    built_traits.add(trait_name);
                    new_traits.add(trait_name);
                }
            }

            if !new_traits.is_empty() {
                for child_id in ty.contained_children_mut() {
                    work.push_back((child_id.clone(), new_traits.clone()));
                }
            }
        } else {
            match ty {
                Type::Enum(_)
                | Type::Struct(_)
                | Type::UnitStruct(_)
                | Type::TupleStruct(_)
                | Type::NewtypeStruct(_)
                | Type::TypeAlias(_) => unreachable!(),

                Type::Native(TypeNative { name, impls, .. }) => {
                    let missing_traits = traits
                        .difference(impls)
                        .cloned()
                        .collect::<TypespaceTraitSet>();
                    if !missing_traits.is_empty() {
                        todo!(
                            "missing traits {:#?} for native type {name}",
                            missing_traits,
                        );
                    }
                }

                // Pass the buck...
                Type::Option(schema_ref) | Type::Box(schema_ref) => {
                    work.push_back((schema_ref.clone(), traits));
                }

                // Vec<T> and arrays impl everything we care about--except for
                // Display and FromStr--as long as T implemented them.
                Type::Vec(schema_ref) | Type::Array(schema_ref, _) => {
                    if traits.contains(&TypespaceTrait::Display)
                        || traits.contains(&TypespaceTrait::FromStr)
                    {
                        todo!();
                    }
                    work.push_back((schema_ref.clone(), traits));
                }
                // Tuples implement everything except for Display and FromStr
                // as long as all their component types do as well.
                Type::Tuple(schema_refs) => {
                    if traits.contains(&TypespaceTrait::Display)
                        || traits.contains(&TypespaceTrait::FromStr)
                    {
                        todo!();
                    }
                    for schema_ref in schema_refs {
                        work.push_back((schema_ref.clone(), traits.clone()));
                    }
                }

                Type::Map(_, _) | Type::Set(_) => todo!("wtf {schema_ref} {:#?}", ty),

                // TODO 3/31/2026
                // Comment and do better
                Type::Float(_) => {
                    if traits.contains(&TypespaceTrait::Ord)
                        || traits.contains(&TypespaceTrait::Eq)
                        || traits.contains(&TypespaceTrait::Hash)
                    {
                        todo!();
                    }
                }

                // These all implement all the traits we care about so there's
                // nothing to do.
                Type::Unit | Type::Boolean | Type::Integer(_) | Type::String => (),

                // JsonValue implements everything except for Eq, Ord,
                // PartialOrd, and Hash.
                Type::JsonValue => {
                    if traits.contains(&TypespaceTrait::Eq)
                        || traits.contains(&TypespaceTrait::Ord)
                        || traits.contains(&TypespaceTrait::PartialOrd)
                        || traits.contains(&TypespaceTrait::Hash)
                    {
                        todo!();
                    }
                }
            }
        }
    }

    Ok(())
}

/// Represents a type in the Typespace.
#[derive(Debug, Clone)]
pub enum Type {
    Enum(TypeEnum),
    Struct(TypeStruct),
    UnitStruct(TypeUnitStruct),
    TupleStruct(TypeTupleStruct),

    NewtypeStruct(TypeNewtypeStruct),
    TypeAlias(TypeTypeAlias),

    Native(TypeNative),
    Option(SchemaRef),

    Box(SchemaRef),
    Vec(SchemaRef),
    Map(SchemaRef, SchemaRef),
    Set(SchemaRef),
    Array(SchemaRef, usize),
    Tuple(Vec<SchemaRef>),
    Unit,
    Boolean,
    /// Integers
    Integer(String),
    /// Floating point numbers; not Eq, Ord, or Hash
    Float(String),
    /// Strings... which we handle a little specially.
    String,
    /// serde_json::Value which we also handle specially.
    JsonValue,
}

/// Common properties of named, generated types.
#[derive(Debug, Clone)]
pub struct TypeCommon {
    pub name: NameBuilder,
    pub description: Option<String>,
    pub default: Option<JsonValue>,

    pub(crate) built: Option<TypeCommonBuilt>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypeCommonBuilt {
    pub name: Name<SchemaRef>,

    // TODO 3/25/2026
    // This definitely needs more consideration after I start feeling it out.
    pub traits: TypespaceTraitSet,
}

// 9.15.2025
// Little bit of a random thought: "Native" is actually kind of a catch-all for
// which things like boolean, integer, unit, etc. could apply. I think we'll
// eventually want more of a builder interface to construct types and and then
// a finished interface to inspect them. I could imagine--for example--
// "native" being used for any non-constructed type (so anything except for
// generated structs, generated enums, and compound types such as tuples and
// arrays). Could these also have type parameters and therefore be inclusive of
// maps and vecs? Maybe? Something to noodle on as we think about Typespace as
// an interface.

impl Type {
    fn add_name_hints(&mut self, hints: Vec<NameBuilderHint>) {
        if let Some(name) = self.get_name_mut() {
            match name {
                NameBuilder::Unset => *name = NameBuilder::Hints(hints),
                NameBuilder::Fixed(_) => {}
                NameBuilder::Hints(_) => unreachable!(),
            }
        }
    }

    fn get_common_mut(&mut self) -> Option<&mut TypeCommon> {
        match self {
            Type::Enum(TypeEnum { common, .. })
            | Type::Struct(TypeStruct { common, .. })
            | Type::UnitStruct(TypeUnitStruct { common, .. })
            | Type::TupleStruct(TypeTupleStruct { common, .. })
            | Type::NewtypeStruct(TypeNewtypeStruct { common, .. })
            | Type::TypeAlias(TypeTypeAlias { common, .. }) => Some(common),
            _ => None,
        }
    }

    fn get_name_mut(&mut self) -> Option<&mut NameBuilder> {
        self.get_common_mut().map(|common| &mut common.name)
    }

    pub fn is_named(&self) -> bool {
        match self {
            Type::Enum(_) => true,
            Type::Struct(_) => true,
            Type::UnitStruct(_) => true,
            Type::TupleStruct(_) => true,
            Type::NewtypeStruct(_) => true,
            _ => false,
        }
    }

    pub fn children(&self) -> Vec<SchemaRef> {
        match self {
            Type::Enum(type_enum) => type_enum.children(),
            Type::Struct(type_struct) => type_struct.children(),
            Type::UnitStruct(_) => Vec::new(),
            Type::TupleStruct(type_tuple_struct) => type_tuple_struct.children(),
            Type::NewtypeStruct(type_newtype_struct) => type_newtype_struct.children(),
            Type::TypeAlias(alias_info) => alias_info.children(),

            Type::Boolean => Vec::new(),
            Type::String => Vec::new(),
            Type::Native(_) => Vec::new(),

            Type::Option(id)
            | Type::Box(id)
            | Type::Vec(id)
            | Type::Set(id)
            | Type::Array(id, _) => vec![id.clone()],

            Type::Map(key_id, value_id) => vec![key_id.clone(), value_id.clone()],
            Type::Tuple(items) => items.clone(),

            Type::Unit => Vec::new(),
            Type::Integer(_) => Vec::new(),
            Type::Float(_) => Vec::new(),
            Type::JsonValue => Vec::new(),
        }
    }

    fn children_with_context(&self) -> Vec<(SchemaRef, String)> {
        match self {
            Type::Enum(type_enum) => type_enum.children_with_context(),
            Type::Struct(type_struct) => type_struct.children_with_context(),
            Type::UnitStruct(_) => Vec::new(),
            Type::TupleStruct(type_tuple_struct) => type_tuple_struct.children_with_context(),
            Type::NewtypeStruct(type_newtype_struct) => type_newtype_struct.children_with_context(),
            Type::TypeAlias(alias_info) => alias_info.children_with_context(),

            Type::Native(_) => Vec::new(),
            Type::Option(id) => vec![(id.clone(), "".to_string())],
            Type::Box(_) => todo!(),
            Type::Vec(id) => vec![(id.clone(), "item".to_string())],
            Type::Map(key_id, value_id) => vec![
                (key_id.clone(), "key".to_string()),
                (value_id.clone(), "value".to_string()),
            ],
            Type::Set(id) => vec![(id.clone(), "item".to_string())],
            Type::Array(_, _) => todo!(),
            Type::Tuple(_items) => todo!(),

            Type::Unit => Vec::new(),
            Type::Boolean => Vec::new(),
            Type::Integer(_) => Vec::new(),
            Type::Float(_) => Vec::new(),
            Type::String => Vec::new(),
            Type::JsonValue => Vec::new(),
        }
    }

    /// Return the list of child types that are contained (i.e. contributed to
    /// the size of this type). This is used to consider containment cycles.
    pub fn contained_children_mut(&mut self) -> Vec<&mut SchemaRef> {
        match self {
            Type::Enum(TypeEnum { variants, .. }) => {
                let mut out = Vec::new();
                for variant in variants {
                    match &mut variant.details {
                        VariantDetails::Unit => {}
                        VariantDetails::Item(schema_ref) => {
                            out.push(schema_ref);
                        }
                        VariantDetails::Tuple(schema_refs) => {
                            out.extend(schema_refs);
                        }
                        VariantDetails::Struct(props) => {
                            for StructProperty { type_id, .. } in props {
                                out.push(type_id);
                            }
                        }
                    }
                }
                out
            }
            Type::Struct(TypeStruct { properties, .. }) => properties
                .iter_mut()
                .map(|prop| &mut prop.type_id)
                .collect(),

            Type::UnitStruct(_) => vec![],
            Type::TupleStruct(type_tuple_struct) => type_tuple_struct.contained_children_mut(),
            Type::NewtypeStruct(type_newtype_struct) => {
                type_newtype_struct.contained_children_mut()
            }

            // 2/4/2026
            // This is an interesting case. Let's say I have something like
            // this:
            // struct Foo{ foo: OptionString }
            // where OptionString is a type alias for Option<String>.
            // I guess we just want to return the target type... but we'll want
            // to make sure that doesn't turn this into an alias to a Box...
            // somehow?
            Type::TypeAlias(alias_info) => {
                vec![&mut alias_info.target]
            }

            Type::Option(id) => vec![id],
            Type::Array(id, _) => vec![id],
            Type::Tuple(items) => items.iter_mut().collect(),

            // TODO maybe native types could have children? Right now these are
            // just for self-contained types...
            Type::Native(_) => Default::default(),
            Type::Box(_)
            | Type::Vec(_)
            | Type::Map(_, _)
            | Type::Set(_)
            | Type::Unit
            | Type::Boolean
            | Type::Integer(_)
            | Type::Float(_)
            | Type::String
            | Type::JsonValue => Default::default(),
        }
    }
}

// TODO 8/29/2025
// These are integration tests; maybe they need to live elsewhere. Better
// written than not.
#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::parse_quote;

    use crate::{
        schemalet::SchemaRef,
        typespace::{
            JsonValue, NameBuilder, StructProperty, StructPropertySerde, StructPropertyState, Type,
            TypeStruct, TypeTupleStruct, TypespaceBuilder, TypespaceSettings, TypespaceSettingsStd,
        },
    };

    #[test]
    fn test_struct_field_serde() {
        let tests = [
            (
                "ConflatedAsAbsent",
                TypespaceSettings {
                    std: TypespaceSettingsStd::Unqualified,
                    optional_nullable:
                        crate::typespace::TypespaceSettingsOptionalNullable::ConflateAsAbsent,
                    ..Default::default()
                },
            ),
            (
                "ConflatedAsNull",
                TypespaceSettings {
                    std: TypespaceSettingsStd::Unqualified,
                    optional_nullable:
                        crate::typespace::TypespaceSettingsOptionalNullable::ConflateAsNull,
                    ..Default::default()
                },
            ),
            (
                "DoubleOption",
                TypespaceSettings {
                    std: TypespaceSettingsStd::Unqualified,
                    optional_nullable:
                        crate::typespace::TypespaceSettingsOptionalNullable::DoubleOption,
                    ..Default::default()
                },
            ),
            (
                "CustomType",
                TypespaceSettings {
                    std: TypespaceSettingsStd::Unqualified,
                    optional_nullable:
                        crate::typespace::TypespaceSettingsOptionalNullable::CustomType(
                            "OptionField".to_string(),
                        ),
                    ..Default::default()
                },
            ),
        ];

        // For each configuration we create a type with the following fields:
        // - optional_string: A string that may be absent
        // - required_option: Either a string or null, but must be present
        // - optional_option: A string or null, or absent
        // - default_string: A string with the intrinsic default (i.e. "")
        // - default_option: A string or null with the intrinsic default (i.e. null)
        // - peanut_string: A string with a custom default of "peanuts"
        // - peanut_option: A string or null with a custom default of "peanuts"
        let test_output = tests.into_iter().map(|(name, settings)| {
            let mut ts = TypespaceBuilder::default();

            let string_id = SchemaRef::Id("string type".to_string());
            let ty = Type::String;
            ts.insert(string_id.clone(), ty);

            let option_id = SchemaRef::Id("option type".to_string());
            let ty = Type::Option(string_id.clone());
            ts.insert(option_id.clone(), ty);

            let properties = vec![
                StructProperty {
                    rust_name: format_ident!("optional_string"),
                    json_name: StructPropertySerde::None,
                    state: StructPropertyState::Optional,
                    description: None,
                    type_id: string_id.clone(),
                },
                StructProperty {
                    rust_name: format_ident!("required_option"),
                    json_name: StructPropertySerde::None,
                    state: StructPropertyState::Required,
                    description: None,
                    type_id: option_id.clone(),
                },
                StructProperty {
                    rust_name: format_ident!("optional_option"),
                    json_name: StructPropertySerde::None,
                    state: StructPropertyState::Optional,
                    description: None,
                    type_id: option_id.clone(),
                },
                StructProperty {
                    rust_name: format_ident!("default_string"),
                    json_name: StructPropertySerde::None,
                    state: StructPropertyState::Default,
                    description: None,
                    type_id: string_id.clone(),
                },
                StructProperty {
                    rust_name: format_ident!("default_option"),
                    json_name: StructPropertySerde::None,
                    state: StructPropertyState::Default,
                    description: None,
                    type_id: option_id.clone(),
                },
                StructProperty {
                    rust_name: format_ident!("peanut_string"),
                    json_name: StructPropertySerde::None,
                    state: StructPropertyState::DefaultValue(JsonValue(serde_json::json!(
                        "peanuts"
                    ))),
                    description: None,
                    type_id: string_id.clone(),
                },
                StructProperty {
                    rust_name: format_ident!("peanut_option"),
                    json_name: StructPropertySerde::None,
                    state: StructPropertyState::DefaultValue(JsonValue(serde_json::json!(
                        "peanuts"
                    ))),
                    description: None,
                    type_id: option_id.clone(),
                },
            ];

            let ty = Type::Struct(TypeStruct::new(
                NameBuilder::Fixed(name.to_string()),
                None,
                None,
                properties,
                false,
            ));

            ts.insert(SchemaRef::Id("X".to_string()), ty);

            let ts = ts.finalize(settings).unwrap();

            ts.render()
        });

        let file = parse_quote! {
            #( #test_output )*
        };
        let out = prettyplease::unparse(&file);

        expectorate::assert_contents("tests/output/test_struct_field_serde.rs", &out);
    }

    /// I don't really like the term cursed, but I think it apply. Hold your
    /// nose. Here we're taking the path and contents. If the file doesn't
    /// exist, the build will fail at the `include!()`, so step 1: make sure
    /// the file exists. If the file exist but doesn't match the expected
    /// contents we'll let `expectorate` do its overwrite thing... but then
    /// we'll fail on an explicit panic. So only if the file matches the
    /// expected contents and doesn't need to be updated do we proceed.
    macro_rules! check_and_include {
        ($path:expr, $out:expr) => {
            if my_assert_contents($path, &$out) {
                panic!("fixture updated; run tests again");
            }

            mod import {
                include!(concat!("../../", $path));
            }
        };
    }

    /// Return true if the file was updated.
    #[track_caller]
    fn my_assert_contents<P: AsRef<std::path::Path>>(path: P, actual: &str) -> bool {
        let path = path.as_ref();
        let a2 = newline_converter::dos2unix(actual);

        let ret = match std::fs::read_to_string(path) {
            Ok(s) if s == a2 => false,
            _ => true,
        };

        expectorate::assert_contents(path, actual);

        ret
    }

    #[test]
    fn test_unit_struct() {
        let mut ts = TypespaceBuilder::default();

        let ty = Type::UnitStruct(crate::typespace::TypeUnitStruct::new(
            NameBuilder::Fixed("MyUnitStruct".to_string()),
            None,
            serde_json::json!("<<+>>"),
        ));

        ts.insert(SchemaRef::Id("MyUnitStruct".to_string()), ty);

        let ts = ts
            .finalize(TypespaceSettings::default())
            .expect("finalize typespace");

        let output = ts.render();

        let file = parse_quote! {
            #output
        };
        let out = prettyplease::unparse(&file);

        // Write out what we just generated... and include it too? Yeah, weird.
        check_and_include!("tests/output/test_unit_struct.rs", out);

        // Test serialization.
        let value = import::MyUnitStruct;
        assert_eq!(serde_json::to_string(&value).unwrap(), "\"<<+>>\"");

        // Test deserialization.
        assert!(serde_json::from_str::<import::MyUnitStruct>("\"<<+>>\"").is_ok());
        assert!(serde_json::from_str::<import::MyUnitStruct>("null").is_err());
    }

    #[test]
    fn test_tuple_struct() {
        let mut ts = TypespaceBuilder::default();

        let int_id = SchemaRef::Id("integer type".to_string());
        let ty = Type::Integer("u32".to_string());
        ts.insert(int_id.clone(), ty);

        let string_id = SchemaRef::Id("string type".to_string());
        let ty = Type::String;
        ts.insert(string_id.clone(), ty);

        let string_array_id = SchemaRef::Id("string array type".to_string());
        let ty = Type::Vec(string_id.clone());
        ts.insert(string_array_id.clone(), ty);

        let ty = Type::TupleStruct(TypeTupleStruct::new(
            NameBuilder::Fixed("MyTupleStruct".to_string()),
            None,
            vec![string_id.clone(), int_id.clone()],
            Some(string_array_id.clone()),
        ));
        ts.insert(SchemaRef::Id("MyTupleStruct".to_string()), ty);

        let ts = ts
            .finalize(TypespaceSettings::default())
            .expect("finalize typespace");

        let output = ts.render();
        let file = parse_quote! {
            #output
        };
        let out = prettyplease::unparse(&file);
        println!("{}", out);

        // Write out what we just generated... and include it too? Yeah, weird.
        check_and_include!("tests/output/test_tuple_struct.rs", out);

        // Test serialization.
        let value = import::MyTupleStruct("hello".to_string(), 42, vec![]);
        assert_eq!(serde_json::to_string(&value).unwrap(), r#"["hello",42]"#);

        let value = import::MyTupleStruct(
            "hello".to_string(),
            42,
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            r#"["hello",42,"a","b","c"]"#
        );

        // Test deserialization.
        let value = serde_json::from_str::<import::MyTupleStruct>(r#"["hello",42]"#).unwrap();
        assert_eq!(value.0, "hello");
        assert_eq!(value.1, 42);
        assert!(value.2.is_empty());

        let value =
            serde_json::from_str::<import::MyTupleStruct>(r#"["hello",42,"a","b"]"#).unwrap();
        assert_eq!(value.0, "hello");
        assert_eq!(value.1, 42);
        assert!(value.2.len() == 2);
        assert_eq!(value.2[0], "a");
        assert_eq!(value.2[1], "b");

        let result = serde_json::from_str::<import::MyTupleStruct>(r#"[]"#);
        let e = result.unwrap_err().to_string();
        assert!(e.contains("invalid length 0"), "{e}");

        let result = serde_json::from_str::<import::MyTupleStruct>(r#"["hello"]"#);
        let e = result.unwrap_err().to_string();
        assert!(e.contains("invalid length 1"), "{e}");

        let result = serde_json::from_str::<import::MyTupleStruct>(r#"["a",1,2]"#);
        let e = result.unwrap_err().to_string();
        assert!(e.contains("invalid type: integer"), "{e}");
    }

    // A map whose key is a struct containing a float cannot satisfy the Ord
    // and Eq constraints required of BTreeMap keys. This should return Err.
    #[test]
    fn test_map_key_struct_with_float() {
        let mut ts = TypespaceBuilder::default();

        let float_id = SchemaRef::Id("float".to_string());
        ts.insert(float_id.clone(), Type::Float("f64".to_string()));

        let key_id = SchemaRef::Id("key".to_string());
        ts.insert(
            key_id.clone(),
            Type::Struct(TypeStruct::new(
                NameBuilder::Fixed("Key".to_string()),
                None,
                None,
                vec![StructProperty {
                    rust_name: format_ident!("value"),
                    json_name: StructPropertySerde::None,
                    state: StructPropertyState::Required,
                    description: None,
                    type_id: float_id.clone(),
                }],
                false,
            )),
        );

        let value_id = SchemaRef::Id("value".to_string());
        ts.insert(value_id.clone(), Type::String);

        let map_id = SchemaRef::Id("map".to_string());
        ts.insert(map_id, Type::Map(key_id, value_id));

        ts.finalize(TypespaceSettings::default()).unwrap();
    }
}
