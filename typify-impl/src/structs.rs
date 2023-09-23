// Copyright 2022 Oxide Computer Company

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use schemars::schema::{InstanceType, Metadata, ObjectValidation, Schema, SchemaObject};

use crate::{
    output::{OutputSpace, OutputSpaceMod},
    type_entry::{
        StructProperty, StructPropertyRename, StructPropertyState, TypeEntry, TypeEntryStruct,
        WrappedValue,
    },
    util::{get_type_name, metadata_description, recase, Case},
    Name, Result, TypeEntryDetails, TypeId, TypeSpace,
};

impl TypeSpace {
    pub(crate) fn struct_members(
        &mut self,
        type_name: Option<String>,
        validation: &ObjectValidation,
    ) -> Result<(Vec<StructProperty>, bool)> {
        // These are the fields we don't currently handle
        //assert!(validation.max_properties.is_none());
        //assert!(validation.min_properties.is_none());
        //assert!(validation.pattern_properties.is_empty());
        //assert!(validation.property_names.is_none());

        let mut properties = validation
            .properties
            .iter()
            .map(|(name, ty)| {
                // Generate a name we can use for the type of this property
                // should there not be a one defined in the schema.
                let sub_type_name = type_name
                    .as_ref()
                    .map(|base| format!("{}_{}", base, name.to_snake_case()));
                self.struct_property(sub_type_name, &validation.required, name, ty)
            })
            .collect::<Result<Vec<_>>>()?;

        // Sort parameters by name to ensure a deterministic result.
        properties.sort_by(|a, b| a.name.cmp(&b.name));

        // If there are additional properties tack them on, flattened, at the
        // end. Note that a `None` value for additional_properties is
        // equivalent to the permissive schema (Schema::Bool(true)) for reasons
        // best known to the JSON Schema authors.
        let deny_unknown_fields = match &validation.additional_properties {
            // No additional properties allowed; we'll tag the struct with
            // #[serde(deny_unknown_fields)]
            Some(a) if a.as_ref() == &Schema::Bool(false) => true,

            // We have a permissive schema so all additional properties are
            // allowed (None is equivalent to the permissive schema).
            Some(a)
                if matches!(
                    a.as_ref(),
                    Schema::Bool(true)
                        | Schema::Object(SchemaObject {
                            metadata: _,
                            instance_type: None,
                            format: None,
                            enum_values: None,
                            const_value: None,
                            subschemas: None,
                            number: None,
                            string: None,
                            array: None,
                            object: None,
                            reference: None,
                            extensions: _,
                        })
                ) =>
            {
                false
            }

            None => false,

            // Only particular additional properties are allowed. Note that
            // #[serde(deny_unknown_fields)] is incompatible with
            // #[serde(flatten)] so we allow them even though that doesn't seem
            // quite right.
            additional_properties @ Some(_) => {
                let sub_type_name = type_name.as_ref().map(|base| format!("{}_extra", base));
                let map_type = self.make_map(
                    sub_type_name,
                    &validation.property_names,
                    additional_properties,
                )?;
                let map_type_id = self.assign_type(map_type);
                let extra_prop = StructProperty {
                    name: "extra".to_string(),
                    rename: StructPropertyRename::Flatten,
                    state: StructPropertyState::Required,
                    description: None,
                    type_id: map_type_id,
                };

                properties.push(extra_prop);
                false
            }
        };

        Ok((properties, deny_unknown_fields))
    }

    pub(crate) fn struct_property(
        &mut self,
        type_name: Option<String>,
        required: &schemars::Set<String>,
        prop_name: &str,
        schema: &schemars::schema::Schema,
    ) -> Result<StructProperty> {
        let sub_type_name = match type_name {
            Some(name) => Name::Suggested(name),
            None => Name::Unknown,
        };
        let (mut type_id, metadata) = self.id_for_schema(sub_type_name, schema)?;

        let state = if required.contains(prop_name) {
            StructPropertyState::Required
        } else {
            // We can use serde's `default` and `skip_serializing_if`
            // construction for options, arrays, and maps--i.e. properties that
            // have an "intrinsic" default value. We can also apply `default`
            // to properties for which there's a default value present. (We
            // could also skip serializing them when they match the default
            // value, but that seems both uncommon and more trouble than it's
            // worth.) Properties with no intrinsic or explicit default value
            // are converted to an Option<T> type in order to represent the
            // field as non-required.
            //
            // Note that arrays, maps, and even options may have default values
            // that differ from the intrinsic default values. That is to say,
            // they may have defaults other than `[]`, `{}`, and `null`
            // respectively. This affects the eventual generated code, but not
            // the internal representation produced here.
            //
            // We will validate the default values, but not here: the type
            // space is not yet in a consistent state with regard to references
            // so we cannot reliably resolve references here.
            match has_default(
                self,
                &type_id,
                metadata.as_ref().and_then(|m| m.default.as_ref()),
            ) {
                StructPropertyState::Required => {
                    type_id = self.id_to_option(&type_id);
                    StructPropertyState::Optional
                }
                other => other,
            }
        };

        let (name, rename) = recase(prop_name, Case::Snake);
        let rename = match rename {
            Some(old_name) => StructPropertyRename::Rename(old_name),
            None => StructPropertyRename::None,
        };

        Ok(StructProperty {
            name,
            rename,
            state,
            description: metadata_description(metadata),
            type_id,
        })
    }

    pub(crate) fn make_map(
        &mut self,
        type_name: Option<String>,
        property_names: &Option<Box<Schema>>,
        additional_properties: &Option<Box<Schema>>,
    ) -> Result<TypeEntry> {
        let key_id = match property_names.as_deref() {
            Some(Schema::Bool(true)) | None => self.assign_type(TypeEntryDetails::String.into()),

            // TODO this would correspond to an empty object: an object with
            // no legal property values.
            Some(Schema::Bool(false)) => todo!(),

            Some(Schema::Object(obj)) => {
                let key_type_name = match &type_name {
                    Some(name) => Name::Suggested(format!("{}Key", name)),
                    None => Name::Unknown,
                };
                self.id_for_schema_string(key_type_name, obj)?
            }
        };

        let (value_id, _) = match additional_properties {
            Some(value_schema) => {
                let value_type_name = match &type_name {
                    Some(name) => Name::Suggested(format!("{}Value", name)),
                    None => Name::Unknown,
                };
                self.id_for_schema(value_type_name, value_schema)?
            }

            None => self.id_for_schema(Name::Unknown, &Schema::Bool(true))?,
        };

        Ok(TypeEntryDetails::Map(key_id, value_id).into())
    }

    /// Perform a schema conversion for a type that must be string-like.
    pub(crate) fn id_for_schema_string(
        &mut self,
        type_name: Name,
        schema_obj: &SchemaObject,
    ) -> Result<TypeId> {
        match schema_obj {
            // If the schema has no subschemas or references, fill in the
            // string instance_type if none is present.
            SchemaObject {
                instance_type: None,
                subschemas: None,
                reference: None,
                ..
            } => {
                let schema = Schema::Object(SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    ..schema_obj.clone()
                });
                Ok(self.id_for_schema(type_name, &schema)?.0)
            }

            // TODO if and when we perform merging of schemas we could wrap the
            // schema in an { allOf: [{ type: string }, <schema> ] }
            _ => {
                let schema = Schema::Object(schema_obj.clone());
                Ok(self.id_for_schema(type_name, &schema)?.0)
            }
        }
    }

    /// This is used by both any-of and all-of subschema processing. This
    /// produces a struct type whose members are the subschemas (flattened).
    ///
    /// ```ignore
    /// struct Name {
    ///     #[serde(flatten)]
    ///     schema1: Schema1Type,
    ///     #[serde(flatten)]
    ///     schema2: Schema2Type
    ///     ...
    /// }
    /// ```
    ///
    /// The only difference between any-of and all-of is that where the latter
    /// has type T_N for each member of the struct, the former has Option<T_N>.
    pub(crate) fn flattened_union_struct<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        subschemas: &[Schema],
        optional: bool,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let properties = subschemas
            .iter()
            .enumerate()
            .map(|(idx, schema)| {
                let type_name = match get_type_name(&type_name, metadata) {
                    Some(name) => Name::Suggested(format!("{}Subtype{}", name, idx)),
                    None => Name::Unknown,
                };

                let (mut type_id, _) = self.id_for_schema(type_name, schema)?;
                if optional {
                    type_id = self.id_to_option(&type_id);
                }

                // TODO we need a reasonable name that could be derived
                // from the name of the type
                let name = format!("subtype_{}", idx);

                Ok(StructProperty {
                    name,
                    rename: StructPropertyRename::Flatten,
                    state: if optional {
                        StructPropertyState::Optional
                    } else {
                        StructPropertyState::Required
                    },
                    description: None,
                    type_id,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok((
            TypeEntryStruct::from_metadata(self, type_name, metadata, properties, false),
            metadata,
        ))
    }
}

pub(crate) enum DefaultFunction {
    None,
    Default,
    Custom(String),
}

/// Generate the serde attribute parameters for the given property.
///
/// This may include a default value that requires a generated function to
/// produce it. In such a case, that function will be added to the OutputSpace.
///
/// Note that if we have several serde attribute parameters, they could each
/// appear in their own attribute. We choose to condense them for the sake of
/// legibility.
pub(crate) fn generate_serde_attr(
    type_name: &str,
    prop_name: &str,
    naming: &StructPropertyRename,
    state: &StructPropertyState,
    prop_type: &TypeEntry,
    type_space: &TypeSpace,
    output: &mut OutputSpace,
) -> (TokenStream, DefaultFunction) {
    let mut serde_options = Vec::new();
    match naming {
        StructPropertyRename::Rename(s) => serde_options.push(quote! { rename = #s }),
        StructPropertyRename::Flatten => serde_options.push(quote! { flatten }),
        StructPropertyRename::None => (),
    }

    let default_fn = match (state, &prop_type.details) {
        (StructPropertyState::Optional, TypeEntryDetails::Option(_)) => {
            serde_options.push(quote! { default });
            serde_options.push(quote! { skip_serializing_if = "Option::is_none" });
            DefaultFunction::Default
        }
        (StructPropertyState::Optional, TypeEntryDetails::Vec(_)) => {
            serde_options.push(quote! { default });
            serde_options.push(quote! { skip_serializing_if = "Vec::is_empty" });
            DefaultFunction::Default
        }
        (StructPropertyState::Optional, TypeEntryDetails::Map(key_id, value_id)) => {
            serde_options.push(quote! { default });

            let key_ty = type_space
                .id_to_entry
                .get(key_id)
                .expect("unresolved key type id for map");
            let value_ty = type_space
                .id_to_entry
                .get(value_id)
                .expect("unresolved value type id for map");

            if key_ty.details == TypeEntryDetails::String
                && value_ty.details == TypeEntryDetails::JsonValue
            {
                serde_options.push(quote! {
                    skip_serializing_if = "serde_json::Map::is_empty"
                });
            } else {
                serde_options.push(quote! {
                    skip_serializing_if = "std::collections::HashMap::is_empty"
                });
            }
            DefaultFunction::Default
        }
        (StructPropertyState::Optional, _) => {
            serde_options.push(quote! { default });
            DefaultFunction::Default
        }

        (StructPropertyState::Default(WrappedValue(value)), _) => {
            let (fn_name, default_fn) =
                prop_type.default_fn(value, type_space, type_name, prop_name);
            serde_options.push(quote! { default = #fn_name });

            if let Some(default_fn) = default_fn {
                output.add_item(OutputSpaceMod::Defaults, type_name, default_fn);
            }
            DefaultFunction::Custom(fn_name)
        }

        (StructPropertyState::Required, _) => DefaultFunction::None,
    };

    let serde = if serde_options.is_empty() {
        quote! {}
    } else {
        quote! {
            #[serde( #(#serde_options),*)]
        }
    };

    (serde, default_fn)
}

/// See if this type is a type that we can omit with a serde directive; note
/// that the type id lookup will fail only for references (and only during
/// initial reference processing).
fn has_default(
    type_space: &mut TypeSpace,
    type_id: &TypeId,
    default: Option<&serde_json::Value>,
) -> StructPropertyState {
    // This lookup can fail in the scenario where a struct (or struct
    // variant) member is optional and the type of that optional member is a
    // reference to a type that has not yet been converted. This is fine: those
    // are necessarily named types and not raw options, arrays, maps, or units.
    match (
        type_space
            .id_to_entry
            .get(type_id)
            .map(|type_entry| &type_entry.details),
        default,
    ) {
        // No default specified.
        (Some(TypeEntryDetails::Option(_)), None) => StructPropertyState::Optional,
        (Some(TypeEntryDetails::Vec(_)), None) => StructPropertyState::Optional,
        (Some(TypeEntryDetails::Map(..)), None) => StructPropertyState::Optional,
        (Some(TypeEntryDetails::Unit), None) => StructPropertyState::Optional,
        (_, None) => StructPropertyState::Required,

        // Default specified is the same as the implicit default: null
        (Some(TypeEntryDetails::Option(_)), Some(serde_json::Value::Null)) => {
            StructPropertyState::Optional
        }
        // Default specified is the same as the implicit default: []
        (Some(TypeEntryDetails::Vec(_)), Some(serde_json::Value::Array(a))) if a.is_empty() => {
            StructPropertyState::Optional
        }
        // Default specified is the same as the implicit default: {}
        (Some(TypeEntryDetails::Map(..)), Some(serde_json::Value::Object(m))) if m.is_empty() => {
            StructPropertyState::Optional
        }
        // Default specified is the same as the implicit default: false
        (Some(TypeEntryDetails::Boolean), Some(serde_json::Value::Bool(false))) => {
            StructPropertyState::Optional
        }
        // Default specified is the same as the implicit default: 0
        (Some(TypeEntryDetails::Integer(_)), Some(serde_json::Value::Number(n)))
            if n.as_u64() == Some(0) =>
        {
            StructPropertyState::Optional
        }
        // Default specified is the same as the implicit default: 0.0
        (Some(TypeEntryDetails::Integer(_)), Some(serde_json::Value::Number(n)))
            if n.as_f64() == Some(0.0) =>
        {
            StructPropertyState::Optional
        }
        // Default specified is the same as the implicit default: ""
        (Some(TypeEntryDetails::String), Some(serde_json::Value::String(s))) if s.is_empty() => {
            StructPropertyState::Optional
        }

        // This is a reference that will resolve to this type id later.
        (None, Some(default)) => StructPropertyState::Default(WrappedValue(default.clone())),
        // All other types as well as types with intrinsic defaults that have
        // been explicitly overridden.
        (Some(_), Some(default)) => StructPropertyState::Default(WrappedValue(default.clone())),
    }
}

#[cfg(test)]
mod tests {
    use schema::Schema;
    use schemars::JsonSchema;
    use serde::Serialize;

    use crate::{test_util::validate_output, Name, TypeSpace};

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(deny_unknown_fields)]
    struct SimpleStruct {
        alpha: u32,
        bravo: String,
        charlie: Vec<(String, u32)>,
        delta: Option<String>,
        echo: Option<(u32, String)>,
    }

    #[test]
    fn test_simple_struct() {
        validate_output::<SimpleStruct>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    struct LessSimpleStruct {
        thing: SimpleStruct,
        things: Vec<SimpleStruct>,
    }

    #[test]
    fn test_less_simple_struct() {
        validate_output::<LessSimpleStruct>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    struct SomeMaps {
        strings: std::collections::HashMap<String, String>,
        things: serde_json::Map<String, serde_json::Value>,
    }

    #[test]
    fn test_some_maps() {
        validate_output::<SomeMaps>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(deny_unknown_fields)]
    struct FlattenStuff {
        number: i32,
        #[serde(flatten)]
        extra: std::collections::HashMap<String, String>,
    }

    #[test]
    fn test_flatten_stuff() {
        validate_output::<FlattenStuff>();
    }

    #[test]
    fn test_default_field() {
        #[allow(dead_code)]
        #[derive(Serialize, JsonSchema, Schema)]
        #[serde(deny_unknown_fields)]
        struct DefaultField {
            #[serde(default)]
            number: i32,
        }

        validate_output::<DefaultField>();
    }

    #[test]
    fn test_object_no_validation() {
        let schema = schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::Object.into()),
            ..Default::default()
        });

        let mut type_space = TypeSpace::default();
        let (ty, _) = type_space.convert_schema(Name::Unknown, &schema).unwrap();
        let output = ty.type_name(&type_space).replace(" ", "");
        assert_eq!(output, "serde_json::Map<String,serde_json::Value>");
    }
}
