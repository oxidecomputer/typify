use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    ops::Deref,
};

use serde::{ser::SerializeMap, Serialize};

use crate::{
    bundler::Resolved,
    schema::{bootstrap, json_schema_2020_12},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SchemaRef {
    Id(String),
    Partial(String, String),

    // TODO Could this be yes/no?
    Merge(Vec<SchemaRef>),
    YesNo {
        yes: Box<SchemaRef>,
        no: Vec<SchemaRef>,
    },
    Internal(String),
    Box(Box<SchemaRef>),
}

impl SchemaRef {
    pub fn partial(&self, part: &str) -> Self {
        let SchemaRef::Id(id) = self else { panic!() };
        SchemaRef::Partial(id.clone(), part.to_string())
    }

    pub fn append(&self, fragment: &str) -> Self {
        let SchemaRef::Id(id) = self else { panic!() };
        SchemaRef::Id(format!("{id}/{fragment}"))
    }

    pub fn id(&self) -> String {
        let SchemaRef::Id(id) = self else { panic!() };
        id.clone()
    }
}

impl Display for SchemaRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaRef::Id(id) => f.write_str(id),
            SchemaRef::Partial(id, part) => {
                f.write_str(id)?;
                f.write_str(" @@ ")?;
                f.write_str(part)
            }
            SchemaRef::Merge(schema_refs) => {
                f.write_str("<merge> [\n")?;
                for schema_ref in schema_refs {
                    f.write_str("  ")?;
                    schema_ref.fmt(f)?;
                    f.write_str("\n")?;
                }
                f.write_str("]")
            }
            SchemaRef::YesNo { yes, no } => {
                f.write_str("<yes/no> [\n  ")?;
                yes.fmt(f)?;
                f.write_str("\n")?;
                for schema_ref in no {
                    f.write_str("  ")?;
                    schema_ref.fmt(f)?;
                    f.write_str("\n")?;
                }
                f.write_str("]")
            }
            SchemaRef::Internal(s) => {
                f.write_str("internal@")?;
                f.write_str(s)
            }
            SchemaRef::Box(id) => {
                f.write_str("box@")?;
                id.fmt(f)
            }
        }
    }
}

impl Serialize for SchemaRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        s.serialize(serializer)
    }
}

/// A Schemalet is a self-contained, bounded schema that references any
/// subordinate schemas rather than including them inline.
#[derive(Serialize, Debug, Clone)]
pub struct Schemalet {
    #[serde(flatten)]
    pub metadata: SchemaletMetadata,
    pub details: SchemaletDetails,
}

#[derive(Default, Serialize, Debug, Clone)]
pub struct SchemaletMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<serde_json::Value>,
}

#[derive(Serialize, Debug, Clone)]
pub enum SchemaletDetails {
    // Native
    Anything,
    Nothing,

    // Subschemas
    OneOf(Vec<SchemaRef>),
    AnyOf(Vec<SchemaRef>),
    AllOf(Vec<SchemaRef>),
    Not(SchemaRef),
    IfThen(SchemaRef, SchemaRef),
    IfThenElse(SchemaRef, SchemaRef, SchemaRef),
    RawRef(String),
    RawDynamicRef(String),
    Constant(serde_json::Value),
    Value(SchemaletValue),

    // Synthetic
    ExclusiveOneOf(Vec<SchemaRef>),
    ResolvedRef(SchemaRef),
    ResolvedDynamicRef(SchemaRef),
    YesNo { yes: SchemaRef, no: Vec<SchemaRef> },
    StringOf(SchemaRef),
}

#[derive(Debug, Clone, Serialize)]
pub struct SchemaletValueString {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pattern: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub format: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub enum SchemaletValue {
    Boolean,
    Array(SchemaletValueArray),
    Object(SchemaletValueObject),
    String(SchemaletValueString),
    Integer {
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<serde_json::Number>,
        #[serde(skip_serializing_if = "Option::is_none")]
        exclusive_minimum: Option<serde_json::Number>,
    },
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        exclusive_minimum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        exclusive_maximum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        multiple_of: Option<f64>,
    },
    Null,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SchemaletType {
    Boolean,
    Array,
    Object,
    String,
    Integer,
    Number,
    Null,
}

impl SchemaletType {
    pub(crate) fn variant_name(&self) -> &'static str {
        match self {
            SchemaletType::Boolean => "Boolean",
            SchemaletType::Array => "Array",
            SchemaletType::Object => "Object",
            SchemaletType::String => "String",
            SchemaletType::Integer => "Integer",
            SchemaletType::Number => "Number",
            SchemaletType::Null => "Null",
        }
    }
}

// TODO don't worry about naming for now, but this will probably be the most
// relevant output type
#[derive(Serialize, Debug, Clone)]
pub struct CanonicalSchemalet {
    #[serde(flatten)]
    pub metadata: SchemaletMetadata,
    pub details: CanonicalSchemaletDetails,
}

impl Deref for CanonicalSchemalet {
    type Target = CanonicalSchemaletDetails;

    fn deref(&self) -> &Self::Target {
        &self.details
    }
}

impl CanonicalSchemaletDetails {
    pub fn get_type(&self) -> Option<SchemaletType> {
        match self {
            CanonicalSchemaletDetails::Constant(value) => match value {
                serde_json::Value::Null => Some(SchemaletType::Null),
                serde_json::Value::Bool(_) => Some(SchemaletType::Boolean),
                serde_json::Value::Number(_) => {
                    todo!()
                }
                serde_json::Value::String(_) => Some(SchemaletType::String),
                serde_json::Value::Array(_) => Some(SchemaletType::Array),
                serde_json::Value::Object(_) => Some(SchemaletType::Object),
            },
            CanonicalSchemaletDetails::Anything => None,
            CanonicalSchemaletDetails::Nothing => None,
            // TODO maybe we should handle this differently?
            CanonicalSchemaletDetails::Reference(_) => todo!(),
            // TODO ^^ Maybe this is a different place we should handle it??
            CanonicalSchemaletDetails::Note(_) => todo!(),
            CanonicalSchemaletDetails::ExclusiveOneOf { typ, .. } => typ.clone(),
            CanonicalSchemaletDetails::Value(value) => match value {
                SchemaletValue::Boolean => Some(SchemaletType::Boolean),
                SchemaletValue::Array { .. } => Some(SchemaletType::Array),
                SchemaletValue::Object(_) => Some(SchemaletType::Object),
                SchemaletValue::String { .. } => Some(SchemaletType::String),
                SchemaletValue::Integer { .. } => Some(SchemaletType::Integer),
                SchemaletValue::Number { .. } => Some(SchemaletType::Number),
                SchemaletValue::Null => Some(SchemaletType::Null),
            },
        }
    }

    fn is_nothing(&self) -> bool {
        matches!(self, CanonicalSchemaletDetails::Nothing)
    }

    pub fn as_object(&self) -> Option<&SchemaletValueObject> {
        let Self::Value(SchemaletValue::Object(obj)) = self else {
            return None;
        };
        Some(obj)
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum CanonicalSchemaletDetails {
    Anything,
    Nothing,
    Constant(serde_json::Value),
    // TODO 6/14/2025 not 100% sure where this is going to be used, but it
    // might be interesting
    // TODO 6/14/2025 yeah this is going to be important: we're going to want
    // to make sure we don't lose description data e.g. so that a struct field
    // has a comment and so does its type. We'll want to keep metadata. Typify
    // will need to deal with it by walking this linked list.
    Reference(SchemaRef),
    // TODO 6/30/2025 What I'm going to do is use "Reference" to indicate some
    // indirection in the original schema and <whatever this is called> to
    // indicate merely and internal node.
    Note(SchemaRef),
    ExclusiveOneOf {
        /// Cached type iff all subschemas share a single type.
        typ: Option<SchemaletType>,
        /// Component subschemas.
        subschemas: Vec<SchemaRef>,
    },
    // TODO 6/14/2025 This is wrong. I know I'm going to need constraints (both
    // affirmative and negative), and we need to handle constant values more
    // similarly, etc. Also "Anything", but we'll roll with that.
    Value(SchemaletValue),
}

pub enum State {
    Stuck(Schemalet),
    Simplified(Schemalet, Vec<(SchemaRef, Schemalet)>),
    Canonical(CanonicalSchemalet),
}

// TODO 6/28/2025
// Rather than having properties, required, additionalProperties,
// patternProperties, propertyNames, unevaluatedProperties... I think I can
// convey the information I need with orthogonal concepts:
// - fields: array of properties with a boolean for required
// - more fields: array of pairs of key/value schema references
// - extras allowed: true/false
//
// The idea is that I can avoid the miasma of overlapping ideas and simplify
// type generation. Fields become fields in a struct; each value of the more
// fields array becomes a flattened structure; extras allowed informs serde
// policy regarding additional fields.

#[derive(Default, Debug, Clone, Serialize)]
pub struct SchemaletValueObject {
    pub properties: BTreeMap<String, SchemaRef>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<SchemaRef>,

    /// Implied that it's a string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_names: Option<SchemaRef>,

    /// Map from a regex to a schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<BTreeMap<String, SchemaRef>>,
}

pub struct CanonicalSchemaletValueObject {
    pub fixed_properties: BTreeMap<String, CanonicalSchemaletValueObjectFixed>,

    /// Note that these may be overlapping
    pub more_properties: Vec<CanonicalSchemaletValueObjectMore>,

    pub allow_unknown: bool,
}
struct CanonicalSchemaletValueObjectFixed {
    pub id: SchemaRef,
    pub required: bool,
}
struct CanonicalSchemaletValueObjectMore {
    pub key: SchemaRef,
    pub value: SchemaRef,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SchemaletValueArray {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<SchemaRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_items: Option<Vec<SchemaRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
}

impl Schemalet {
    pub fn new(details: SchemaletDetails, metadata: SchemaletMetadata) -> Self {
        Self { metadata, details }
    }

    pub fn from_details(details: SchemaletDetails) -> Self {
        Self {
            metadata: Default::default(),
            details,
        }
    }

    pub fn simplify(self, done: &BTreeMap<SchemaRef, CanonicalSchemalet>) -> State {
        let Self { metadata, details } = self;
        match details {
            SchemaletDetails::OneOf(..) => todo!(),
            SchemaletDetails::Not(..) => todo!(),
            SchemaletDetails::IfThen(..) => todo!(),
            SchemaletDetails::IfThenElse(..) => todo!(),
            SchemaletDetails::RawRef(_) => todo!(),
            SchemaletDetails::RawDynamicRef(_) => todo!(),
            SchemaletDetails::AllOf(schema_refs) => {
                if let Some(subschemas) = resolve_all(done, &schema_refs) {
                    println!("{}", serde_json::to_string_pretty(&subschemas).unwrap());
                    merge_all(metadata, subschemas, done)
                } else {
                    State::Stuck(Schemalet {
                        metadata,
                        details: SchemaletDetails::AllOf(schema_refs),
                    })
                }
            }
            SchemaletDetails::AnyOf(schema_refs) => {
                if let Some(subschemas) = resolve_all(done, &schema_refs) {
                    println!(
                        "canonical anyof {}",
                        serde_json::to_string_pretty(&subschemas).unwrap()
                    );
                    expand_any_of(metadata, subschemas)
                } else {
                    State::Stuck(Schemalet {
                        metadata,
                        details: SchemaletDetails::AnyOf(schema_refs),
                    })
                }
            }
            SchemaletDetails::Anything => State::Canonical(CanonicalSchemalet {
                metadata,
                details: CanonicalSchemaletDetails::Anything,
            }),
            SchemaletDetails::Nothing => State::Canonical(CanonicalSchemalet {
                metadata,
                details: CanonicalSchemaletDetails::Nothing,
            }),
            SchemaletDetails::Constant(value) => State::Canonical(CanonicalSchemalet {
                metadata,
                details: CanonicalSchemaletDetails::Constant(value),
            }),
            SchemaletDetails::ResolvedDynamicRef(reference)
            | SchemaletDetails::ResolvedRef(reference) => State::Canonical(CanonicalSchemalet {
                metadata,
                details: CanonicalSchemaletDetails::Reference(reference),
            }),
            SchemaletDetails::Value(value) => State::Canonical(CanonicalSchemalet {
                metadata,
                details: CanonicalSchemaletDetails::Value(value),
            }),
            SchemaletDetails::ExclusiveOneOf(schema_refs) => {
                if let Some(subschemas) = resolve_all(done, &schema_refs) {
                    let subschemas = subschemas
                        .into_iter()
                        .filter(|(_, schemalet)| !schemalet.is_nothing())
                        .collect::<Vec<_>>();

                    let new_schema = match subschemas.len() {
                        0 => CanonicalSchemalet {
                            metadata,
                            details: CanonicalSchemaletDetails::Nothing,
                        },

                        // TODO 7/28/2025
                        // I can't really recall the thinking here. I think the
                        // idea was that I don't want to throw away information
                        // in case there are useful descriptions or titles,
                        // but... this doesn't seem like a fully formed plan.
                        1 => {
                            let xxx = subschemas.into_iter().next().unwrap().0;
                            CanonicalSchemalet {
                                metadata,
                                details: CanonicalSchemaletDetails::Note(xxx),
                            }
                        }

                        _ => {
                            let typ = subschemas
                                .iter()
                                .map(|(_, schemalet)| schemalet.get_type())
                                .reduce(|a, b| match (a, b) {
                                    (Some(aa), Some(bb)) if aa == bb => Some(aa),
                                    _ => None,
                                })
                                .flatten();
                            let subschemas = subschemas
                                .into_iter()
                                .map(|(schema_ref, _)| schema_ref)
                                .collect();

                            CanonicalSchemalet {
                                metadata,
                                details: CanonicalSchemaletDetails::ExclusiveOneOf {
                                    typ,
                                    subschemas,
                                },
                            }
                        }
                    };

                    // TODO we need to remove any `Never` schemalets and then
                    // special case 1 => the type, and 0 => Never
                    // TODO memoize the type
                    State::Canonical(new_schema)
                } else {
                    State::Stuck(Schemalet {
                        metadata,
                        details: SchemaletDetails::ExclusiveOneOf(schema_refs),
                    })
                }
            }
            SchemaletDetails::YesNo { yes, no } => {
                let ryes = resolve(done, &yes);
                let rno = no
                    .iter()
                    .map(|sr| resolve(done, sr))
                    .collect::<Option<Vec<_>>>();
                if let (Some(yes), Some(no)) = (ryes, rno) {
                    println!(
                        "yes/no {}",
                        serde_json::to_string_pretty(&serde_json::json!({ "yes": yes, "no": no }))
                            .unwrap()
                    );
                    merge_yes_no(yes, no, done)
                } else {
                    State::Stuck(Schemalet {
                        metadata,
                        details: SchemaletDetails::YesNo { yes, no },
                    })
                }
            }
            SchemaletDetails::StringOf(schema_ref) => {
                simplify_string_of(metadata, done, schema_ref)
            }
        }
    }
}

fn simplify_string_of(
    metadata: SchemaletMetadata,
    done: &BTreeMap<SchemaRef, CanonicalSchemalet>,
    schema_ref: SchemaRef,
) -> State {
    println!("string of {schema_ref}");

    let Some((sr, ss)) = resolve(done, &schema_ref) else {
        return State::Stuck(Schemalet {
            metadata,
            details: SchemaletDetails::StringOf(schema_ref),
        });
    };

    if let Some(ty) = ss.get_type() {
        match ty {
            SchemaletType::String => {
                return State::Canonical(CanonicalSchemalet {
                    metadata,
                    details: CanonicalSchemaletDetails::Note(schema_ref),
                });
            }

            _ => {
                return State::Canonical(CanonicalSchemalet {
                    metadata,
                    details: CanonicalSchemaletDetails::Nothing,
                });
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(ss).unwrap());

    match &ss.details {
        CanonicalSchemaletDetails::Anything => todo!(),
        CanonicalSchemaletDetails::Nothing => todo!(),
        CanonicalSchemaletDetails::Constant(value) => todo!(),
        CanonicalSchemaletDetails::Reference(schema_ref) => todo!(),
        CanonicalSchemaletDetails::Note(schema_ref) => todo!(),

        CanonicalSchemaletDetails::ExclusiveOneOf { typ, subschemas } => {
            let mut new_work = Vec::new();
            let mut new_subschemas = Vec::new();

            println!("subschemas {:#?}", subschemas);

            for subschema in subschemas {
                // TODO 7/31/2025
                // I need to give some more thought to how we handle this
                let subschema_string_of =
                    SchemaRef::Partial(subschema.to_string(), "stringOf".to_string());

                let new_subschema = Schemalet {
                    metadata: Default::default(),
                    details: SchemaletDetails::StringOf(subschema.clone()),
                };

                new_work.push((subschema_string_of.clone(), new_subschema));
                new_subschemas.push(subschema_string_of);
            }

            let new_schemalet = Schemalet {
                metadata,
                details: SchemaletDetails::ExclusiveOneOf(new_subschemas),
            };

            State::Simplified(new_schemalet, new_work)
        }

        CanonicalSchemaletDetails::Value(schemalet_value) => {
            println!(
                "value {}",
                serde_json::to_string_pretty(schemalet_value).unwrap()
            );
            panic!()
        }
    }
}

fn merge_yes_no(
    yes: (SchemaRef, &CanonicalSchemalet),
    no: Vec<(SchemaRef, &CanonicalSchemalet)>,
    done: &BTreeMap<SchemaRef, CanonicalSchemalet>,
) -> State {
    if let Some(typ) = yes.1.get_type() {
        if no.iter().all(|(_, no_subschema)| {
            no_subschema
                .get_type()
                .map_or(false, |no_typ| no_typ != typ)
        }) {
            return State::Simplified(
                Schemalet {
                    metadata: Default::default(),
                    details: SchemaletDetails::ResolvedRef(yes.0),
                },
                Default::default(),
            );
        }
    }

    if no
        .iter()
        .all(|(_, no_subschema)| type_incompatible(&yes.1, no_subschema, done))
    {
        return State::Simplified(
            Schemalet {
                metadata: Default::default(),
                details: SchemaletDetails::ResolvedRef(yes.0),
            },
            Default::default(),
        );
    }

    match &yes.1.details {
        CanonicalSchemaletDetails::Anything => todo!(),
        CanonicalSchemaletDetails::Nothing => State::Simplified(
            Schemalet {
                metadata: Default::default(),
                details: SchemaletDetails::ResolvedRef(yes.0),
            },
            Default::default(),
        ),
        CanonicalSchemaletDetails::Constant(value) => todo!(),
        CanonicalSchemaletDetails::Reference(schema_ref) => todo!(),
        CanonicalSchemaletDetails::Note(schema_ref) => todo!(),
        CanonicalSchemaletDetails::ExclusiveOneOf { typ, subschemas } => {
            todo!()
        }
        CanonicalSchemaletDetails::Value(schemalet_value) => {
            todo!()
        }
    }
}

fn type_incompatible(
    a: &CanonicalSchemalet,
    b: &CanonicalSchemalet,
    done: &BTreeMap<SchemaRef, CanonicalSchemalet>,
) -> bool {
    match (a, b) {
        (
            other @ CanonicalSchemalet {
                details: CanonicalSchemaletDetails::Value(_),
                ..
            },
            CanonicalSchemalet {
                details: CanonicalSchemaletDetails::ExclusiveOneOf { subschemas, .. },
                ..
            },
        )
        | (
            CanonicalSchemalet {
                details: CanonicalSchemaletDetails::ExclusiveOneOf { subschemas, .. },
                ..
            },
            other @ CanonicalSchemalet {
                details: CanonicalSchemaletDetails::Value(_),
                ..
            },
        ) => {
            let subschemas = resolve_all(done, subschemas).unwrap();
            subschemas
                .iter()
                .all(|(_, subschema)| type_incompatible(other, subschema, done))
        }
        (a, b)
            if match (a.get_type(), b.get_type()) {
                (Some(aa), Some(bb)) if aa != bb => true,
                _ => false,
            } =>
        {
            true
        }
        _ => {
            println!("unhandled type_incompatible");
            println!(
                "{}\n{}",
                serde_json::to_string_pretty(a).unwrap(),
                serde_json::to_string_pretty(b).unwrap(),
            );
            todo!()
        }
    }
}

fn expand_any_of(
    metadata: SchemaletMetadata,
    subschemas: Vec<(SchemaRef, &CanonicalSchemalet)>,
) -> State {
    let len = subschemas.len();

    // TODO this could be a lot smarter by looking at the schemas
    let permutations = (1..(1 << len))
        .map(|bitmap| {
            let mut yes = Vec::new();
            let mut no = Vec::new();

            for (ii, (schema_ref, _)) in subschemas.iter().enumerate() {
                if (1 << ii) & bitmap != 0 {
                    yes.push(schema_ref.clone());
                } else {
                    no.push(schema_ref.clone());
                }
            }

            (yes, no)
        })
        .collect::<Vec<_>>();
    println!("yes/no {:#?}", permutations);

    let mut new_work = Vec::new();
    let mut new_subschemas = Vec::new();

    for (yes, no) in permutations {
        let yes = match yes.as_slice() {
            [] => unreachable!(),
            [solo] => solo.clone(),
            all => {
                let schema_refs = all.iter().cloned().collect::<Vec<_>>();
                let merge_ref = SchemaRef::Merge(schema_refs.clone());
                let merge = Schemalet {
                    metadata: Default::default(),
                    details: SchemaletDetails::AllOf(schema_refs),
                };

                new_work.push((merge_ref.clone(), merge));
                merge_ref
            }
        };

        let new_ref = SchemaRef::YesNo {
            yes: Box::new(yes.clone()),
            no: no.clone(),
        };

        let new_subschema = Schemalet {
            metadata: Default::default(),
            details: SchemaletDetails::YesNo { yes, no },
        };

        new_work.push((new_ref.clone(), new_subschema));
        new_subschemas.push(new_ref);
    }

    let new_schemalet = Schemalet {
        metadata,
        details: SchemaletDetails::ExclusiveOneOf(new_subschemas),
    };

    State::Simplified(new_schemalet, new_work)
}

// TODO 6/14/2025 not fully sure why we need the done map...
fn merge_all(
    metadata: SchemaletMetadata,
    subschemas: Vec<(SchemaRef, &CanonicalSchemalet)>,
    done: &BTreeMap<SchemaRef, CanonicalSchemalet>,
) -> State {
    // Separate out xors (disjunctions) from other schemas.
    let mut xors = Vec::new();
    let mut rest = Vec::new();
    for (schema_ref, schema) in subschemas {
        match &schema.details {
            CanonicalSchemaletDetails::ExclusiveOneOf { subschemas, .. } => xors.push(subschemas),
            _ => rest.push((schema_ref, schema)),
        }
    }

    if let Some(subschemas) = xors.pop() {
        let mut merge_groups = subschemas
            .iter()
            .map(|schema_ref| (schema_ref, vec![schema_ref]))
            .collect::<Vec<_>>();

        for subschemas in xors {
            merge_groups = merge_groups
                .into_iter()
                .flat_map(|(representative, group)| {
                    subschemas
                        .iter()
                        .filter(|schema_ref| {
                            !trivially_incompatible(done, representative, schema_ref)
                        })
                        .map(move |schema_ref| {
                            let mut new_group = group.clone();
                            new_group.push(schema_ref);
                            (representative, new_group)
                        })
                })
                .collect::<Vec<_>>()
        }

        let mut merge_groups = merge_groups
            .into_iter()
            .map(|(_, group)| group)
            .collect::<Vec<_>>();

        for group in &mut merge_groups {
            for (schema_ref, _) in &rest {
                group.push(schema_ref);
            }
        }

        // TODO do we know anything about the cardinality of `groups` at this
        // point i.e. do we know that it's >1?

        println!(
            "groups {}",
            serde_json::to_string_pretty(&merge_groups).unwrap()
        );

        // let xxx = merge_groups.iter().map(|group| {
        //     let subschemas = group
        //         .iter()
        //         .map(|schema_ref| {
        //             resolve(done, schema_ref)
        //                 .expect("already resolved previously, so should be infallible")
        //                 .1
        //         })
        //         .collect::<Vec<_>>();
        //     merge_all_values(subschemas)
        //     todo!();
        //     todo!()
        // });

        let mut new_work = Vec::new();
        let mut new_subschemas = Vec::new();

        for group in merge_groups {
            let refs = group.into_iter().cloned().collect::<Vec<_>>();
            let new_schemaref = SchemaRef::Merge(refs.clone());
            let new_schemalet = Schemalet {
                metadata: Default::default(),
                details: SchemaletDetails::AllOf(refs.clone()),
            };

            new_work.push((new_schemaref.clone(), new_schemalet));
            new_subschemas.push(new_schemaref);
        }

        let new_schemalet = Schemalet {
            metadata,
            details: SchemaletDetails::ExclusiveOneOf(new_subschemas),
        };

        State::Simplified(new_schemalet, new_work)
    } else {
        // Here we know that we've got a flat collection of canonical
        // schemalets with no nesting. We can also assume that the list of
        // subschemas is non-empty.

        let subschemas = rest
            .into_iter()
            .map(|(_, schemalet)| schemalet)
            .collect::<Vec<_>>();

        // TODO 6/14/2025
        // I need to be thoughtful about when I can and don't preserve
        // metadata. For example, some metadata might become comments on struct
        // fields.

        let mut merged_details = CanonicalSchemaletDetails::Anything;

        for subschema in subschemas {
            merged_details = merge_two(&merged_details, &subschema.details);
        }

        println!(
            "merged {}",
            serde_json::to_string_pretty(&merged_details).unwrap()
        );

        let new_schemalet = CanonicalSchemalet {
            metadata,
            details: merged_details,
        };

        State::Canonical(new_schemalet)
    }
}

fn merge_two(
    a: &CanonicalSchemaletDetails,
    b: &CanonicalSchemaletDetails,
) -> CanonicalSchemaletDetails {
    match (a.get_type(), b.get_type()) {
        (Some(aa), Some(bb)) if aa != bb => return CanonicalSchemaletDetails::Nothing,
        _ => (),
    }
    match (a, b) {
        (CanonicalSchemaletDetails::Anything, other)
        | (other, CanonicalSchemaletDetails::Anything) => other.clone(),

        (CanonicalSchemaletDetails::Nothing, _) | (_, CanonicalSchemaletDetails::Nothing) => {
            CanonicalSchemaletDetails::Nothing
        }

        (
            CanonicalSchemaletDetails::Value(SchemaletValue::Boolean),
            CanonicalSchemaletDetails::Value(SchemaletValue::Boolean),
        ) => CanonicalSchemaletDetails::Value(SchemaletValue::Boolean),

        (
            CanonicalSchemaletDetails::Value(SchemaletValue::Object(aa)),
            CanonicalSchemaletDetails::Value(SchemaletValue::Object(bb)),
        ) => merge_two_objects(aa, bb),

        (
            CanonicalSchemaletDetails::Value(SchemaletValue::String(aa)),
            CanonicalSchemaletDetails::Value(SchemaletValue::String(bb)),
        ) => merge_two_strings(aa, bb),

        (
            CanonicalSchemaletDetails::Value(SchemaletValue::Array(aa)),
            CanonicalSchemaletDetails::Value(SchemaletValue::Array(bb)),
        ) => merge_two_arrays(aa, bb),

        _ => todo!(
            "merge_two {}",
            serde_json::to_string_pretty(&[a, b]).unwrap()
        ),
    }
}

fn merge_two_arrays(
    aa: &SchemaletValueArray,
    bb: &SchemaletValueArray,
) -> CanonicalSchemaletDetails {
    todo!()
}

fn merge_two_strings(
    aa: &SchemaletValueString,
    bb: &SchemaletValueString,
) -> CanonicalSchemaletDetails {
    let pattern = aa
        .pattern
        .iter()
        .chain(bb.pattern.iter())
        .cloned()
        .collect();
    let format = aa.format.iter().chain(bb.format.iter()).cloned().collect();
    let min_length = match (aa.min_length, bb.min_length) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) | (None, Some(a)) => Some(a),
        (None, None) => None,
    };
    let max_length = match (aa.max_length, bb.max_length) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) | (None, Some(a)) => Some(a),
        (None, None) => None,
    };
    CanonicalSchemaletDetails::Value(SchemaletValue::String(SchemaletValueString {
        pattern,
        format,
        min_length,
        max_length,
    }))
}

fn merge_two_objects(
    aa: &SchemaletValueObject,
    bb: &SchemaletValueObject,
) -> CanonicalSchemaletDetails {
    let prop_names = aa.properties.keys().chain(bb.properties.keys());
    let properties = prop_names
        .map(
            |prop_name| match (aa.properties.get(prop_name), bb.properties.get(prop_name)) {
                (None, None) => unreachable!("must exist in one or the other"),
                (None, Some(prop_ref)) | (Some(prop_ref), None) => {
                    // TODO need to consider the *other* object's
                    // additionalProperties field.
                    (prop_name.clone(), prop_ref.clone())
                }
                (Some(_), Some(_)) => todo!(),
            },
        )
        .collect();

    let additional_properties = match (&aa.additional_properties, &bb.additional_properties) {
        (None, None) => None,
        (None, Some(other)) | (Some(other), None) => Some(other.clone()),
        (Some(_), Some(_)) => todo!(),
    };

    assert!(additional_properties.is_none());

    CanonicalSchemaletDetails::Value(SchemaletValue::Object(SchemaletValueObject {
        properties,
        additional_properties,
        ..Default::default()
    }))
}

fn trivially_incompatible(
    done: &BTreeMap<SchemaRef, CanonicalSchemalet>,
    a: &SchemaRef,
    b: &SchemaRef,
) -> bool {
    let (_, aaa) = resolve(done, a).unwrap();
    let (_, bbb) = resolve(done, b).unwrap();

    match (aaa.get_type(), bbb.get_type()) {
        (Some(a_type), Some(b_type)) if a_type != b_type => true,
        _ => false,
    }
}

pub fn to_schemalets(resolved: &Resolved<'_>) -> anyhow::Result<Vec<(SchemaRef, Schemalet)>> {
    match resolved.schema {
        "bootstrap" => bootstrap::to_schemalets(resolved),
        "https://json-schema.org/draft/2020-12/schema" => {
            json_schema_2020_12::to_schemalets(resolved)
        }
        _ => todo!(),
    }
}

trait Refers {
    fn refers(&self) -> Option<&SchemaRef>;
}

impl Refers for Schemalet {
    fn refers(&self) -> Option<&SchemaRef> {
        match &self.details {
            SchemaletDetails::ResolvedRef(reference)
            | SchemaletDetails::ResolvedDynamicRef(reference) => Some(reference),
            _ => None,
        }
    }
}

impl Refers for CanonicalSchemalet {
    fn refers(&self) -> Option<&SchemaRef> {
        if let CanonicalSchemaletDetails::Reference(reference) = &self.details {
            Some(reference)
        } else {
            None
        }
    }
}

fn resolve<'a, T>(
    wip: &'a BTreeMap<SchemaRef, T>,
    schema_ref: &SchemaRef,
) -> Option<(SchemaRef, &'a T)>
where
    T: Refers,
{
    let mut schema_ref = schema_ref;
    loop {
        let schemalet = wip.get(&schema_ref)?;
        if let Some(reference) = schemalet.refers() {
            schema_ref = reference;
        } else {
            break Some((schema_ref.clone(), schemalet));
        }
    }
}

fn resolve_all<'a, T, I>(
    wip: &'a BTreeMap<SchemaRef, T>,
    schemas: I,
) -> Option<Vec<(SchemaRef, &'a T)>>
where
    T: Refers,
    I: IntoIterator<Item = &'a SchemaRef>,
{
    schemas
        .into_iter()
        .map(|schema_ref| resolve(wip, schema_ref))
        .collect()
}

pub fn schemalet_to_type(
    schemalet: &CanonicalSchemalet,
    graph: &BTreeMap<SchemaRef, CanonicalSchemalet>,
) {
    match &schemalet.details {
        CanonicalSchemaletDetails::Anything => todo!(),
        CanonicalSchemaletDetails::Nothing => todo!(),
        CanonicalSchemaletDetails::Constant(value) => todo!(),
        CanonicalSchemaletDetails::Reference(schema_ref) => todo!(),
        CanonicalSchemaletDetails::Note(schema_ref) => todo!(),
        CanonicalSchemaletDetails::ExclusiveOneOf { subschemas, .. } => {
            schemalet_to_type_enum(&schemalet.metadata, subschemas, graph)
        }
        CanonicalSchemaletDetails::Value(schemalet_value) => {
            schemalet_to_type_value(&schemalet.metadata, schemalet_value, graph)
        }
    }
}

fn schemalet_to_type_value(
    metadata: &SchemaletMetadata,
    value: &SchemaletValue,
    graph: &BTreeMap<SchemaRef, CanonicalSchemalet>,
) {
    match value {
        SchemaletValue::Boolean => todo!(),
        SchemaletValue::Array(array) => todo!(),

        SchemaletValue::Object(object) => schemalet_to_type_value_object(metadata, object, graph),
        SchemaletValue::String { .. } => todo!(),
        SchemaletValue::Integer {
            minimum,
            exclusive_minimum,
        } => todo!(),
        SchemaletValue::Number {
            minimum,
            exclusive_minimum,
            maximum,
            exclusive_maximum,
            multiple_of,
        } => todo!(),
        SchemaletValue::Null => todo!(),
    }
}

fn schemalet_to_type_value_object(
    metadata: &SchemaletMetadata,
    object: &SchemaletValueObject,
    graph: &BTreeMap<SchemaRef, CanonicalSchemalet>,
) {
    let SchemaletValueObject {
        properties,
        additional_properties,
        ..
    } = object;

    if properties.is_empty() {
        assert!(additional_properties.is_some());
    }

    todo!()
}

fn schemalet_to_type_enum(
    metadata: &SchemaletMetadata,
    subschemas: &[SchemaRef],
    graph: &BTreeMap<SchemaRef, CanonicalSchemalet>,
) {
    // TODO what I'm not sure of here is how deep to chase references...
    // something we can figure out later...

    let resolved_subschemas = subschemas
        .into_iter()
        .map(|schema_ref| graph.get(schema_ref).unwrap())
        .collect::<Vec<_>>();

    println!(
        "subschemas {}",
        serde_json::to_string_pretty(&resolved_subschemas).unwrap()
    );

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

    // TODO since we know we don't have good names yet, let's just move on

    let variant_names = if let Some(types) = xxx_rename_all_types_unique(&resolved_subschemas) {
        types
            .into_iter()
            .map(|ty| ty.variant_name().to_string())
            .collect::<Vec<_>>()
    } else {
        (0..resolved_subschemas.len())
            .map(|ii| format!("Variant{ii}"))
            .collect::<Vec<_>>()
    };
    println!(
        "variant_names {}",
        serde_json::to_string_pretty(&variant_names).unwrap()
    );

    // 1. Peek into the type to see if there are references that might be
    //    comments we want to put at the top of our variant
    // 2. Convert the type. I want to make sure the recursive depth is bounded
    //    just as a matter of principle. For structs this seems fine--no need
    //    to recur. Tuples? Also, won't recur for discrete items. What about
    //    other enums? I'm not really interested in that result. Perhaps I can
    //    just peek and convert depending on what I see?

    // subschemas
    //     .iter()
    //     .zip(variant_names)
    //     .map(|(variant_schema, variant_name)| {
    //         make_variant_meta(graph, variant_schema, &variant_name)
    //     })
    //     .collect::<Vec<_>>();

    // This is just for untagged.
    let variants = subschemas
        .iter()
        .zip(variant_names)
        .map(|(variant_ref, variant_name)| untagged_variant(graph, variant_ref, &variant_name))
        .collect::<Vec<_>>();

    println!(
        "variants {}",
        serde_json::to_string_pretty(&variants).unwrap()
    );
}

fn untagged_variant(
    graph: &BTreeMap<SchemaRef, CanonicalSchemalet>,
    variant_ref: &SchemaRef,
    variant_name: &str,
) -> EnumVariant {
    let variant_schema = graph.get(variant_ref).unwrap();

    let details = match &variant_schema.details {
        CanonicalSchemaletDetails::Anything => unreachable!(),
        CanonicalSchemaletDetails::Nothing => unreachable!(),
        // TODO not sure about these ones:
        CanonicalSchemaletDetails::Reference(schema_ref) => todo!(),
        CanonicalSchemaletDetails::Constant(value) => todo!(),

        CanonicalSchemaletDetails::Value(SchemaletValue::Object(SchemaletValueObject {
            ..
        })) => {
            // For an object whose contents we're willing to inline (?? TODO
            // TBD ??), we convert the type and jam the fields directly into
            // this variant. Because we can.
            let xxx = schemalet_to_type(variant_schema, graph);
            todo!()
        }
        CanonicalSchemaletDetails::Value(SchemaletValue::Array(array)) => {
            // TODO we're going to need to do something here for tuples...
            todo!()
        }

        CanonicalSchemaletDetails::Value(SchemaletValue::Null) => {
            // TODO another interesting case in that we need to produce a simple variant.
            todo!()
        }

        _ => EnumVariantDetails::Item(variant_ref.clone()),
    };

    println!(
        "details {}, {}",
        variant_name,
        serde_json::to_string_pretty(&details).unwrap()
    );

    EnumVariant {
        name: variant_name.to_string(),
        details,
    }
}

#[derive(Debug, Serialize)]
struct EnumVariant {
    name: String,

    details: EnumVariantDetails,
}

#[derive(Debug, Serialize)]
enum EnumVariantDetails {
    /// A simple variant corresponds to a particular value. This will typically
    /// be a string label for an externally tagged enum, or a tag value for an
    /// adjacently tagged enum. We also use it to represent, say, some
    /// particular constant json value in an untagged variant--in which case we
    /// also need custom serialization and deserialization logic.
    Simple,
    Item(SchemaRef),
    Tuple(Vec<SchemaRef>),
    // Struct(Vec<StructProperty>),
}

fn make_variant_meta(
    graph: &BTreeMap<SchemaRef, CanonicalSchemalet>,
    variant_schema: &CanonicalSchemalet,
    variant_name: &str,
) -> VariantMeta {
    match &variant_schema.details {
        CanonicalSchemaletDetails::Anything => {
            // This should not be reachable. What would it mean to have an
            // exclusive one-of that included a permissive schema?
            // Tautologically this would to need to be the only variant, and
            // thus we should have simplified away the one-of construction.
            unreachable!()
        }
        CanonicalSchemaletDetails::Nothing => {
            // Similarly, we only construct exclusive one-of schemas knowing
            // that each variant is viable. This variant should already have
            // been eliminated.
            unreachable!()
        }
        CanonicalSchemaletDetails::Constant(value) => todo!(),
        CanonicalSchemaletDetails::Reference(schema_ref) => todo!(),
        CanonicalSchemaletDetails::Note(schema_ref) => todo!(),
        CanonicalSchemaletDetails::ExclusiveOneOf { .. } => {
            // Fine: we aren't going to be able to do anything interesting
            // inline in the context of this variants, so we are fine making
            // this simply an "item" variant.
            todo!()
        }
        CanonicalSchemaletDetails::Value(SchemaletValue::Object(SchemaletValueObject {
            ..
        })) => {
            // This is the most interesting case and is a preconsition for
            // externally-, internally-, and adjacently-tagged enums.
            todo!()
        }
        CanonicalSchemaletDetails::Value(schemalet_value) => todo!(),
    }

    todo!()
}

struct VariantMeta {
    description: Option<String>,

    constant_value_fields: BTreeMap<String, String>,
}

fn xxx_rename_all_types_unique(subschemas: &[&CanonicalSchemalet]) -> Option<Vec<SchemaletType>> {
    let types = subschemas
        .into_iter()
        .map(|schema| schema.get_type())
        .collect::<Option<Vec<_>>>()?;

    let unique_types = types.iter().collect::<BTreeSet<_>>();

    (types.len() == unique_types.len()).then_some(types)
}

struct ThingPrinter<'a> {
    graph: &'a BTreeMap<SchemaRef, CanonicalSchemalet>,
    schema_ref: SchemaRef,
}

impl Serialize for ThingPrinter<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self { graph, schema_ref } = self;
        let schemalet = graph.get(schema_ref).unwrap();

        let mut map = serializer.serialize_map(None)?;

        let CanonicalSchemalet { metadata, details } = schemalet;
        let SchemaletMetadata {
            title,
            description,
            examples,
        } = metadata;

        if let Some(title) = title {
            map.serialize_entry("title", title)?;
        }
        if let Some(description) = description {
            map.serialize_entry("description", description)?;
        }
        if !examples.is_empty() {
            map.serialize_entry("examples", examples)?;
        }

        match details {
            CanonicalSchemaletDetails::Anything => {
                // TODO?
            }
            CanonicalSchemaletDetails::Nothing => todo!(),
            CanonicalSchemaletDetails::Constant(value) => {
                map.serialize_entry("const", value)?;
            }
            CanonicalSchemaletDetails::Reference(schema_ref) => {
                map.serialize_entry("$ref", schema_ref)?;
            }
            CanonicalSchemaletDetails::Note(schema_ref) => {
                map.serialize_entry("$note", schema_ref)?;
            }
            CanonicalSchemaletDetails::ExclusiveOneOf { typ, subschemas } => {
                let xxx = subschemas
                    .iter()
                    .map(|ss| ThingPrinter {
                        graph,
                        schema_ref: ss.clone(),
                    })
                    .collect::<Vec<_>>();
                map.serialize_entry("xor", &xxx)?;
            }
            CanonicalSchemaletDetails::Value(schemalet_value) => match schemalet_value {
                SchemaletValue::Boolean => {
                    map.serialize_entry("type", "boolean")?;
                }
                SchemaletValue::Array(array) => {
                    map.serialize_entry("type", "array")?;
                    // TODO
                }
                SchemaletValue::Object(schemalet_value_object) => {
                    map.serialize_entry("type", "object")?;
                    let SchemaletValueObject {
                        properties,
                        additional_properties,
                        ..
                    } = schemalet_value_object;

                    let properties = properties
                        .iter()
                        .map(|(k, v)| {
                            (
                                k,
                                ThingPrinter {
                                    graph,
                                    schema_ref: v.clone(),
                                },
                            )
                        })
                        .collect::<BTreeMap<_, _>>();

                    map.serialize_entry("properties", &properties)?;

                    if let Some(additional_properties) = additional_properties {
                        map.serialize_entry("additionalProperties", &additional_properties)?;
                    }
                }
                SchemaletValue::String(s) => {
                    map.serialize_entry("type", "string")?;
                    if !s.pattern.is_empty() {
                        map.serialize_entry("pattern", &s.pattern)?;
                    }
                    if !s.format.is_empty() {
                        map.serialize_entry("format", &s.format)?;
                    }
                    if let Some(min_length) = s.min_length {
                        map.serialize_entry("minLength", &min_length)?;
                    }
                    if let Some(max_length) = s.max_length {
                        map.serialize_entry("maxLength", &max_length)?;
                    }
                }
                SchemaletValue::Integer {
                    minimum: _,
                    exclusive_minimum: _,
                } => {
                    map.serialize_entry("type", "integer")?;
                }
                SchemaletValue::Number {
                    minimum: _,
                    exclusive_minimum: _,
                    maximum: _,
                    exclusive_maximum: _,
                    multiple_of,
                } => {
                    map.serialize_entry("type", "number")?;
                }
                SchemaletValue::Null => todo!(),
            },
        }

        map.end()
    }
}

pub fn schemalet_print(graph: &BTreeMap<SchemaRef, CanonicalSchemalet>, schema_ref: &SchemaRef) {
    let tp = ThingPrinter {
        graph,
        schema_ref: schema_ref.clone(),
    };

    println!("{}", serde_json::to_string_pretty(&tp).unwrap());
}
