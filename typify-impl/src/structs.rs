// Copyright 2022 Oxide Computer Company

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use schemars::schema::{
    InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
};

use crate::{
    enums::get_object,
    output::{OutputSpace, OutputSpaceMod},
    type_entry::{
        StructProperty, StructPropertyRename, StructPropertyState, TypeEntry, TypeEntryStruct,
        WrappedValue,
    },
    util::{get_type_name, metadata_description, recase, schema_is_named, Case},
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

            // Only particular additional properties are allowed.
            additional_properties @ Some(_) => {
                let sub_type_name = type_name.as_ref().map(|base| format!("{}_extra", base));
                let (map_type, _) = self.make_map(sub_type_name, additional_properties)?;
                let map_type_id = self.assign_type(map_type);
                let extra_prop = StructProperty {
                    name: "extra".to_string(),
                    rename: StructPropertyRename::Flatten,
                    state: StructPropertyState::Required,
                    description: None,
                    type_id: map_type_id,
                };

                properties.push(extra_prop);
                true
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

    pub(crate) fn make_map<'a>(
        &mut self,
        type_name: Option<String>,
        additional_properties: &Option<Box<Schema>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let (type_id, _) = match additional_properties {
            Some(schema) => {
                let sub_type_name = match type_name {
                    Some(name) => Name::Suggested(format!("{}Extra", name)),
                    None => Name::Unknown,
                };
                self.id_for_schema(sub_type_name, schema)?
            }

            None => self.id_for_schema(Name::Unknown, &Schema::Bool(true))?,
        };

        Ok((TypeEntryDetails::Map(type_id).into(), &None))
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

    /// This handles the case where an allOf is used to effect inheritance: the
    /// subschemas consist of one or more "super classes" that have names with
    /// a final, anonymous object.
    ///
    /// ```text
    /// "allOf": [
    ///     { "$ref": "#/definitions/SuperClass" },
    ///     { "type": "object", "properties": { "prop_a": .., "prop_b": .. }}
    /// ]
    /// ```
    ///
    /// This turns into a struct of this form:
    /// ```compile_fail
    /// struct MyType {
    ///     #[serde(flatten)]
    ///     super_class: SuperClass,
    ///     prop_a: (),
    ///     prop_b: (),
    /// }
    /// ```
    ///
    /// Note that the super class member names are derived from the type and
    /// are flattened into the struct; the subclass properties are simply
    /// included alongside.
    pub(crate) fn maybe_all_of_subclass(
        &mut self,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        assert!(subschemas.len() > 1);

        // Split the subschemas into named (superclass) and unnamed (subclass)
        // schemas.
        let mut named = Vec::new();
        let mut unnamed = Vec::new();
        for schema in subschemas {
            match schema_is_named(schema) {
                Some(name) => named.push((schema, name)),
                None => unnamed.push(schema),
            }
        }

        // We required exactly one unnamed subschema for this special case.
        // Note that zero unnamed subschemas would be trivial to handle, but
        // the generic case already does so albeit slightly differently.
        if unnamed.len() != 1 {
            return None;
        }

        // Get the object validation (or fail to match this special case).
        let unnamed_schema = unnamed.first()?;
        let validation = match unnamed_schema {
            Schema::Object(SchemaObject {
                metadata: _,
                instance_type: Some(SingleOrVec::Single(single)),
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: None,
                string: None,
                array: None,
                object: Some(validation),
                reference: None,
                extensions: _,
            }) if single.as_ref() == &InstanceType::Object => Some(validation),
            _ => None,
        }?;
        let tmp_type_name = get_type_name(&type_name, metadata);
        let (unnamed_properties, deny) = self.struct_members(tmp_type_name, validation).ok()?;

        let named_properties = named
            .iter()
            .map(|(schema, property_name)| {
                let (type_id, metadata) = self.id_for_schema(type_name.clone(), schema)?;
                let (name, _) = recase(property_name, Case::Snake);
                Ok(StructProperty {
                    name,
                    rename: StructPropertyRename::Flatten,
                    state: StructPropertyState::Required,
                    description: metadata_description(metadata),
                    type_id,
                })
            })
            .collect::<Result<Vec<_>>>()
            .ok()?;

        Some(TypeEntryStruct::from_metadata(
            self,
            type_name,
            metadata,
            named_properties
                .into_iter()
                .chain(unnamed_properties.into_iter())
                .collect(),
            deny,
        ))
    }

    /// This handles the case where an allOf is used to denote constraints.
    /// Currently we just look for a referenced type, which may be "closed"
    /// (i.e. no unknown types permitted) and an explicit type. The latter must
    /// be a subset of the former and compatible from the perspective of JSON
    /// Schema validation (that is to say, it must be "open" or must fully
    /// cover the named type).
    ///
    /// ```text
    /// "allOf": [
    ///     { "$ref": "#/definitions/SomeType" },
    ///     { "type": "object", "properties": { "prop_a": .., "prop_b": .. }}
    /// ]
    /// ```
    ///
    /// Types such as these should be treated as the named type along with
    /// constraints. What do we do to enforce these constraints or communicate
    /// them to the consumer? Nothing!
    ///
    /// What could we do? We could add a custom `serialize_with` /
    /// `deserialize_with` functions to validate the constrains in and out.
    /// Or we could introduce a newtype wrapper to enforce those constraints.
    pub(crate) fn maybe_all_of_constraints(
        &mut self,
        type_name: Name,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        assert!(subschemas.len() > 1);

        // Split the subschemas into named (superclass) and unnamed (subclass)
        // schemas.
        let mut named = Vec::new();
        let mut unnamed = Vec::new();
        for schema in subschemas {
            match schema_is_named(schema) {
                Some(name) => named.push((schema, name)),
                None => unnamed.push(schema),
            }
        }

        // We required exactly one named subschema and at least one unnamed
        // subschema.
        if unnamed.len() != 1 && !named.is_empty() {
            return None;
        }

        let (named_schema, _) = named.first()?;

        let (_, _, validation) = get_object(Name::Unknown, named_schema, &self.definitions)?;

        if validation.additional_properties.as_ref().map(Box::as_ref) != Some(&Schema::Bool(false))
        {
            return None;
        }

        unnamed
            .into_iter()
            .all(|constraint_schema| is_obj_subset(validation, constraint_schema))
            .then_some(())?;

        let (type_entry, _) = self.convert_schema(type_name, named_schema).ok()?;
        Some(type_entry)
    }
}

fn is_obj_subset(validation: &ObjectValidation, constraint_schema: &Schema) -> bool {
    if let Schema::Object(SchemaObject {
        object: Some(obj), ..
    }) = constraint_schema
    {
        // TODO there's a lot more we could do to determine whether this is a
        // subset including inspection of the property types and confirming
        // that either all properties are covered or that the constraint object
        // is "open".
        obj.properties
            .keys()
            .all(|key| validation.properties.contains_key(key))
    } else {
        false
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
        (StructPropertyState::Optional, TypeEntryDetails::Array(_)) => {
            serde_options.push(quote! { default });
            serde_options.push(quote! { skip_serializing_if = "Vec::is_empty" });
            DefaultFunction::Default
        }
        (StructPropertyState::Optional, TypeEntryDetails::Map(_)) => {
            serde_options.push(quote! { default });
            serde_options
                .push(quote! { skip_serializing_if = "std::collections::HashMap::is_empty" });
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
        (Some(TypeEntryDetails::Array(_)), None) => StructPropertyState::Optional,
        (Some(TypeEntryDetails::Map(_)), None) => StructPropertyState::Optional,
        (Some(TypeEntryDetails::Unit), None) => StructPropertyState::Optional,
        (_, None) => StructPropertyState::Required,

        // Default specified is the same as the implicit default: null
        (Some(TypeEntryDetails::Option(_)), Some(serde_json::Value::Null)) => {
            StructPropertyState::Optional
        }
        // Default specified is the same as the implicit default: []
        (Some(TypeEntryDetails::Array(_)), Some(serde_json::Value::Array(a))) if a.is_empty() => {
            StructPropertyState::Optional
        }
        // Default specified is the same as the implicit default: {}
        (Some(TypeEntryDetails::Map(_)), Some(serde_json::Value::Object(m))) if m.is_empty() => {
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
        things: std::collections::HashMap<String, serde_json::Value>,
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
        assert_eq!(
            output,
            "std::collections::HashMap<String,serde_json::Value>"
        );
    }
}
