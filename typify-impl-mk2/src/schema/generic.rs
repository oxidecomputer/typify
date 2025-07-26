use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{
    schema::util::{self, ObjectOrBool},
    schemalet::{
        SchemaRef, Schemalet, SchemaletDetails, SchemaletMetadata, SchemaletValue,
        SchemaletValueArray, SchemaletValueObject,
    },
    Error, ErrorKind,
};

pub type GenericSchemaOrBool = ObjectOrBool<GenericSchema>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GenericSimpleTypes {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum GenericType {
    Single(GenericSimpleTypes),
    Array(Vec<GenericSimpleTypes>),
}

#[derive(Deserialize, Serialize)]
pub struct GenericSchema {
    // Schema metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<::serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<::serde_json::Value>,
    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub const_: Option<::serde_json::Value>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<::serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<GenericType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    //     #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    //     pub read_only: Option<bool>,
    //     #[serde(rename = "writeOnly", skip_serializing_if = "Option::is_none")]
    //     pub write_only: Option<bool>,

    // Schema document metadata
    //     #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    //     schema: Option<String>,
    #[serde(rename = "$anchor", skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    //     #[serde(rename = "$comment", skip_serializing_if = "Option::is_none")]
    //     comment: Option<String>,
    //     #[serde(rename = "$defs", skip_serializing_if = "Option::is_none")]
    //     defs: Option<BTreeMap<String, SchemaOrBool>>,
    #[serde(rename = "$dynamicAnchor", skip_serializing_if = "Option::is_none")]
    pub dynamic_anchor: Option<String>,
    #[serde(rename = "$dynamicRef", skip_serializing_if = "Option::is_none")]
    pub dynamic_ref: Option<String>,
    //     #[serde(rename = "$id", skip_serializing_if = "Option::is_none")]
    //     id: Option<String>,
    #[serde(rename = "$recursiveAnchor", skip_serializing_if = "Option::is_none")]
    pub recursive_anchor: Option<String>,
    #[serde(rename = "$recursiveRef", skip_serializing_if = "Option::is_none")]
    pub recursive_ref: Option<String>,
    //     #[serde(rename = "$vocabulary", skip_serializing_if = "Option::is_none")]
    //     vocabulary: Option<BTreeMap<String, bool>>,
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     definitions: Option<BTreeMap<String, SchemaOrBool>>,

    // Reference
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    // Object
    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<GenericSchemaOrBool>,
    #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<u64>,
    #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<u64>,
    #[serde(rename = "patternProperties", skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<BTreeMap<String, GenericSchemaOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, GenericSchemaOrBool>>,
    #[serde(rename = "propertyNames", skip_serializing_if = "Option::is_none")]
    pub property_names: Option<GenericSchemaOrBool>,
    #[serde(
        rename = "unevaluatedProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub unevaluated_properties: Option<GenericSchemaOrBool>,
    #[serde(rename = "dependentRequired", skip_serializing_if = "Option::is_none")]
    pub dependent_required: Option<BTreeMap<String, Vec<String>>>,
    #[serde(rename = "dependentSchemas", skip_serializing_if = "Option::is_none")]
    pub dependent_schemas: Option<BTreeMap<String, GenericSchemaOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<BTreeMap<String, GenericSchemaDependencies>>,

    // Array
    #[serde(flatten)]
    pub items: Option<GenericItems>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<GenericSchemaOrBool>,
    #[serde(rename = "maxContains", skip_serializing_if = "Option::is_none")]
    pub max_contains: Option<u64>,
    #[serde(rename = "minContains", skip_serializing_if = "Option::is_none")]
    pub min_contains: Option<u64>,

    #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(rename = "unevaluatedItems", skip_serializing_if = "Option::is_none")]
    pub unevaluated_items: Option<GenericSchemaOrBool>,

    // Number and integer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<serde_json::Number>,
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<serde_json::Number>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<serde_json::Number>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<serde_json::Number>,

    // String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,

    // Subschemas
    #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<GenericSchemaOrBool>>,
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<GenericSchemaOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<GenericSchemaOrBool>,
    #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<GenericSchemaOrBool>>,

    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    pub if_: Option<GenericSchemaOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub then: Option<GenericSchemaOrBool>,
    #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
    pub else_: Option<GenericSchemaOrBool>,
    // TODO: ???
    // #[serde(rename = "contentEncoding", skip_serializing_if = "Option::is_none")]
    // pub content_encoding: Option<String>,
    // #[serde(rename = "contentMediaType", skip_serializing_if = "Option::is_none")]
    // pub content_media_type: Option<String>,
    // #[serde(rename = "contentSchema", skip_serializing_if = "Option::is_none")]
    // pub content_schema: Option<Box<GenericSchemaOrBool>>,
}

/// Model the pre- and post-2020-12 array schemas. Either we have a
/// `prefixItems` array (with an optional `items` schema), a (singular) `items`
/// schema, or an `items` array (with an optional `additionalItems` schema).
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum GenericItems {
    Prefix {
        #[serde(rename = "prefixItems")]
        prefix_items: Vec<GenericSchemaOrBool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        items: Option<GenericSchemaOrBool>,
    },
    Single {
        items: GenericSchemaOrBool,
    },
    Additional {
        items: Vec<GenericSchemaOrBool>,
        #[serde(rename = "additionalItems", skip_serializing_if = "Option::is_none")]
        additional_items: Option<GenericSchemaOrBool>,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum GenericSchemaDependencies {
    Schema(GenericSchemaOrBool),
    Strings(Vec<String>),
}

pub trait ToGeneric<T> {
    fn to_generic(self) -> T;
}

impl<T, Generic> ToGeneric<BTreeMap<String, Generic>> for BTreeMap<String, T>
where
    T: ToGeneric<Generic>,
{
    fn to_generic(self) -> BTreeMap<String, Generic> {
        self.into_iter()
            .map(|(key, value)| (key, value.to_generic()))
            .collect()
    }
}

impl<T, Generic> ToGeneric<Option<Generic>> for Option<T>
where
    T: ToGeneric<Generic>,
{
    fn to_generic(self) -> Option<Generic> {
        self.map(ToGeneric::to_generic)
    }
}

impl<T, Generic> ToGeneric<Box<Generic>> for Box<T>
where
    T: ToGeneric<Generic>,
{
    fn to_generic(self) -> Box<Generic> {
        Box::new((*self).to_generic())
    }
}

impl<T, Generic> ToGeneric<Vec<Generic>> for Vec<T>
where
    T: ToGeneric<Generic>,
{
    fn to_generic(self) -> Vec<Generic> {
        self.into_iter().map(ToGeneric::to_generic).collect()
    }
}

impl<T, Generic> ToGeneric<ObjectOrBool<Generic>> for ObjectOrBool<T>
where
    T: ToGeneric<Generic>,
{
    fn to_generic(self) -> ObjectOrBool<Generic> {
        match self {
            ObjectOrBool::Bool(b) => ObjectOrBool::Bool(b),
            ObjectOrBool::Object(o) => ObjectOrBool::Object(o.to_generic()),
        }
    }
}

type WorkQueue<'a> = util::WorkQueue<'a, SchemaRef, GenericSchemaOrBool, Schemalet>;

impl GenericSchemaOrBool {
    pub fn to_schemalets<'a>(
        &'a self,
        work: &mut WorkQueue<'a>,
        id: SchemaRef,
    ) -> Result<(), Error> {
        match self {
            ObjectOrBool::Bool(value) => {
                let schemalet = if *value {
                    Schemalet::from_details(SchemaletDetails::Anything)
                } else {
                    Schemalet::from_details(SchemaletDetails::Nothing)
                };
                work.done(id, schemalet);
                Ok(())
            }
            ObjectOrBool::Object(schema) => schema.to_schemalets(work, id),
        }
    }
}

impl GenericSchema {
    pub fn to_schemalets<'a>(
        &'a self,
        work: &mut WorkQueue<'a>,
        id: SchemaRef,
    ) -> Result<(), Error> {
        let Self {
            title,
            description,
            examples,
            default,
            const_,
            enum_,
            deprecated,
            type_,
            format,
            anchor,
            dynamic_anchor,
            dynamic_ref,
            recursive_anchor,
            recursive_ref,
            ref_,
            additional_properties,
            max_properties,
            min_properties,
            pattern_properties,
            properties,
            property_names,
            unevaluated_properties,
            dependent_required,
            dependent_schemas,
            required,
            dependencies,
            items,
            max_items,
            min_items,
            contains,
            max_contains,
            min_contains,
            unique_items,
            unevaluated_items,
            minimum,
            maximum,
            multiple_of,
            exclusive_maximum,
            exclusive_minimum,
            pattern,
            max_length,
            min_length,
            all_of,
            any_of,
            not,
            one_of,
            if_,
            then,
            else_,
        } = self;

        let concrete_value = match type_ {
            Some(GenericType::Single(single_type)) => {
                Some(self.to_schemalet_for_type(work, &id, single_type)?)
            }
            Some(GenericType::Array(types)) => {
                let subtypes = types
                    .iter()
                    .map(|ty| {
                        let (sref, sout) = self.to_schemalet_for_type(work, &id, ty)?;
                        work.done(sref.clone(), Schemalet::from_details(sout));
                        Ok(sref)
                    })
                    .collect::<Result<_, _>>()?;
                let value_id = id.partial("value");
                let value = SchemaletDetails::ExclusiveOneOf(subtypes);
                Some((value_id, value))
            }
            None => None,
        };

        let all_of = Self::to_schemalet_subschemas(
            work,
            &id,
            "allOf",
            SchemaletDetails::AllOf,
            all_of.as_ref(),
        );
        let any_of = Self::to_schemalet_subschemas(
            work,
            &id,
            "anyOf",
            SchemaletDetails::AnyOf,
            any_of.as_ref(),
        );

        let subref = ref_.as_ref().map(|raw_ref| {
            let value_id = id.partial("$ref");
            let value = SchemaletDetails::RawRef(raw_ref.clone());
            (value_id, value)
        });
        let dynref = dynamic_ref.as_ref().map(|raw_dyn_ref| {
            assert!(raw_dyn_ref.starts_with("#"));
            let raw_dyn_fragment = &raw_dyn_ref[1..];
            let value_id = id.partial("$dynamicRef");
            let value = SchemaletDetails::RawDynamicRef(raw_dyn_fragment.to_string());
            (value_id, value)
        });

        let enum_values = enum_.as_ref().map(|values| {
            let enum_id = id.append("enum");
            let xxx = values
                .iter()
                .enumerate()
                .map(|(ii, value)| {
                    let value_id = enum_id.append(&ii.to_string());
                    let value = Schemalet::from_details(SchemaletDetails::Constant(value.clone()));
                    work.done(value_id.clone(), value);
                    value_id
                })
                .collect();
            let value = SchemaletDetails::ExclusiveOneOf(xxx);
            (enum_id, value)
        });

        let everything = [concrete_value, all_of, any_of, subref, dynref, enum_values]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let metadata = SchemaletMetadata {
            title: title.clone(),
            description: description.clone(),
            examples: examples.clone().unwrap_or_default(),
        };

        let details = match everything.len() {
            0 => SchemaletDetails::Anything,

            1 => everything.into_iter().next().unwrap().1,

            _ => {
                let subs = everything
                    .iter()
                    .map(|(schema_ref, _)| schema_ref)
                    .cloned()
                    .collect();
                for (sref, details) in everything {
                    work.done(sref, Schemalet::from_details(details));
                }
                // TODO 7/25/2025
                // This isn't quite right. We need to treat this kind of merge
                // differently than a general allOf merge because here we need
                // to consider unevaluatedItems and unevaluatedProperties.
                SchemaletDetails::AllOf(subs)
            }
        };
        let value = Schemalet::new(details, metadata);

        work.done(id, value);
        Ok(())
    }

    fn to_schemalet_for_type<'a>(
        &'a self,
        work: &mut util::WorkQueue<'a, SchemaRef, ObjectOrBool<GenericSchema>, Schemalet>,
        id: &SchemaRef,
        schema_type: &GenericSimpleTypes,
    ) -> Result<(SchemaRef, SchemaletDetails), Error> {
        let Self {
            title: _,
            description: _,
            examples: _,
            default: _,
            const_: _,
            enum_: _,
            deprecated: _,
            type_: _,
            format,
            anchor: _,
            dynamic_anchor: _,
            dynamic_ref: _,
            recursive_anchor: _,
            recursive_ref: _,
            ref_: _,

            // Objects
            additional_properties,
            max_properties,
            min_properties,
            pattern_properties,
            properties,
            property_names,
            unevaluated_properties,
            dependent_required,
            dependent_schemas,
            required,
            dependencies,

            // Arrays
            items,
            max_items,
            min_items,
            contains,
            max_contains,
            min_contains,
            unique_items,
            unevaluated_items,

            // Numbers
            minimum,
            maximum,
            multiple_of,
            exclusive_maximum,
            exclusive_minimum,
            pattern,
            max_length,
            min_length,
            all_of: _,
            any_of: _,
            not: _,
            one_of: _,
            if_: _,
            then: _,
            else_: _,
        } = self;
        match schema_type {
            GenericSimpleTypes::Array => {
                if contains.is_some() || min_contains.is_some() || max_contains.is_some() {
                    return Err(Error::unsupported_schema_construction(
                        id.id(),
                        "contains, min_contains, or max_contains",
                    ));
                }

                // TODO 7/21/2025
                // The `unevaluatedItems` property is intersting and I'm not
                // really set up to handle it properly. It's--effectively, not
                // precisely (I'm flying and don't have the spec
                // handy)--applied after evaluating adjacent properties. Today
                // I'm stuffing all "disjoint" elements into a big `allOf`
                // equivalent. But that's not quite right. I do need to apply
                // the merging logic of `allOf`, but there's special handling
                // of `unevaluatedItems` (and `unevaluatedProperties`) where we
                // need to assess what items experienced "evaluation". Weirdly,
                // this includes not just `prefixItems` and `items` (and
                // `additionalItems`), but also `contains`. There's a clear
                // improvement to be had--a new Schemalet representing the
                // various components of a schema--and then some more complex
                // consideration about the handling of the idea of "evaluation"
                if unevaluated_items.is_some() {
                    return Err(Error::unsupported_schema_construction(
                        id.id(),
                        "contains unevaluatedItems",
                    ));
                }
                let (items, prefix_items) = match &items {
                    Some(GenericItems::Prefix {
                        prefix_items,
                        items,
                    }) => {
                        let items = xxx_get_ref_for_opt(work, id, "items", items.as_ref());
                        let prefix_items =
                            xxx_get_ref_for_vec(work, id, "prefixItems", prefix_items);
                        (items, Some(prefix_items))
                    }
                    Some(GenericItems::Single { items }) => {
                        let items = xxx_get_ref(work, id, "items", items);
                        (Some(items), None)
                    }
                    // Some(GenericItems::Additional {
                    //     items,
                    //     additional_items,
                    // }) => (additional_items.as_ref(), Some(items)),
                    // None => (None, None),
                    _ => todo!(),
                };

                let schema_ref = id.partial("array");
                let ir = SchemaletDetails::Value(SchemaletValue::Array(SchemaletValueArray {
                    items,
                    prefix_items,
                    min_items: *min_items,
                    max_items: *max_items,
                    unique_items: *unique_items,
                }));
                Ok((schema_ref, ir))
            }

            GenericSimpleTypes::Boolean => {
                let schema_ref = id.partial("boolean");
                let details = SchemaletDetails::Value(SchemaletValue::Boolean);
                Ok((schema_ref, details))
            }

            GenericSimpleTypes::Integer => {
                let schema_ref = id.partial("integer");
                let ir = SchemaletDetails::Value(SchemaletValue::Integer {
                    minimum: minimum.clone(),
                    exclusive_minimum: exclusive_minimum.clone(),
                });
                Ok((schema_ref, ir))
            }
            //     let schema_ref = id.partial("integer");
            //     let ir = SchemaletDetails::Value(SchemaletValue::Integer {
            //         minimum: minimum,
            //         exclusive_minimum: exclusive_minimum,
            //     });
            //     Ok((schema_ref, ir))
            // }
            GenericSimpleTypes::Null => todo!(),
            GenericSimpleTypes::Number => {
                let schema_ref = id.partial("number");

                // Conversion from serde_json::Number to f64 should be infallible.
                let ir = SchemaletDetails::Value(SchemaletValue::Number {
                    minimum: minimum.as_ref().map(|n| n.as_f64().unwrap()),
                    exclusive_minimum: exclusive_minimum.as_ref().map(|n| n.as_f64().unwrap()),
                    maximum: maximum.as_ref().map(|n| n.as_f64().unwrap()),
                    exclusive_maximum: exclusive_maximum.as_ref().map(|n| n.as_f64().unwrap()),
                    multiple_of: multiple_of.as_ref().map(|n| n.as_f64().unwrap()),
                });
                Ok((schema_ref, ir))
            }
            GenericSimpleTypes::Object => {
                let props_id = id.append("properties");
                let properties = properties
                    .as_ref()
                    .map_or_else(Default::default, |properties| {
                        properties
                            .iter()
                            .map(|(prop_name, prop_schema)| {
                                let prop_id = props_id.append(prop_name);
                                work.push(prop_id.id(), prop_schema);
                                (prop_name.clone(), prop_id)
                            })
                            .collect::<BTreeMap<_, _>>()
                    });

                let required = required.clone().unwrap_or_default();

                let additional_properties = additional_properties.as_ref().map(|ap_schema| {
                    let ap_id = id.append("additionalProperties");
                    work.push(ap_id.id(), ap_schema);
                    ap_id
                });

                let property_names = property_names.as_ref().map(|pn_schema| {
                    let pn_id = id.append("propertyNames");
                    work.push(pn_id.id(), pn_schema);
                    pn_id
                });

                let pattern_properties = pattern_properties.as_ref().map(|pattern_properties| {
                    let props_id = id.append("patternProperties");

                    pattern_properties
                        .iter()
                        .map(|(pattern, schema)| {
                            let pattern_id = props_id.append(pattern);
                            work.push(pattern_id.id(), schema);
                            (pattern.clone(), pattern_id)
                        })
                        .collect::<BTreeMap<_, _>>()
                });

                let details =
                    SchemaletDetails::Value(SchemaletValue::Object(SchemaletValueObject {
                        properties,
                        required,
                        additional_properties,
                        property_names,
                        pattern_properties,
                    }));
                let sref = id.partial("object");
                Ok((sref, details))
            }
            GenericSimpleTypes::String => {
                let schema_ref = id.partial("string");
                let ir = SchemaletDetails::Value(SchemaletValue::String {
                    pattern: self.pattern.clone(),
                    format: self.format.clone(),
                });
                Ok((schema_ref, ir))
            }
        }
    }

    fn to_schemalet_subschemas<'a, Variant>(
        work: &mut util::WorkQueue<'a, SchemaRef, GenericSchemaOrBool, Schemalet>,
        id: &SchemaRef,
        label: &str,
        variant: Variant,
        maybe_subschemas: Option<&'a Vec<GenericSchemaOrBool>>,
    ) -> Option<(SchemaRef, SchemaletDetails)>
    where
        Variant: Fn(Vec<SchemaRef>) -> SchemaletDetails,
    {
        maybe_subschemas.map(|subschemas| {
            let label_id = id.append(label);
            let subschemas = subschemas
                .iter()
                .enumerate()
                .map(|(ii, subschema)| {
                    let sref = label_id.append(&ii.to_string());
                    work.push(sref.id(), subschema);
                    sref
                })
                .collect();
            let sref = id.partial(label);
            (sref, variant(subschemas))
        })
    }
}

fn xxx_get_ref_for_opt<'a>(
    work: &mut util::WorkQueue<'a, SchemaRef, ObjectOrBool<GenericSchema>, Schemalet>,
    id: &SchemaRef,
    label: &str,
    maybe_schema: Option<&'a GenericSchemaOrBool>,
) -> Option<SchemaRef> {
    maybe_schema.map(|schema| xxx_get_ref(work, id, label, schema))
}

fn xxx_get_ref_for_vec<'a>(
    work: &mut util::WorkQueue<'a, SchemaRef, ObjectOrBool<GenericSchema>, Schemalet>,
    id: &SchemaRef,
    label: &str,
    schemas: &'a Vec<GenericSchemaOrBool>,
) -> Vec<SchemaRef> {
    schemas
        .iter()
        .enumerate()
        .map(|(ii, schema)| xxx_get_ref(work, id, &format!("{label}/{ii}"), schema))
        .collect()
}

fn xxx_get_ref<'a>(
    work: &mut util::WorkQueue<'a, SchemaRef, ObjectOrBool<GenericSchema>, Schemalet>,
    id: &SchemaRef,
    label: &str,
    schema: &'a GenericSchemaOrBool,
) -> SchemaRef {
    let sub_id = id.append(label);
    work.push(sub_id.id(), schema);
    sub_id
}

pub(crate) fn to_schemalets(
    resolved: &crate::bundler::Resolved<'_>,
    generic_schema: ObjectOrBool<GenericSchema>,
) -> Result<Vec<(SchemaRef, Schemalet)>, anyhow::Error> {
    let mut work = WorkQueue::new(resolved.context.location.to_string(), &generic_schema);

    while let Some((id, generic_schema)) = work.pop() {
        let id = SchemaRef::Id(id);
        generic_schema.to_schemalets(&mut work, id)?;
    }

    Ok(work.into_output())
}
