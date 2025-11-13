mod type_common;
mod type_enum;
mod type_struct;

use serde::Deserialize;
pub use type_common::*;
pub use type_enum::*;
pub use type_struct::*;

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet, VecDeque};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::{namespace::Namespace, schemalet::SchemaRef};

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

// TODO 9/15/2025
// Placeholder type for non-generated types. We're going to want some mechanism
// to specify the traits we care about so that users have to specify which ones
// are implemented. I'm considering a struct of booleans so that things fail to
// compile if we start to care about some new trait.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TypespaceNativeType {
    pub name: String,
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
/// shape of the data and while--as much as possible--generated code makes use
/// of the derived implementations, the serialized form of some generated types
/// may be a little different.
///
/// - `schemars::JsonSchema` -- As with serde traits, JsonSchema depends on the
/// shape of data and may be customized in some circumstances. In addition,
/// typify supports multiple version of `schemars` so additional configuration
/// may be required to specify the version or to customize the crate name e.g.
/// if one were to support multiple versions simultaneously.
///
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
    /// When true, (the default), types in the `std` crate are fully qualified.
    /// For example, the `Option` type is rendered as `::std::option::Option`.
    /// When false, these types appear in their more typical, auto-imported
    /// form. The latter is useful if one intends to use type generation as a
    /// starting point for manually-edited code.
    #[serde(default)]
    std: TypespaceSettingsStd,

    #[serde(default)]
    optional_nullable: TypespaceSettingsOptionalNullable,
}

#[derive(Debug, Default, Deserialize)]
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
    /// Model `null` and `optional` as equivalent using the
    /// `std::option::Option<T>` type. Skip serialization of `None` values.
    /// This is the default.
    #[default]
    ConflateAsAbsent,

    /// Model `null` and `optional` as equivalent using the
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
    /// specifies the value of a field when absent; the `Deserialize`
    /// implementation produces a value otherwise (null or a non-null value of
    /// T). In addition, `Opt` must implement `is_absent(&self) -> bool` which
    /// is used with the serde `skip_serializing_if` attribute to omit the
    /// field.
    CustomType(String),
}

pub struct Typespace {
    settings: TypespaceSettings,
    types: BTreeMap<SchemaRef, Type>,
}

impl Typespace {
    pub fn render(&self) -> TokenStream {
        let types = self.types.iter().map(|(id, typ)| match typ {
            Type::Enum(type_enum) => {
                let TypeEnum {
                    name,
                    description,
                    default,
                    tag_type,
                    variants,
                    deny_unknown_fields,
                    built,
                } = type_enum;
                let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});
                let serde = match tag_type {
                    EnumTagType::External => TokenStream::new(),
                    EnumTagType::Internal { tag } => quote! {
                        #[serde(tag = #tag)]
                    },
                    EnumTagType::Adjacent { tag, content } => quote! {
                        #[serde(tag = #tag, content = #content)]
                    },
                    EnumTagType::Untagged => quote! {
                        #[serde(untagged)]
                    },
                };

                let variants = variants.iter().map(|variant| {
                    let EnumVariant {
                        rust_name,
                        rename,
                        description,
                        details,
                    } = variant;
                    let name = format_ident!("{}", rust_name);
                    let variant_serde = rename.as_ref().map(|n| {
                        quote! {
                            #[serde(rename = #n)]
                        }
                    });
                    let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});

                    let data = match details {
                        VariantDetails::Simple => TokenStream::new(),
                        VariantDetails::Item(item) => {
                            let item_ident = self.render_ident(item);
                            quote! {
                                (#item_ident)
                            }
                        }
                        VariantDetails::Tuple(items) => todo!(),
                        VariantDetails::Struct(properties) => {
                            let properties = properties
                                .iter()
                                .map(|struct_prop| self.render_struct_property(struct_prop, false));
                            quote! {
                                {
                                    #( #properties, )*
                                }
                            }
                        }
                    };

                    quote! {
                        #description
                        #variant_serde
                        #name #data
                    }
                });

                let name = built.as_ref().unwrap().name.to_string();
                let name_ident = format_ident!("{name}");

                quote! {
                    // TODO I want to have the original unique id available
                    #description
                    #[derive(::serde::Deserialize, ::serde::Serialize)]
                    #serde
                    pub enum #name_ident {
                        #( #variants, )*
                    }
                }
            }
            Type::Struct(type_struct) => {
                let TypeStruct {
                    name: _,
                    description,
                    default,
                    properties,
                    deny_unknown_fields,
                    built,
                } = type_struct;
                let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});
                let properties = properties
                    .iter()
                    .map(|prop| self.render_struct_property(prop, true));

                let name = built.as_ref().unwrap().name.to_string();
                let name_ident = format_ident!("{name}");

                quote! {
                    #description
                    #[derive(::serde::Deserialize, ::serde::Serialize)]
                    pub struct #name_ident {
                        #( #properties, )*
                    }
                }
            }
            _ => quote! {},
        });

        quote! {
            #( #types )*
        }
    }

    fn render_ident(&self, id: &SchemaRef) -> TokenStream {
        let ty = self.types.get(id).unwrap();
        match ty {
            Type::Enum(type_enum) => {
                let name = type_enum.built.as_ref().unwrap().name.to_string();
                let name_ident = format_ident!("{name}");
                name_ident.into_token_stream()
            }
            Type::Struct(_) => {
                quote! { Ref<"???"> }
            }
            Type::Native(native_type) => syn::parse_str::<syn::Type>(native_type)
                .unwrap()
                .into_token_stream(),
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
            Type::Vec(inner_id) => {
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
                let key_ident = self.render_ident(key_id);
                let value_ident = self.render_ident(value_id);
                quote! {
                    ::std::collections::BTreeMap<#key_ident, #value_ident>
                }
            }
            // Type::Set(_) => todo!(),
            // Type::Array(_, _) => todo!(),
            // Type::Tuple(items) => todo!(),
            // Type::Unit => todo!(),
            Type::Boolean => quote! { bool },
            Type::Integer(name) | Type::Float(name) => syn::parse_str::<syn::TypePath>(name)
                .unwrap()
                .to_token_stream(),
            Type::String => match &self.settings.std {
                TypespaceSettingsStd::FullyQualified => quote! { ::std::string::String },
                TypespaceSettingsStd::Unqualified => quote! { String },
            },
            Type::JsonValue => quote! { ::serde_json::Value },
            _ => quote! { () },
        }
    }

    fn render_struct_property(
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
            (StructPropertyState::Required, None) => ty_ident,
            (StructPropertyState::Required, Some(_)) => {
                let opt_deserialize = format!("{std_opt_type}::deserialize");
                // TODO schemars schema_with?
                serde_options.push(quote! { deserialize_with = #opt_deserialize });
                ty_ident
            }
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
                    Type::Enum(type_enum) => todo!(),
                    Type::Struct(type_struct) => todo!(),
                    Type::Native(_) => todo!(),
                    Type::Option(schema_ref) => {
                        // This case is basically meaningless, but it's also
                        // fine. Note that #[serde(default)] is a no-op for
                        // Option<T>.
                        serde_options.push(quote! { skip_serializing_if = #std_opt_is_none });
                    }
                    Type::Box(schema_ref) => todo!(),

                    Type::Vec(_) | Type::Map(_, _) | Type::Set(_) | Type::String => {
                        let is_empty = format!("{ty_ident}::is_empty");
                        serde_options.push(quote! { skip_serializing_if = #is_empty });
                    }

                    Type::Array(schema_ref, _) => todo!(),
                    Type::Tuple(schema_refs) => todo!(),
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
            (StructPropertyState::DefaultValue(json_value), _) => {
                // XXX
                // - make a function that produces the value
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
        match self.types.entry(id.into()) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(typ.into());
            }
            Entry::Occupied(occupied_entry) => {
                let key = occupied_entry.key();
                todo!()
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

        while let Some((parent_id, child_id, child_sigil)) = work.pop_front() {
            let child_typ = types.get(&child_id).unwrap();

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

        println!("{:#?}", name_hints);

        types.iter_mut().for_each(|(id, typ)| {
            if let Some(hints) = name_hints.remove(id) {
                typ.add_name_hints(hints);
            }
        });

        let mut namespace = Namespace::<SchemaRef>::default();

        for (id, typ) in &mut types {
            match typ {
                Type::Enum(type_enum) => {
                    let name = match &type_enum.name {
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
                    type_enum.built = Some(TypeEnumBuilt { name });
                }
                Type::Struct(type_struct) => {
                    let name = match &type_struct.name {
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
                    type_struct.built = Some(TypeStructBuilt { name });
                }
                _ => {}
            }

            println!("{:#?}", typ);
        }

        let n2 = namespace.finalize().unwrap();

        // TODO 7/1/2025
        // Let's do names first.

        // TODO Make sure that all referenced schemas are present.
        // TODO break cycles
        // TODO resolve names
        // TODO propagate trait impls

        // Break cycles
        break_cycles(&mut types);

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

#[derive(Debug, Clone)]
pub enum Type {
    Enum(TypeEnum),
    Struct(TypeStruct),

    Native(String),
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
        if let Some(name) = match self {
            Type::Enum(type_enum) => Some(&mut type_enum.name),
            Type::Struct(type_struct) => Some(&mut type_struct.name),
            _ => None,
        } {
            match name {
                NameBuilder::Unset => *name = NameBuilder::Hints(hints),
                NameBuilder::Fixed(_) => {}
                NameBuilder::Hints(_) => unreachable!(),
            }
        }
    }

    fn get_name(&self) -> Option<&NameBuilder> {
        match self {
            Type::Enum(type_enum) => Some(&type_enum.name),
            Type::Struct(type_struct) => Some(&type_struct.name),
            _ => None,
        }
    }
    fn is_named(&self) -> bool {
        match self {
            Type::Enum(type_enum) => true,
            Type::Struct(type_struct) => true,
            _ => false,
        }
    }

    pub fn children(&self) -> Vec<SchemaRef> {
        match self {
            Type::Enum(type_enum) => type_enum.children(),
            Type::Struct(type_struct) => type_struct.children(),
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
            Type::Native(_) => Vec::new(),
            Type::Option(id) => vec![(id.clone(), "".to_string())],
            Type::Box(_) => todo!(),
            Type::Vec(id) => vec![(id.clone(), "item".to_string())],
            Type::Map(key_id, value_id) => vec![
                (key_id.clone(), "key".to_string()),
                (value_id.clone(), "value".to_string()),
            ],
            Type::Set(_) => todo!(),
            Type::Array(_, _) => todo!(),
            Type::Tuple(items) => todo!(),

            Type::Unit => Vec::new(),
            Type::Boolean => Vec::new(),
            Type::Integer(_) => Vec::new(),
            Type::Float(_) => Vec::new(),
            Type::String => Vec::new(),
            Type::JsonValue => Vec::new(),
        }
    }

    pub fn contained_children_mut(&mut self) -> Vec<&mut SchemaRef> {
        match self {
            Type::Enum(TypeEnum { variants, .. }) => {
                let mut out = Vec::new();
                for variant in variants {
                    match &mut variant.details {
                        VariantDetails::Simple => {}
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
            Type::Option(id) => vec![id],
            Type::Array(_, _) => todo!(),
            Type::Tuple(items) => todo!(),

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
            TypeStruct, TypespaceBuilder, TypespaceSettings, TypespaceSettingsStd,
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
}
