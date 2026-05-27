mod type_common;
mod type_enum;
mod type_native;
mod type_struct;

pub use type_common::*;
pub use type_enum::*;
pub use type_native::*;
pub use type_struct::*;

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet, VecDeque};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;

/// Modify how types are processed and generated.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct TypespaceSettings {
    #[serde(default)]
    pub std: TypespaceSettingsStd,
    #[serde(default)]
    pub optional_nullable: TypespaceSettingsOptionalNullable,
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TypespaceSettingsStd {
    #[default]
    FullyQualified,
    Unqualified,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TypespaceSettingsOptionalNullable {
    #[default]
    ConflateAsAbsent,
    ConflateAsNull,
    DoubleOption,
    CustomType(String),
}

/// Traits for which Typespace has particular awareness.
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

#[derive(Debug, Clone, Default)]
pub struct TypespaceTraitSet(BTreeSet<TypespaceTrait>);

impl TypespaceTraitSet {
    pub fn empty() -> Self {
        Self(BTreeSet::new())
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
}

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

/// Represents a type in the Typespace.
#[derive(Debug, Clone)]
pub enum Type<Id> {
    Enum(TypeEnum<Id>),
    Struct(TypeStruct<Id>),
    UnitStruct(TypeUnitStruct),
    TupleStruct(TypeTupleStruct<Id>),
    NewtypeStruct(TypeNewtypeStruct<Id>),
    TypeAlias(TypeTypeAlias<Id>),

    Native(TypeNative),
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
    pub fn children(&self) -> Vec<Id> {
        match self {
            Type::Enum(e) => e.children(),
            Type::Struct(s) => s.children(),
            Type::UnitStruct(_) | Type::TupleStruct(_) | Type::NewtypeStruct(_) => {
                self.contained_children()
            }
            Type::TypeAlias(a) => vec![a.type_id.clone()],
            Type::Native(_) => vec![],
            Type::Option(id) | Type::Box(id) | Type::Vec(id) | Type::Set(id) => vec![id.clone()],
            Type::Map(k, v) => vec![k.clone(), v.clone()],
            Type::Array(id, _) => vec![id.clone()],
            Type::Tuple(ids) => ids.clone(),
            Type::Unit
            | Type::Boolean
            | Type::Integer(_)
            | Type::Float(_)
            | Type::String
            | Type::JsonValue => vec![],
        }
    }

    /// Children that this type "contains" (i.e. cycle-breaking candidates).
    pub fn contained_children(&self) -> Vec<Id> {
        match self {
            Type::TupleStruct(TypeTupleStruct { fields, .. }) => fields.clone(),
            Type::NewtypeStruct(TypeNewtypeStruct { type_id, .. }) => vec![type_id.clone()],
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
    pub fn finalize<F>(self, make_box_id: F) -> Result<Typespace<Id>, ()>
    where
        F: FnMut(&Id) -> Id,
    {
        let Self { mut types } = self;

        break_cycles(&mut types, make_box_id);
        push_traits(&mut types)?;

        Ok(Typespace { types })
    }
}

pub fn no_cycles<Id>(_: &Id) -> Id {
    panic!("unexpected cycle in typespace")
}

pub struct Typespace<Id> {
    pub(crate) types: BTreeMap<Id, Type<Id>>,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> Typespace<Id> {
    pub fn render(&self, settings: TypespaceSettings) -> TokenStream {
        TypespaceRenderer {
            types: &self.types,
            settings,
        }
        .render()
    }
}

pub(crate) struct TypespaceRenderer<'a, Id> {
    pub(crate) types: &'a BTreeMap<Id, Type<Id>>,
    pub(crate) settings: TypespaceSettings,
}

impl<'a, Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypespaceRenderer<'a, Id> {
    fn render(&self) -> TokenStream {
        let types = self.types.iter().filter_map(|(_, typ)| match typ {
            Type::Enum(e) => Some(e.render(self)),
            Type::Struct(s) => Some(s.render(self)),
            Type::UnitStruct(u) => Some(u.render()),
            Type::TupleStruct(t) => Some(t.render(self)),
            Type::NewtypeStruct(n) => Some(n.render(self)),
            Type::TypeAlias(a) => Some(a.render(self)),
            _ => None,
        });

        quote! { #( #types )* }
    }

    pub fn render_ident(&self, id: &Id) -> TokenStream {
        match self.types.get(id) {
            Some(typ) => {
                if let Some(common) = match typ {
                    Type::Enum(TypeEnum { common, .. })
                    | Type::Struct(TypeStruct { common, .. })
                    | Type::UnitStruct(TypeUnitStruct { common, .. })
                    | Type::TupleStruct(TypeTupleStruct { common, .. })
                    | Type::NewtypeStruct(TypeNewtypeStruct { common, .. })
                    | Type::TypeAlias(TypeTypeAlias { common, .. }) => Some(common),
                    _ => None,
                } {
                    let ident = format_ident!("{}", common.name);
                    quote! { #ident }
                } else {
                    self.render_type(id)
                }
            }
            None => panic!("unknown type id: {id}"),
        }
    }

    pub fn render_type(&self, id: &Id) -> TokenStream {
        let std_opt = self.std_prefix("option::Option");
        let std_vec = self.std_prefix("vec::Vec");
        let std_box = self.std_prefix("boxed::Box");
        let std_map = self.std_prefix("collections::BTreeMap");
        let std_set = self.std_prefix("collections::BTreeSet");
        let std_str = self.std_prefix("string::String");

        match self.types.get(id).expect("unknown type id") {
            Type::Enum(_)
            | Type::Struct(_)
            | Type::UnitStruct(_)
            | Type::TupleStruct(_)
            | Type::NewtypeStruct(_)
            | Type::TypeAlias(_) => self.render_ident(id),
            Type::Native(n) => n.render(),
            Type::Option(inner) => {
                let inner = self.render_type(inner);
                quote! { #std_opt<#inner> }
            }
            Type::Box(inner) => {
                let inner = self.render_type(inner);
                quote! { #std_box<#inner> }
            }
            Type::Vec(inner) => {
                let inner = self.render_type(inner);
                quote! { #std_vec<#inner> }
            }
            Type::Map(k, v) => {
                let k = self.render_type(k);
                let v = self.render_type(v);
                quote! { #std_map<#k, #v> }
            }
            Type::Set(inner) => {
                let inner = self.render_type(inner);
                quote! { #std_set<#inner> }
            }
            Type::Array(inner, len) => {
                let inner = self.render_type(inner);
                quote! { [#inner; #len] }
            }
            Type::Tuple(ids) => {
                let items = ids.iter().map(|id| self.render_type(id));
                quote! { (#( #items, )*) }
            }
            Type::Unit => quote! { () },
            Type::Boolean => quote! { bool },
            Type::Integer(s) | Type::Float(s) => {
                let ident = format_ident!("{s}");
                quote! { #ident }
            }
            Type::String => quote! { #std_str },
            Type::JsonValue => quote! { ::serde_json::Value },
        }
    }

    fn std_prefix(&self, path: &str) -> TokenStream {
        if self.settings.std == TypespaceSettingsStd::FullyQualified {
            let segments = std::iter::once("std")
                .chain(path.split("::"))
                .map(|s| format_ident!("{s}"));
            quote! { :: #( #segments )::* }
        } else {
            let last = path.split("::").last().unwrap();
            let ident = format_ident!("{last}");
            quote! { #ident }
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
        default_fn_name: Option<&str>,
    ) -> TokenStream {
        let description = description.as_ref().map(|text| quote! { #[doc = #text] });

        let mut serde_options = Vec::new();

        match json_name {
            StructPropertySerde::None => {}
            StructPropertySerde::Rename(s) => {
                serde_options.push(quote! { rename = #s });
            }
            StructPropertySerde::Flatten => {
                serde_options.push(quote! { flatten });
            }
        }

        let ty = self
            .types
            .get(type_id)
            .expect("unknown type_id in struct property");

        let maybe_option_inner = if let Type::Option(id) = ty {
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

        let ty_ident = match (state, maybe_option_inner) {
            (StructPropertyState::Required, None) => ty_ident,

            (StructPropertyState::Required, Some(_)) => {
                let opt_deserialize = format!("{std_opt_type}::deserialize");
                serde_options.push(quote! { deserialize_with = #opt_deserialize });
                ty_ident
            }

            (StructPropertyState::Optional, None) => {
                serde_options.push(quote! { default });
                serde_options.push(quote! {
                    deserialize_with = "::json_serde::deserialize_some"
                });
                serde_options.push(quote! { skip_serializing_if = #std_opt_is_none });
                quote! { #std_opt_type<#ty_ident> }
            }

            (StructPropertyState::Optional, Some(inner_id)) => {
                match &self.settings.optional_nullable {
                    TypespaceSettingsOptionalNullable::ConflateAsAbsent => {
                        serde_options.push(quote! { skip_serializing_if = #std_opt_is_none });
                        ty_ident
                    }
                    TypespaceSettingsOptionalNullable::ConflateAsNull => ty_ident,
                    TypespaceSettingsOptionalNullable::DoubleOption => {
                        serde_options.push(quote! { default });
                        serde_options.push(quote! {
                            deserialize_with = "::json_serde::deserialize_some"
                        });
                        serde_options.push(quote! { skip_serializing_if = #std_opt_is_none });
                        quote! { #std_opt_type<#ty_ident> }
                    }
                    TypespaceSettingsOptionalNullable::CustomType(custom_type_name) => {
                        let custom_type_path =
                            syn::parse_str::<syn::TypePath>(custom_type_name).unwrap();
                        serde_options.push(quote! { default });
                        let custom_is_absent = format!("{}::is_absent", custom_type_name);
                        serde_options.push(quote! {
                            skip_serializing_if = #custom_is_absent
                        });
                        let inner_ident = self.render_ident(inner_id);
                        quote! { #custom_type_path<#inner_ident> }
                    }
                }
            }

            (StructPropertyState::Default, _) => {
                serde_options.push(quote! { default });
                match ty {
                    Type::Option(_) => {
                        serde_options.push(quote! { skip_serializing_if = #std_opt_is_none });
                    }
                    Type::Vec(_) | Type::Map(_, _) | Type::Set(_) | Type::String => {
                        let is_empty = format!("{ty_ident}::is_empty");
                        serde_options.push(quote! { skip_serializing_if = #is_empty });
                    }
                    _ => {}
                }
                ty_ident
            }

            (StructPropertyState::DefaultValue(_), _) => {
                let fn_name =
                    default_fn_name.expect("DefaultValue field requires a default_fn_name");
                serde_options.push(quote! { default = #fn_name });
                ty_ident
            }

            // Phantom variant — unreachable in generated code.
            (StructPropertyState::_Phantom(_), _) => unreachable!(),
        };

        let serde = (!serde_options.is_empty()).then(|| {
            quote! { #[serde( #( #serde_options ),* )] }
        });
        let vis = vis_pub.then(|| quote! { pub });

        quote! {
            #description
            #serde
            #vis #rust_name: #ty_ident
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
        Processing { type_id: Id, children: Vec<Id> },
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
                Node::Start { type_id } if visited.contains(type_id) => {
                    let type_id = type_id.clone();
                    *top = Node::Processing {
                        type_id,
                        children: Vec::new(),
                    };
                }
                Node::Start { type_id } => {
                    visited.insert(type_id.clone());

                    let contained = types
                        .get(type_id)
                        .map(|t| t.contained_children())
                        .unwrap_or_default();

                    let (snip, descend): (Vec<_>, Vec<_>) =
                        contained.into_iter().partition(|c| active.contains(c));

                    for cyclic_id in snip {
                        let box_id = make_box_id(&cyclic_id);
                        let inner = types.remove(&cyclic_id).expect("cyclic type missing");
                        types.insert(box_id.clone(), inner);
                        types.insert(cyclic_id.clone(), Type::Box(box_id));

                        // Rewrite references to the cyclic type in all types.
                        for typ in types.values_mut() {
                            rewrite_id(typ, &cyclic_id, &cyclic_id);
                        }
                    }

                    let type_id = type_id.clone();
                    *top = Node::Processing {
                        type_id,
                        children: descend,
                    };
                }
                Node::Processing { type_id, children } => {
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

fn rewrite_id<Id: Clone + PartialEq + std::fmt::Display>(typ: &mut Type<Id>, from: &Id, to: &Id) {
    let rewrite = |id: &mut Id| {
        if id == from {
            *id = to.clone();
        }
    };
    match typ {
        Type::Option(id) | Type::Box(id) | Type::Vec(id) | Type::Set(id) => rewrite(id),
        Type::Map(k, v) => {
            rewrite(k);
            rewrite(v);
        }
        Type::Array(id, _) => rewrite(id),
        Type::Tuple(ids) => ids.iter_mut().for_each(rewrite),
        Type::TypeAlias(a) => rewrite(&mut a.type_id),
        Type::NewtypeStruct(n) => rewrite(&mut n.type_id),
        Type::TupleStruct(t) => t.fields.iter_mut().for_each(rewrite),
        Type::Struct(s) => s
            .properties
            .iter_mut()
            .for_each(|p| rewrite(&mut p.type_id)),
        Type::Enum(e) => e.variants.iter_mut().for_each(|v| v.rewrite_id(from, to)),
        _ => {}
    }
}

fn push_traits<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display>(
    types: &mut BTreeMap<Id, Type<Id>>,
) -> Result<(), ()> {
    let mut work = types
        .iter()
        .filter_map(|(_, ty)| match ty {
            Type::Map(key_id, _) => Some((
                key_id.clone(),
                [
                    TypespaceTrait::Eq,
                    TypespaceTrait::PartialEq,
                    TypespaceTrait::Ord,
                    TypespaceTrait::PartialOrd,
                ]
                .into_iter()
                .collect::<TypespaceTraitSet>(),
            )),
            _ => None,
        })
        .collect::<VecDeque<_>>();

    while let Some((id, traits)) = work.pop_front() {
        // Validate and collect children before any mutable access.
        let (is_named, children) = match types.get(&id) {
            Some(Type::Float(_)) => {
                if traits.contains(&TypespaceTrait::Ord)
                    || traits.contains(&TypespaceTrait::Eq)
                    || traits.contains(&TypespaceTrait::Hash)
                {
                    return Err(());
                }
                (false, vec![])
            }
            Some(Type::JsonValue) => {
                if traits.contains(&TypespaceTrait::Eq)
                    || traits.contains(&TypespaceTrait::Ord)
                    || traits.contains(&TypespaceTrait::PartialOrd)
                    || traits.contains(&TypespaceTrait::Hash)
                {
                    return Err(());
                }
                (false, vec![])
            }
            Some(typ) => (typ.is_named(), typ.children()),
            None => (false, vec![]),
        };

        if !is_named {
            continue;
        }

        let typ = types.get_mut(&id).unwrap();
        let common = match typ {
            Type::Enum(TypeEnum { common, .. })
            | Type::Struct(TypeStruct { common, .. })
            | Type::UnitStruct(TypeUnitStruct { common, .. })
            | Type::TupleStruct(TypeTupleStruct { common, .. })
            | Type::NewtypeStruct(TypeNewtypeStruct { common, .. }) => Some(common),
            _ => None,
        };

        if let Some(common) = common {
            let mut new_traits = TypespaceTraitSet::empty();
            for trait_name in traits {
                if !common.traits.contains(&trait_name) {
                    common.traits.add(trait_name);
                    new_traits.add(trait_name);
                }
            }

            if !new_traits.is_empty() {
                for child_id in children {
                    work.push_back((child_id, new_traits.clone()));
                }
            }
        }
    }

    Ok(())
}
