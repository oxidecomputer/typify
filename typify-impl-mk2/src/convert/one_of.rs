use std::{borrow::Cow, collections::BTreeSet};

use heck::ToPascalCase;

use crate::{
    convert::Converter,
    schemalet::{
        CanonicalSchemalet, CanonicalSchemaletDetails, SchemaRef, SchemaletMetadata,
        SchemaletValue, SchemaletValueObject,
    },
    typespace::{
        EnumTagType, EnumVariant, NameBuilder, StructProperty, Type, TypeEnum, VariantDetails,
    },
};

impl Converter {
    // 4/7/2026
    // Below are some old notes that I wanted to preserve because they're not
    // fully stupid.
    //
    // There are 4 different patterns for enum variants
    // - Externally tagged: either strings or objects with a single property
    // - Adjacently tagged: object with a constant-value property and an
    //   optional other property whose value can be anything
    // - Internally tagged: object whose constant-value properties we record
    // - Untagged: everything else
    //
    // Note that there is the potential for overlap! For example, every
    // adjacently tagged variant could be interpreted as an internally tagged
    // variant (so we need to evaluate the criteria for adjacent tagging before
    // internal tagging!). Note that an adjacently tagged enum is strictly
    // less repetitive so there's good reason to prefer it.

    // 6/21/2025
    // Interesting question about enum variants whose schemas are objects:
    // when should we embed them vs. creating a new type? I expect we'll need
    // some pass where we decide which schemas / schema refs we **want** to be
    // distinct types and which are we simply fine with that happening should
    // that be what shakes out.
    // Often I expect we'll want everything with "$defs" in it to be a named
    // schema in part because we know we have some recognizable name. In the
    // case of the meta schema I think we don't need particularly bless
    // anything--we can just let the chips fall where they may.

    // TODO For now, we're just going to do untagged variants.

    // 6/21/2025
    // For untagged enums, naming is interesting, and maybe an interesting
    // place to start for the naming project. For each variant, we can
    // construct a clearly unique name: Variant{n}. But that name kind of
    // sucks. We have the possibility of naming variants after types they
    // reference or after the fundamental type of the variant (e.g. boolean,
    // integer, object) if that alone is a sufficiently distinguishing
    // characteristic. I think the "best" names are going to be those provided
    // by the schema itself in the form of titles and named types. After that,
    // we fall back to boolean/integer, and after that we fall back to
    // Variant{n}.
    //
    // Two additional, signifcant pieces of complexity. First, we don't want to
    // mix and match e.g. Variant0, Object looks dumb. Second, we don't
    // necessarily know type name resolution until later, but--I suppose we
    // *do* know there will be a name and we *do* know the names will be unique
    // (initially I thought we'd need to choose the modality after doing *all*
    // the other type generation, etc. but now I don't think that's the
    // case--we can probably know right away if a type is going to have some
    // good name or not).
    //
    // So! We'll first try to make a set of names based on named types, then
    // make a set of onological names, and finally fall back to numbered names.
    // This could suck! Imagine an enum whose variants were all complex types
    // for which we needed to invent names because the schema declined to name
    // them: the variant names could also suck. But... meh? We'll see how it
    // shakes out.
    //
    // But without looking pretty closely at a type, how do we know if it needs
    // a name, etc? I guess we'll just suck it up and look at it.

    pub(crate) fn convert_one_of(
        &self,
        name: NameBuilder,
        metadata: &SchemaletMetadata,
        subschemas: &[SchemaRef],
    ) -> Type {
        let resolved_subschemas = subschemas
            .into_iter()
            .map(|schema_ref| self.get(schema_ref))
            .collect::<Vec<_>>();

        println!(
            "subschemas {}",
            serde_json::to_string_pretty(&resolved_subschemas).unwrap()
        );

        let proto_variants = subschemas
            .iter()
            .map(|variant_id| {
                let schemalet = self.get(variant_id);
                // TODO this is where we are going to look for titles and
                // descriptions. We're going to keep walking until we hit a
                // concrete type.
                ProtoVariant {
                    id: variant_id,
                    schemalet,
                    name: None,
                    description: None,
                }
            })
            .collect::<Vec<_>>();

        println!("{}", serde_json::to_string_pretty(&proto_variants).unwrap());

        let ty = if let Some(ty) =
            self.maybe_externally_tagged_enum(name.clone(), metadata, &proto_variants)
        {
            ty
        } else {
            // TODO ... adjacent and internal
            self.untagged_enum(name, metadata, &proto_variants)
        };

        ty
    }

    fn maybe_externally_tagged_enum(
        &self,
        name: NameBuilder,
        metadata: &SchemaletMetadata,
        proto_variants: &[ProtoVariant],
    ) -> Option<Type> {
        let variants = proto_variants
            .iter()
            .map(|proto| match &proto.schemalet.details {
                CanonicalSchemaletDetails::Anything => None,
                CanonicalSchemaletDetails::Nothing => None,
                CanonicalSchemaletDetails::Constant(value) => Some(vec![ProtoVariantExternal {
                    proto: Cow::Borrowed(proto),
                    kind: ProtoVariantExternalKind::Simple(value.as_str()?.to_string()),
                }]),
                CanonicalSchemaletDetails::Reference(_) | CanonicalSchemaletDetails::Note(_) => {
                    unreachable!("we should have already eliminated this possibility")
                }
                CanonicalSchemaletDetails::ExclusiveOneOf { subschemas, .. } => subschemas
                    .iter()
                    .map(|id| {
                        let ss = self.resolve(id);

                        if let CanonicalSchemaletDetails::Constant(value) = &ss.details {
                            Some(ProtoVariantExternal {
                                proto: Cow::Owned(ProtoVariant {
                                    id,
                                    schemalet: ss,
                                    name: None,
                                    description: None,
                                }),
                                kind: ProtoVariantExternalKind::Simple(value.as_str()?.to_string()),
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Option<Vec<_>>>(),
                CanonicalSchemaletDetails::Value(SchemaletValue::Object(
                    // TODO more checks?
                    // TODO required!
                    SchemaletValueObject { properties, .. },
                )) if properties.len() == 1 => {
                    let (name, schema_ref) = properties.iter().next().unwrap().clone();
                    Some(vec![ProtoVariantExternal {
                        proto: Cow::Borrowed(proto),
                        kind: ProtoVariantExternalKind::Typed(name.clone(), schema_ref),
                    }])
                }
                CanonicalSchemaletDetails::Value(_) => None,
            })
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .flatten()
            .map(|ProtoVariantExternal { proto, kind }| {
                match kind {
                    ProtoVariantExternalKind::Simple(variant_name) => {
                        let rust_name = variant_name.to_pascal_case();
                        let rename = (variant_name != rust_name).then_some(variant_name);
                        EnumVariant {
                            rust_name,
                            rename,
                            description: proto.description.clone(),
                            details: VariantDetails::Unit,
                        }
                    }
                    ProtoVariantExternalKind::Typed(variant_name, schema_ref) => todo!(),
                }
                // todo!();
                // EnumVariant {
                //     variant_name,
                //     description: proto.description.clone(),
                //     details: (),
                // }
            })
            .collect::<Vec<_>>();

        Some(Type::Enum(TypeEnum::new(
            name,
            metadata.description.clone(),
            None,
            EnumTagType::External,
            variants,
            false,
        )))
    }

    fn untagged_enum(
        &self,
        name: NameBuilder,
        metadata: &crate::schemalet::SchemaletMetadata,
        proto_variants: &[ProtoVariant],
    ) -> Type {
        // 6/27/2025
        // I need to figure out decent names for the variants... and I'm a
        // little unhappy that I may not know the names of the types yet. I
        // wonder if I'm going to end up needing another pass in here, but I'm
        // trying to put that out of my mind so that I can just get something
        // done.

        // Variants names should all be "of a kind" meaning that we shouldn't
        // mix and match. We'll first try to apply names from the schemas (i.e.
        // the tiles and JSON paths); then we'll use type classification
        // (integer, boolean, object), and finally we'll fall back on
        // Variant{n}.

        let variant_names = if let Some(title_names) = proto_variants
            .iter()
            .map(|proto| {
                proto
                    .name
                    .clone()
                    .or_else(|| proto.schemalet.metadata.title.clone())
            })
            .collect::<Option<Vec<_>>>()
        {
            todo!()
        } else if let Some(kind_names) = maybe_kind_names(proto_variants) {
            kind_names
        } else {
            (0..proto_variants.len())
                .map(|ii| format!("Variant{ii}"))
                .collect()
        };
        println!("{}", serde_json::to_string_pretty(&variant_names).unwrap());

        let variants = proto_variants
            .iter()
            .zip(variant_names)
            .map(|(proto, variant_name)| {
                let details = if let Some(struct_props) =
                    self.xxx_maybe_struct_props(&variant_name, &proto.id, &proto.schemalet)
                {
                    VariantDetails::Struct(struct_props)
                } else {
                    VariantDetails::Item(proto.id.clone())
                };

                EnumVariant {
                    rust_name: variant_name,
                    rename: None,
                    description: proto.description.clone(),
                    details,
                }
            })
            .collect::<Vec<_>>();

        Type::Enum(TypeEnum::new(
            name,
            metadata.description.clone(),
            None,
            EnumTagType::Untagged,
            variants,
            false,
        ))
    }

    fn xxx_maybe_struct_props(
        &self,
        variant_name: &str,
        id: &SchemaRef,
        schemalet: &CanonicalSchemalet,
    ) -> Option<Vec<StructProperty>> {
        // TODO somehow I need to know if this is a type that's going to have a
        // name.
        // TODO or we somehow defer that decision to the Typespace's finalize step?
        let object = schemalet.as_object()?;

        let result = self.convert_object(
            id,
            NameBuilder::Fixed(variant_name.to_string()),
            &schemalet.metadata,
            object,
        );
        let Type::Struct(struct_ty) = result.primary else {
            return None;
        };

        println!("{:#?}", struct_ty);

        Some(struct_ty.properties)
    }
}

fn maybe_kind_names(proto_variants: &[ProtoVariant]) -> Option<Vec<String>> {
    let type_list = proto_variants
        .iter()
        .map(|proto| proto.schemalet.get_type())
        .collect::<Option<Vec<_>>>()?;

    let type_set = type_list.iter().collect::<BTreeSet<_>>();

    (type_list.len() == type_set.len()).then(|| {
        type_list
            .into_iter()
            .map(|t| t.variant_name().to_string())
            .collect()
    })
}

#[derive(Clone, Debug, serde::Serialize)]
struct ProtoVariant<'a> {
    id: &'a SchemaRef,
    schemalet: &'a CanonicalSchemalet,
    /// A name from a part of the schema.
    name: Option<String>,
    /// A comment from a part of the schema.
    description: Option<String>,
}

struct ProtoVariantExternal<'a> {
    proto: Cow<'a, ProtoVariant<'a>>,
    kind: ProtoVariantExternalKind<'a>,
}

enum ProtoVariantExternalKind<'a> {
    Simple(String),
    Typed(String, &'a SchemaRef),
}
