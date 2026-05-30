use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    StructPropertyState, Type, TypeEnum, TypeNewtypeStruct, TypeStruct, TypeTupleStruct,
    TypeTypeAlias, TypeUnitStruct, Typespace, TypespaceRenderer, TypespaceTrait, VariantDetails,
};

/// Identifies a trait implementation that typespace is aware of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeSpaceImpl {
    Display,
    FromStr,
}

/// A view of a type in a finalized [`Typespace`]. Mirrors `typify::Type<'_>`.
pub struct TypeInfo<'a, Id> {
    pub(crate) typespace: &'a Typespace<Id>,
    pub(crate) id: &'a Id,
    pub(crate) typ: &'a Type<Id>,
}

impl<'a, Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypeInfo<'a, Id> {
    /// The name of this type, or its rendered token representation for unnamed
    /// types.
    pub fn name(&self) -> String {
        match self.typ {
            Type::Enum(e) => e.common.name.clone(),
            Type::Struct(s) => s.common.name.clone(),
            Type::UnitStruct(u) => u.common.name.clone(),
            Type::TupleStruct(t) => t.common.name.clone(),
            Type::NewtypeStruct(n) => n.common.name.clone(),
            Type::TypeAlias(a) => a.common.name.clone(),
            _ => self.ident().to_string(),
        }
    }

    /// The Rust identifier for this type as a [`TokenStream`].
    pub fn ident(&self) -> TokenStream {
        TypespaceRenderer {
            types: &self.typespace.types,
            settings: &self.typespace.settings,
        }
        .render_ident(self.id)
    }

    /// The Rust identifier suitable for use as a function parameter type.
    ///
    /// Complex owned types (structs, enums, Vec, Map, etc.) are prefixed with
    /// `&`; simple types (Option, primitives) are returned unchanged.
    pub fn parameter_ident(&self) -> TokenStream {
        if self.is_simple() {
            self.ident()
        } else {
            let ident = self.ident();
            quote! { &#ident }
        }
    }

    /// The Rust identifier suitable for use as a function parameter type with
    /// an explicit lifetime.
    pub fn parameter_ident_with_lifetime(&self, lifetime: &str) -> TokenStream {
        if self.is_simple() {
            self.ident()
        } else {
            let lifetime_tok =
                syn::Lifetime::new(&format!("'{lifetime}"), proc_macro2::Span::call_site());
            let ident = self.ident();
            quote! { &#lifetime_tok #ident }
        }
    }

    fn is_simple(&self) -> bool {
        matches!(
            self.typ,
            Type::Boolean
                | Type::Integer(_)
                | Type::Float(_)
                | Type::Unit
                | Type::String
                | Type::Option(_)
        )
    }

    /// The description (doc comment source) for this type, if any.
    pub fn description(&self) -> Option<&str> {
        let common = match self.typ {
            Type::Enum(e) => &e.common,
            Type::Struct(s) => &s.common,
            Type::UnitStruct(u) => &u.common,
            Type::TupleStruct(t) => &t.common,
            Type::NewtypeStruct(n) => &n.common,
            Type::TypeAlias(a) => &a.common,
            _ => return None,
        };
        common.description.as_deref()
    }

    /// Structural details of this type.
    pub fn details(&self) -> TypeDetails<'a, Id> {
        match self.typ {
            Type::Enum(e) => TypeDetails::Enum(TypeEnumInfo { inner: e }),
            Type::Struct(s) => TypeDetails::Struct(TypeStructInfo { inner: s }),
            Type::NewtypeStruct(n) => TypeDetails::Newtype(TypeNewtypeInfo { inner: n }),

            Type::Option(id) => TypeDetails::Option(id.clone()),
            Type::Vec(id) => TypeDetails::Vec(id.clone()),
            Type::Map(k, v) => TypeDetails::Map(k.clone(), v.clone()),
            Type::Set(id) => TypeDetails::Set(id.clone()),
            Type::Box(id) => TypeDetails::Box(id.clone()),
            Type::Array(id, n) => TypeDetails::Array(id.clone(), *n),
            Type::Tuple(ids) => TypeDetails::Tuple(Box::new(ids.clone().into_iter())),

            Type::Unit => TypeDetails::Unit,
            Type::String => TypeDetails::String,
            Type::Boolean => TypeDetails::Builtin("bool"),
            Type::Integer(s) => TypeDetails::Builtin(s.as_str()),
            Type::Float(s) => TypeDetails::Builtin(s.as_str()),
            Type::JsonValue => TypeDetails::Builtin("::serde_json::Value"),
            Type::Native(n) => TypeDetails::Builtin(n.name.as_str()),

            // Treat these less-common named types as opaque to callers.
            Type::UnitStruct(_) | Type::TupleStruct(_) | Type::TypeAlias(_) => {
                TypeDetails::Builtin(self.name_str())
            }
        }
    }

    fn name_str(&self) -> &'a str {
        match self.typ {
            Type::UnitStruct(TypeUnitStruct { common, .. })
            | Type::TupleStruct(TypeTupleStruct { common, .. }) => common.name.as_str(),
            Type::TypeAlias(TypeTypeAlias { common, .. }) => common.name.as_str(),
            _ => "",
        }
    }

    /// Returns whether this type has the given trait implementation.
    pub fn has_impl(&self, impl_name: TypeSpaceImpl) -> bool {
        let trait_ = match impl_name {
            TypeSpaceImpl::Display => TypespaceTrait::Display,
            TypeSpaceImpl::FromStr => TypespaceTrait::FromStr,
        };
        match self.typ {
            Type::Native(n) => n.impls.contains(&trait_),
            Type::Enum(e) => e
                .common
                .built
                .as_ref()
                .is_some_and(|b| b.traits.contains(&trait_)),
            Type::Struct(s) => s
                .common
                .built
                .as_ref()
                .is_some_and(|b| b.traits.contains(&trait_)),
            Type::NewtypeStruct(n) => n
                .common
                .built
                .as_ref()
                .is_some_and(|b| b.traits.contains(&trait_)),
            _ => false,
        }
    }
}

/// Structural details of a type. Mirrors `typify::TypeDetails<'_>`.
pub enum TypeDetails<'a, Id> {
    Enum(TypeEnumInfo<'a, Id>),
    Struct(TypeStructInfo<'a, Id>),
    Newtype(TypeNewtypeInfo<'a, Id>),
    Option(Id),
    Vec(Id),
    Map(Id, Id),
    Set(Id),
    Box(Id),
    Tuple(Box<dyn Iterator<Item = Id> + 'a>),
    Array(Id, usize),
    Builtin(&'a str),
    Unit,
    String,
}

// ── Struct view ──────────────────────────────────────────────────────────────

/// A view of a struct type's properties.
pub struct TypeStructInfo<'a, Id> {
    inner: &'a TypeStruct<Id>,
}

impl<'a, Id: Clone> TypeStructInfo<'a, Id> {
    /// Iterate over `(property_name, type_id)` pairs.
    pub fn properties(&'a self) -> impl Iterator<Item = (String, Id)> + 'a {
        self.inner
            .properties
            .iter()
            .map(|p| (p.rust_name.to_string(), p.type_id.clone()))
    }

    /// Iterate over full property information.
    pub fn properties_info(&'a self) -> impl Iterator<Item = TypeStructPropInfo<'a, Id>> {
        self.inner.properties.iter().map(|p| TypeStructPropInfo {
            name: p.rust_name.to_string(),
            description: p.description.as_deref(),
            required: matches!(p.state, StructPropertyState::Required),
            type_id: p.type_id.clone(),
        })
    }
}

/// Information about a single struct property.
pub struct TypeStructPropInfo<'a, Id> {
    /// The Rust field name as a string.
    pub name: String,
    pub description: Option<&'a str>,
    /// `true` if the field must be present in the serialized form.
    pub required: bool,
    pub type_id: Id,
}

// ── Enum view ─────────────────────────────────────────────────────────────────

/// A view of an enum type's variants.
pub struct TypeEnumInfo<'a, Id> {
    inner: &'a TypeEnum<Id>,
}

impl<'a, Id: Clone> TypeEnumInfo<'a, Id> {
    /// Iterate over `(variant_name, variant_details)` pairs.
    pub fn variants(&'a self) -> impl Iterator<Item = (&'a str, TypeEnumVariant<Id>)> {
        self.inner
            .variants
            .iter()
            .map(|v| (v.rust_name.as_str(), variant_details_to_info(&v.details)))
    }

    /// Iterate over full variant information.
    pub fn variants_info(&'a self) -> impl Iterator<Item = TypeEnumVariantInfo<'a, Id>> {
        self.inner.variants.iter().map(|v| TypeEnumVariantInfo {
            name: v.rust_name.as_str(),
            description: v.description.as_deref(),
            details: variant_details_to_info(&v.details),
        })
    }
}

fn variant_details_to_info<Id: Clone>(details: &VariantDetails<Id>) -> TypeEnumVariant<Id> {
    match details {
        VariantDetails::Unit => TypeEnumVariant::Simple,
        VariantDetails::Item(id) => TypeEnumVariant::Tuple(vec![id.clone()]),
        VariantDetails::Tuple(ids) => TypeEnumVariant::Tuple(ids.clone()),
        VariantDetails::Struct(props) => TypeEnumVariant::Struct(
            props
                .iter()
                .map(|p| (p.rust_name.to_string(), p.type_id.clone()))
                .collect(),
        ),
    }
}

/// Full information about a single enum variant.
pub struct TypeEnumVariantInfo<'a, Id> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub details: TypeEnumVariant<Id>,
}

/// The shape of an enum variant's associated data.
pub enum TypeEnumVariant<Id> {
    Simple,
    Tuple(Vec<Id>),
    Struct(Vec<(String, Id)>),
}

// ── Newtype view ──────────────────────────────────────────────────────────────

/// A view of a newtype struct's inner type.
pub struct TypeNewtypeInfo<'a, Id> {
    inner: &'a TypeNewtypeStruct<Id>,
}

impl<'a, Id: Clone> TypeNewtypeInfo<'a, Id> {
    /// The inner type wrapped by this newtype.
    pub fn inner(&self) -> Id {
        self.inner.inner.clone()
    }
}
