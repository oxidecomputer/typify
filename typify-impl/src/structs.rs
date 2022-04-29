// Copyright 2021 Oxide Computer Company

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use schemars::schema::{
    InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
};

use crate::{
    type_entry::{
        SerdeNaming, SerdeRules, StructProperty, TypeEntry, TypeEntryStruct, ValidDefault,
    },
    util::{get_type_name, metadata_description, recase, schema_is_named},
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
                let prop_name = name.to_case(Case::Snake);
                let sub_type_name = type_name
                    .as_ref()
                    .map(|base| format!("{}_{}", base, prop_name));
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
                    serde_naming: SerdeNaming::Flatten,
                    serde_rules: SerdeRules::None,
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

        let serde_rules = if required.contains(prop_name) {
            SerdeRules::None
        } else {
            // We can use serde's `default` and `skip_serializing_if`
            // construction for options, arrays, and maps. Properties the have
            // a default value can have the `default` property applied (we
            // could also skip serializing them if they had the same value as
            // the default but that hardly seems worth checking). Otherwise we
            // need to turn this property into an option type in order to
            // represent the field as non-required.
            //
            // Note that arrays, maps, and even options may have default values
            // that differ from the ... er ... default default values. That is
            // they may have defaults other than `[]`, `{}`, and `null`
            // respectively. This affects the eventual generated code, but not
            // the internal representation produced here.
            //
            // TODO I think we should skip this, tag the prop as optional, not
            // create an option type, and deal with it later.
            // TODO we can check that there **is** a default at this point, but
            // **not** that's it's valid because references may not yet have
            // been resolved.
            match has_default(
                self,
                &type_id,
                metadata.as_ref().and_then(|m| m.default.as_ref()),
            )? {
                SerdeRules::None => {
                    type_id = self.id_to_option(&type_id);
                    SerdeRules::ImplicitDefault
                }
                other => other,
            }
        };

        let (name, rename) = recase(prop_name.to_string(), Case::Snake);
        let serde_naming = match rename {
            Some(old_name) => SerdeNaming::Rename(old_name),
            None => SerdeNaming::None,
        };

        Ok(StructProperty {
            name,
            serde_naming,
            serde_rules,
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
                let type_name = match get_type_name(&type_name, metadata, Case::Pascal) {
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
                    serde_naming: SerdeNaming::Flatten,
                    serde_rules: if optional {
                        SerdeRules::ImplicitDefault
                    } else {
                        SerdeRules::None
                    },
                    description: None,
                    type_id,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok((
            TypeEntryStruct::from_metadata(type_name, metadata, properties, false).into(),
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
        let tmp_type_name = get_type_name(&type_name, metadata, Case::Pascal);
        let (unnamed_properties, deny) = self.struct_members(tmp_type_name, validation).ok()?;

        let named_properties = named
            .iter()
            .map(|(schema, property_name)| {
                let (type_id, metadata) = self.id_for_schema(type_name.clone(), schema)?;
                Ok(StructProperty {
                    name: property_name.to_case(Case::Snake),
                    serde_naming: SerdeNaming::Flatten,
                    serde_rules: SerdeRules::None,
                    description: metadata_description(metadata),
                    type_id,
                })
            })
            .collect::<Result<Vec<_>>>()
            .ok()?;

        Some(
            TypeEntryStruct::from_metadata(
                type_name,
                metadata,
                named_properties
                    .into_iter()
                    .chain(unnamed_properties.into_iter())
                    .collect(),
                deny,
            )
            .into(),
        )
    }
}

pub(crate) fn output_struct_property(
    prop: &StructProperty,
    type_space: &TypeSpace,
    make_pub: bool,
) -> TokenStream {
    let name = format_ident!("{}", prop.name);
    let doc = match &prop.description {
        Some(s) => quote! {#[doc = #s]},
        None => quote! {},
    };

    let prop_type = type_space.id_to_entry.get(&prop.type_id).unwrap();
    let type_name = prop_type.type_ident(type_space, false);
    let pub_token = if make_pub {
        quote! { pub }
    } else {
        quote! {}
    };

    // TODO add the default_fn to the type_space... somehow
    let (serde, default_fn) =
        generate_serde_attr(&prop.serde_naming, &prop.serde_rules, prop_type, type_space);
    quote! {
        #doc
        #serde
        #pub_token #name: #type_name,
    }
}

fn generate_serde_attr(
    serde_naming: &SerdeNaming,
    serde_rules: &SerdeRules,
    prop_type: &TypeEntry,
    type_space: &TypeSpace,
) -> (TokenStream, Option<TokenStream>) {
    let mut serde_options = Vec::new();
    match serde_naming {
        SerdeNaming::Rename(s) => serde_options.push(quote! { rename = #s }),
        SerdeNaming::Flatten => serde_options.push(quote! { flatten }),
        SerdeNaming::None => (),
    }

    let default_fn = match (serde_rules, &prop_type.details) {
        (SerdeRules::ImplicitDefault, TypeEntryDetails::Option(_)) => {
            serde_options.push(quote! { default });
            serde_options.push(quote! { skip_serializing_if = "Option::is_none" });
            None
        }
        (SerdeRules::ImplicitDefault, TypeEntryDetails::Array(_)) => {
            serde_options.push(quote! { default });
            serde_options.push(quote! { skip_serializing_if = "Vec::is_empty" });
            None
        }
        (SerdeRules::ImplicitDefault, TypeEntryDetails::Map(_)) => {
            serde_options.push(quote! { default });
            serde_options
                .push(quote! { skip_serializing_if = "std::collections::HashMap::is_empty" });
            None
        }
        (SerdeRules::ImplicitDefault, _) => {
            serde_options.push(quote! { default });
            None
        }
        (SerdeRules::None, _) => None,
        (SerdeRules::ExplicitDefault(value), _) => {
            let (fn_name, default_fn) = prop_type.default_fn(value, type_space);
            serde_options.push(quote! { default = #fn_name });
            default_fn
        }
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

/// See if this type is one

/// See if this type is a type that we can omit with a serde directive; note
/// that the type id lookup will fail only for references (and only during
/// initial reference processing).
// TODO this requires updating to deal with defaults
fn has_default(
    type_space: &mut TypeSpace,
    type_id: &TypeId,
    default: Option<&serde_json::Value>,
) -> Result<SerdeRules> {
    // TODO This lookup can fail in the scenario where a struct (or struct
    // variant) member is optional and the type of that optional member is a
    // reference to a type that has not yet been converted. This means that
    // types that match that description may not properly have default member
    // values translated properly.
    //
    // The proper way to handle this is merely to tag a member as optional,
    // include the default value, and validate the default values after all
    // references have been properly resolved. This is pretty painful, because
    // we'd need to modify members to insert Option<T> types if T turns out
    // to make this not a member we can tag with #[serde(default)]
    let type_entry = if let Some(type_entry) = type_space.id_to_entry.get(type_id) {
        type_entry
    } else {
        return Ok(SerdeRules::None);
    };

    if let Some(default) = default {
        let x = type_entry.validate_default(default, type_space)?;
        match x {
            ValidDefault::Intrinsic => Ok(SerdeRules::ImplicitDefault),
            ValidDefault::Specific => Ok(SerdeRules::ExplicitDefault(default.clone())),
            ValidDefault::Generic(g) => {
                type_space.defaults.insert(g);
                Ok(SerdeRules::ExplicitDefault(default.clone()))
            }
        }
    } else {
        match &type_entry.details {
            TypeEntryDetails::Option(_) | TypeEntryDetails::Array(_) | TypeEntryDetails::Map(_) => {
                Ok(SerdeRules::ImplicitDefault)
            }

            _ => Ok(SerdeRules::None),
        }
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
