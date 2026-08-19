//! Local extensions to the standalone `typespace` crate's interface.
//!
//! Each item in this file is an interface proposal: a call site elsewhere in
//! this crate is written as though the interface existed in `typespace`, and
//! the shim here backs it. Each item is annotated with the proposed
//! `typespace` signature and a one-line rationale. Nothing in this file
//! should grow logic of its own beyond what the proposed interface would
//! contain.

use typespace::{StructProperty, Type, TypeNewtypeStruct, TypeStruct, TypeTypeAlias, VariantDetails};

/// Proposed typespace signature:
///
///     impl<Id: Clone> Type<Id> {
///         pub fn children_with_context(&self) -> Vec<(Id, String)>
///     }
///
/// Rationale: a pre-insertion naming pass needs each child ID along with the
/// context (property name, variant name, tuple index, ...) in which the
/// parent refers to it; only typespace knows the structure of every variant,
/// and reaching into each type's public fields from the outside duplicates
/// that knowledge here.
///
/// (This function--and the per-variant helpers below--are ports of the
/// `children_with_context` methods that lived on the internal typespace
/// module's types before the migration to the standalone crate.)
pub(crate) fn children_with_context<Id: Clone>(typ: &Type<Id>) -> Vec<(Id, String)> {
    match typ {
        Type::Enum(type_enum) => type_enum
            .variants
            .iter()
            .flat_map(variant_children_with_context)
            .collect::<Vec<_>>(),
        Type::Struct(type_struct) => struct_children_with_context(type_struct),
        Type::UnitStruct(_) => Vec::new(),
        Type::TupleStruct(type_tuple_struct) => {
            let mut children = type_tuple_struct
                .fields
                .iter()
                .cloned()
                .enumerate()
                .map(|(ii, type_id)| (type_id, ii.to_string()))
                .collect::<Vec<_>>();

            if let Some(rest) = &type_tuple_struct.rest {
                children.push((rest.clone(), type_tuple_struct.fields.len().to_string()));
            }

            children
        }
        Type::NewtypeStruct(TypeNewtypeStruct { inner, .. }) => {
            vec![(inner.clone(), "inner".to_string())]
        }

        // TODO 2/4/2026
        // I'm not really sure what to do here; the type alias shouldn't really
        // confer any context to the inner type. I think this is fine, but also
        // may mean that I want to change the name of this pass/fn/concept to
        // indicate that we should only express context insofar as we have it,
        // and that the cardinality of this fn may be different from that of
        // children().
        Type::TypeAlias(TypeTypeAlias { .. }) => Vec::new(),

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

        // Type is #[non_exhaustive]; a new variant would need context
        // plumbing here (and in the proposed method), so fail loudly.
        _ => todo!("children_with_context for unknown Type variant"),
    }
}

fn struct_children_with_context<Id: Clone>(type_struct: &TypeStruct<Id>) -> Vec<(Id, String)> {
    type_struct
        .properties
        .iter()
        .map(
            |StructProperty {
                 rust_name, type_id, ..
             }| (type_id.clone(), format!("{rust_name}")),
        )
        .collect::<Vec<_>>()
}

fn variant_children_with_context<Id: Clone>(
    variant: &typespace::EnumVariant<Id>,
) -> Vec<(Id, String)> {
    match &variant.details {
        VariantDetails::Unit => Vec::new(),
        VariantDetails::Item(id) => vec![(id.clone(), variant.rust_name.clone())],
        VariantDetails::Tuple(items) => items
            .iter()
            .enumerate()
            .map(|(ii, id)| (id.clone(), format!("{}.{}", &variant.rust_name, ii)))
            .collect::<Vec<_>>(),
        VariantDetails::Struct(items) => items
            .iter()
            .map(|prop| {
                (
                    prop.type_id.clone(),
                    format!("{}.{}", &variant.rust_name, &prop.rust_name),
                )
            })
            .collect::<Vec<_>>(),
    }
}
