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
                            details: VariantDetails::Simple,
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
        // the tiles and paths); then we'll use type classification (integer,
        // boolean, object), and finally we'll fall back on Variant{n}.

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
            .map(|(proto, name)| {
                let details =
                    if let Some(struct_props) = self.xxx_maybe_struct_props(&proto.schemalet) {
                        VariantDetails::Struct(struct_props)
                    } else {
                        VariantDetails::Item(proto.id.clone())
                    };

                EnumVariant {
                    rust_name: name,
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
        schemalet: &CanonicalSchemalet,
    ) -> Option<Vec<StructProperty>> {
        // TODO somehow I need to know if this is a type that's going to have a
        // name.
        // TODO or we somehow defer that decision to the Typespace's finalize step?
        let object = schemalet.as_object()?;

        let typ = self.convert_object(NameBuilder::Unset, &schemalet.metadata, object);
        let Type::Struct(struct_ty) = typ else {
            return None;
        };

        println!("{:#?}", struct_ty);

        Some(struct_ty.properties)
    }
}

fn maybe_kind_names(proto_variants: &[ProtoVariant]) -> Option<Vec<String>> {
    let xxx = proto_variants
        .iter()
        .map(|proto| proto.schemalet.get_type())
        .collect::<Option<Vec<_>>>()?;

    let yyy = xxx.iter().collect::<BTreeSet<_>>();

    (xxx.len() == yyy.len()).then(|| {
        xxx.into_iter()
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
