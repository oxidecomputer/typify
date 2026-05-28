mod type_alias;
mod type_common;
mod type_enum;
mod type_native;
mod type_struct;
pub(crate) mod value_tokens;

pub use type_alias::*;
pub use type_common::*;
pub use type_enum::*;
pub use type_native::*;
pub use type_struct::*;

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet, VecDeque};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;

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
    // map_type: Option<()>,
    // set_type: Option<()>,
}

impl TypespaceSettings {
    pub fn with_std(mut self, std: TypespaceSettingsStd) -> Self {
        self.std = std;
        self
    }

    pub fn with_optional_nullable(
        mut self,
        optional_nullable: TypespaceSettingsOptionalNullable,
    ) -> Self {
        self.optional_nullable = optional_nullable;
        self
    }
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
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

/// Represents a type in the Typespace.
#[derive(Debug, Clone)]
pub enum Type<Id> {
    Enum(TypeEnum<Id>),
    Struct(TypeStruct<Id>),
    UnitStruct(TypeUnitStruct),
    TupleStruct(TypeTupleStruct<Id>),
    NewtypeStruct(TypeNewtypeStruct<Id>),
    TypeAlias(TypeTypeAlias<Id>),

    Native(TypeNative<Id>),
    Option(Id),
    Box(Id),
    Vec(Id),
    Map(Id, Id),
    Set(Id),
    Array(Id, usize),
    Tuple(Vec<Id>),
    Unit,
    Boolean,
    Integer(String),
    Float(String),
    String,
    JsonValue,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> Type<Id> {
    // fn get_common_mut(&mut self) -> Option<&mut TypeCommon> {
    //     match self {
    //         Type::Enum(TypeEnum { common, .. })
    //         | Type::Struct(TypeStruct { common, .. })
    //         | Type::UnitStruct(TypeUnitStruct { common, .. })
    //         | Type::TupleStruct(TypeTupleStruct { common, .. })
    //         | Type::NewtypeStruct(TypeNewtypeStruct { common, .. })
    //         | Type::TypeAlias(TypeTypeAlias { common, .. }) => Some(common),
    //         _ => None,
    //     }
    // }

    pub fn children(&self) -> Vec<Id> {
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

    /// Children that this type "contains" (i.e. cycle-breaking candidates).
    pub fn contained_children(&self) -> Vec<Id> {
        match self {
            Type::TupleStruct(TypeTupleStruct { fields, .. }) => fields.clone(),
            Type::NewtypeStruct(TypeNewtypeStruct { inner, .. }) => vec![inner.clone()],
            Type::Option(id) | Type::Vec(id) | Type::Set(id) | Type::Array(id, _) => {
                vec![id.clone()]
            }
            Type::Map(k, v) => vec![k.clone(), v.clone()],
            Type::Tuple(ids) => ids.clone(),
            Type::Struct(s) => s.properties.iter().map(|p| p.type_id.clone()).collect(),
            Type::Enum(e) => e
                .variants
                .iter()
                .flat_map(|v| v.contained_children())
                .collect(),
            _ => vec![],
        }
    }

    /// Return the list of child types that are contained (i.e. contributed to
    /// the size of this type). This is used to consider containment cycles.
    pub fn contained_children_mut(&mut self) -> Vec<&mut Id> {
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

    pub fn is_named(&self) -> bool {
        matches!(
            self,
            Type::Enum(_)
                | Type::Struct(_)
                | Type::UnitStruct(_)
                | Type::TupleStruct(_)
                | Type::NewtypeStruct(_)
                | Type::TypeAlias(_)
        )
    }
}

pub struct TypespaceBuilder<Id> {
    types: BTreeMap<Id, Type<Id>>,
}

impl<Id> Default for TypespaceBuilder<Id> {
    fn default() -> Self {
        Self {
            types: Default::default(),
        }
    }
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypespaceBuilder<Id> {
    pub fn insert(&mut self, id: Id, typ: Type<Id>) {
        match self.types.entry(id) {
            Entry::Vacant(e) => {
                e.insert(typ);
            }
            Entry::Occupied(_) => {
                // Duplicate insertions are a caller error.
                panic!("duplicate type id");
            }
        }
    }

    pub fn contains_type(&self, id: &Id) -> bool {
        self.types.contains_key(id)
    }

    /// Finalize the typespace.
    ///
    /// `make_box_id` is called to generate a fresh ID for each `Box<T>`
    /// wrapper inserted to break a containment cycle. The argument is the ID
    /// of the inner type being wrapped.
    pub fn finalize<F>(
        self,
        settings: TypespaceSettings,
        make_box_id: F,
    ) -> Result<Typespace<Id>, ()>
    where
        F: FnMut(&Id) -> Id,
    {
        // Basic steps:
        // 1. Break containment cycles with Box types
        // 2. Propagate trait impls
        // 3. Type-specific finalization

        let Self { mut types } = self;

        build_commons(&mut types);
        break_cycles(&mut types, make_box_id);
        push_traits(&mut types)?;

        Ok(Typespace { types, settings })
    }
}

pub fn no_cycles<Id>(_: &Id) -> Id {
    panic!("unexpected cycle in typespace")
}

pub struct Typespace<Id> {
    pub(crate) types: BTreeMap<Id, Type<Id>>,
    pub settings: TypespaceSettings,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> Typespace<Id> {
    pub fn to_codespace(&self) -> codespace::Codespace {
        TypespaceRenderer {
            types: &self.types,
            settings: &self.settings,
        }
        .render()
    }

    pub fn render(&self) -> proc_macro2::TokenStream {
        self.to_codespace().into_stream()
    }
}

pub(crate) struct TypespaceRenderer<'a, Id> {
    pub(crate) types: &'a BTreeMap<Id, Type<Id>>,
    pub(crate) settings: &'a TypespaceSettings,
}

impl<'a, Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypespaceRenderer<'a, Id> {
    fn render(&self) -> codespace::Codespace {
        let mut cs = codespace::Codespace::default();

        for (_, typ) in self.types.iter() {
            match typ {
                Type::Struct(s) => {
                    let name = s.common.built.as_ref().unwrap().name.to_string();
                    let tokens = s.render(self, &mut cs);
                    cs.add_item(name, tokens);
                }
                Type::Enum(e) => {
                    let name = e.common.built.as_ref().unwrap().name.to_string();
                    let tokens = e.render(self, &mut cs);
                    cs.add_item(name, tokens);
                }
                Type::UnitStruct(u) => {
                    let name = u.common.built.as_ref().unwrap().name.to_string();
                    cs.add_item(name, u.render());
                }
                Type::TupleStruct(t) => {
                    let name = t.common.built.as_ref().unwrap().name.to_string();
                    cs.add_item(name, t.render(self));
                }
                Type::NewtypeStruct(n) => {
                    let name = n.common.built.as_ref().unwrap().name.to_string();
                    cs.add_item(name, n.render(self));
                }
                Type::TypeAlias(a) => {
                    let name = a.common.built.as_ref().unwrap().name.to_string();
                    cs.add_item(name, a.render(self));
                }
                _ => {}
            }
        }

        cs
    }

    pub(crate) fn render_ident(&self, id: &Id) -> TokenStream {
        self.render_ident_with_scope(id, None)
    }

    pub(crate) fn render_ident_with_scope(&self, id: &Id, scope: Option<&str>) -> TokenStream {
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

                if let Some(scope) = scope {
                    let scope_ident = format_ident!("{scope}");
                    quote! { #scope_ident::#name_ident }
                } else {
                    name_ident.into_token_stream()
                }
            }

            Type::Native(TypeNative {
                name, parameters, ..
            }) => {
                let name_ident = syn::parse_str::<syn::TypePath>(name).unwrap();
                let parameters = (!parameters.is_empty()).then(|| {
                    let parameter_idents = parameters
                        .iter()
                        .map(|param_id| self.render_ident_with_scope(param_id, scope));
                    quote! {
                        < #( #parameter_idents ),* >
                    }
                });
                quote! {
                    #name_ident #parameters
                }
            }

            Type::Array(schema_ref, n) => {
                let inner_ident = self.render_ident_with_scope(schema_ref, scope);
                quote! {
                    [#inner_ident; #n]
                }
            }
            Type::Tuple(schema_refs) => {
                let inner_idents = schema_refs
                    .iter()
                    .map(|id| self.render_ident_with_scope(id, scope));
                quote! {
                    ( #( #inner_idents ),* )
                }
            }

            Type::Option(option_id) => {
                let option_type = match &self.settings.std {
                    TypespaceSettingsStd::FullyQualified => quote! { ::std::option::Option },
                    TypespaceSettingsStd::Unqualified => quote! { Option },
                };
                let option_ident = self.render_ident_with_scope(option_id, scope);
                quote! {
                    #option_type<#option_ident>
                }
            }
            Type::Box(boxed_id) => {
                let box_type = match &self.settings.std {
                    TypespaceSettingsStd::FullyQualified => quote! { ::std::boxed::Box },
                    TypespaceSettingsStd::Unqualified => quote! { Box },
                };
                let boxed_ident = self.render_ident_with_scope(boxed_id, scope);
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
                let inner_ident = self.render_ident_with_scope(inner_id, scope);
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
                let inner_ident = self.render_ident_with_scope(inner_id, scope);
                quote! {
                    #vec_type<#inner_ident>
                }
            }
            Type::Map(key_id, value_id) => {
                // TODO 3/25/2026
                // Configurable like typify 1
                let key_ident = self.render_ident_with_scope(key_id, scope);
                let value_ident = self.render_ident_with_scope(value_id, scope);
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
        }: &StructProperty<Id>,
        vis_pub: bool,
        context: &str,
        cs: &mut codespace::Codespace,
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
                    flatten
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
            (StructPropertyState::DefaultValue(JsonValue(value)), _) => {
                let fn_name_str = format!("{}__{}", context, rust_name);
                let fn_name_ident = format_ident!("{}", fn_name_str);
                let serde_path = format!("defaults::{fn_name_str}");
                serde_options.push(quote! { default = #serde_path });

                let ty_for_fn = self.render_ident_with_scope(type_id, Some("super"));
                let value_tokens = crate::value_tokens::value_tokens(value);
                cs.get_root_mod().get_mod("defaults").add_item(
                    &fn_name_str,
                    quote! {
                        pub fn #fn_name_ident() -> #ty_for_fn {
                            ::serde_json::from_value(#value_tokens)
                                .expect("invalid default value")
                        }
                    },
                );

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

/// Initialize `TypeCommonBuilt` for every named type before trait propagation.
fn build_commons<Id: Clone>(types: &mut BTreeMap<Id, Type<Id>>) {
    for typ in types.values_mut() {
        let common_opt = match typ {
            Type::Enum(TypeEnum { common, .. })
            | Type::Struct(TypeStruct { common, .. })
            | Type::UnitStruct(TypeUnitStruct { common, .. })
            | Type::TupleStruct(TypeTupleStruct { common, .. })
            | Type::NewtypeStruct(TypeNewtypeStruct { common, .. })
            | Type::TypeAlias(TypeTypeAlias { common, .. }) => Some(common),
            _ => None,
        };
        if let Some(common) = common_opt {
            common.built = Some(TypeCommonBuilt {
                name: common.name.clone(),
                traits: TypespaceTraitSet::empty(),
            });
        }
    }
}

fn break_cycles<Id, F>(types: &mut BTreeMap<Id, Type<Id>>, mut make_box_id: F)
where
    Id: Clone + Ord + std::fmt::Debug + std::fmt::Display,
    F: FnMut(&Id) -> Id,
{
    enum Node<Id> {
        Start { type_id: Id },
        Processing { type_id: Id, children_ids: Vec<Id> },
    }

    let mut visited = BTreeSet::<Id>::new();

    for type_id in types.keys().cloned().collect::<Vec<_>>() {
        if visited.contains(&type_id) {
            continue;
        }

        let mut active = BTreeSet::<Id>::new();
        let mut stack = Vec::<Node<Id>>::new();

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
                            let box_id = make_box_id(&type_id);
                            let box_typ = Type::Box(type_id.clone());
                            types.insert(box_id.clone(), box_typ);

                            (type_id, box_id)
                        })
                        .collect::<BTreeMap<Id, Id>>();

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
                Node::Processing {
                    type_id,
                    children_ids: children,
                } => {
                    if let Some(child) = children.pop() {
                        active.insert(child.clone());
                        stack.push(Node::Start { type_id: child });
                    } else {
                        let type_id = type_id.clone();
                        active.remove(&type_id);
                        stack.pop();
                    }
                }
            }
        }
    }
}

fn push_traits<Id>(types: &mut BTreeMap<Id, Type<Id>>) -> Result<(), ()>
where
    Id: Clone + Ord + std::fmt::Debug + std::fmt::Display,
{
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
                        return Err(());
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
                        return Err(());
                    }
                }
            }
        }
    }

    Ok(())
}
