use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use schemars::schema::{Metadata, Schema};
use thiserror::Error;

mod convert;
mod enums;
mod structs;
#[cfg(test)]
mod test_util;
mod type_entry;
mod util;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected value type")]
    BadValue(String, serde_json::Value),
    #[error("unknown")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct TypeEntryIdentifier {
    pub ident: TokenStream,
    pub parameter: TokenStream,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeEntry {
    // TODO all these properties only apply to types we define: enum, struct,
    // newtype. Perhaps we can move these into there.
    name: Option<String>,
    rename: Option<String>,
    description: Option<String>,
    details: TypeDetails,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct TypeId(u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TypeDetails {
    Enum {
        tag_type: EnumTagType,
        variants: Vec<Variant>,
        deny_unknown_fields: bool,
    },
    Struct {
        properties: Vec<StructProperty>,
        deny_unknown_fields: bool,
    },
    Option(TypeId),
    Array(TypeId),
    Map(TypeId, TypeId),
    Tuple(Vec<TypeId>),
    Newtype(TypeId),
    Unit,
    // Built-in complex types with no type generics such as Uuid
    BuiltIn,
    // Primitive types such as integer and floating-point flavors.
    Primitive,
    // Strings... which we handle a little specially.
    String,

    // While these types won't very make their way out to the user, we need
    // reference types in particular to represent simple type aliases between
    // types named as reference targets.
    Reference(TypeId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EnumTagType {
    External,
    Internal { tag: String },
    Adjacent { tag: String, content: String },
    Untagged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Variant {
    name: String,
    rename: Option<String>,
    description: Option<String>,
    details: VariantDetails,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VariantDetails {
    Simple,
    Tuple(Vec<TypeId>),
    Struct(Vec<StructProperty>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StructProperty {
    name: String,
    serde_naming: SerdeNaming,
    serde_rules: SerdeRules,
    description: Option<String>,
    type_id: TypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SerdeNaming {
    None,
    Rename(String),
    Flatten,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SerdeRules {
    None,
    Optional,
}

#[derive(Debug, Clone)]
pub enum Name {
    Required(String),
    Suggested(String),
    Unknown,
}

// TODO we need two String -> Type maps:
// 1. the one for references. these will almost certainly need to be used by
// name
// 2. one for types that we create by necessity; these may not have great
// names. They're types that are embedded within other types and that Rust
// requires us to define as their own type.
// 3. allow for formatting configuration such as the derive macros that are
// included, and the destination for types e.g. into their own mod with a given
// name.

#[derive(Debug)]
pub struct TypeSpace {
    next_id: u64,

    // TODO we need this in order to inspect the collection of reference types
    // e.g. to do `all_mutually_exclusive`. In the future, we could obviate the
    // need this by keeping a single Map of referenced types whose value was an
    // enum of a "raw" or a "converted" schema.
    definitions: BTreeMap<String, Schema>,

    // TODO needs an API
    pub(crate) id_to_entry: BTreeMap<TypeId, TypeEntry>,
    name_to_id: BTreeMap<String, TypeId>,
    ref_to_id: BTreeMap<String, TypeId>,
    id_to_option_id: BTreeMap<TypeId, TypeId>,

    uses_chrono: bool,
    uses_uuid: bool,
    uses_serde_json: bool,
    pub(crate) type_mod: Option<String>,
}

impl Default for TypeSpace {
    fn default() -> Self {
        Self {
            next_id: 1,
            definitions: BTreeMap::new(),
            id_to_entry: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            ref_to_id: BTreeMap::new(),
            id_to_option_id: BTreeMap::new(),
            uses_chrono: false,
            uses_uuid: false,
            uses_serde_json: false,
            type_mod: None,
        }
    }
}

impl TypeSpace {
    /// Add a collection of types that will be used as references. Regardless
    /// of how these types are defined--*de novo* or built-in--these types will
    /// appear in the final output in some form. This method may be called
    /// multiple times, but collections of references must be self-contained;
    /// in other words, a type in one invocation may not refer to a type in
    /// another invocation.
    // TODO on an error the TypeSpace is in a weird state; we, perhaps, create
    // a child TypeSpace and then merge it in once all conversions hae
    // succeeded.
    pub fn add_ref_types<I, S>(&mut self, type_defs: I) -> Result<()>
    where
        I: IntoIterator<Item = (S, Schema)>,
        S: AsRef<str>,
    {
        // Gather up all types to make things a little more convenient.
        let definitions = type_defs
            .into_iter()
            .map(|(name, schema)| (name.as_ref().to_string(), schema))
            .collect::<Vec<(String, Schema)>>();

        // Assign IDs to reference types before actually converting them. We'll
        // need these in the case of forward (or circular) references.
        let base_id = self.next_id;
        self.next_id += definitions.len() as u64;

        for (index, (ref_name, _)) in definitions.iter().enumerate() {
            self.ref_to_id
                .insert(ref_name.to_string(), TypeId(base_id + index as u64));
        }

        // Convert all types; note that we use the type assigned from the
        // previous step because each type may create additional types.
        for (index, (ref_name, schema)) in definitions.into_iter().enumerate() {
            let type_name = match ref_name.rfind('/') {
                Some(idx) => &ref_name[idx..],
                None => &ref_name,
            };

            let (type_entry, metadata) =
                self.convert_schema(Name::Required(type_name.to_string()), &schema)?;
            let type_entry = match type_entry {
                TypeEntry {
                    name: None,
                    rename: None,
                    description: None,
                    details: TypeDetails::Reference(type_id),
                } => TypeEntry::from_metadata(
                    Name::Required(type_name.to_string()),
                    metadata,
                    TypeDetails::Newtype(type_id),
                ),
                _ => type_entry,
            };
            self.definitions.insert(ref_name, schema);
            self.id_to_entry
                .insert(TypeId(base_id + index as u64), type_entry);
        }
        Ok(())
    }

    /// Add a new type and return a type identifier that may be used in
    /// function signatures or embedded within other types.
    pub fn add_type(&mut self, schema: &Schema) -> Result<TokenStream> {
        let (type_entry, _) = self.convert_schema(Name::Unknown, schema)?;

        let type_id = self.assign_type(type_entry);
        let type_entry = self.id_to_entry.get(&type_id).unwrap();
        Ok(type_entry.type_ident(self, true))
    }

    /// Add a new type and return a the components necessary to use the type
    /// for various components of a function signature.
    pub fn add_type_details(&mut self, schema: &Schema) -> Result<TypeEntryIdentifier> {
        let (type_entry, _) = self.convert_schema(Name::Unknown, schema)?;

        let type_id = self.assign_type(type_entry);
        let type_entry = self.id_to_entry.get(&type_id).unwrap();
        Ok(type_entry.type_ident_details(self))
    }

    /// Add a new type with a name hint and return a the components necessary
    /// to use the type for various components of a function signature.
    pub fn add_type_details_with_name(
        &mut self,
        schema: &Schema,
        name_hint: Option<String>,
    ) -> Result<TypeEntryIdentifier> {
        let name = match name_hint {
            Some(s) => Name::Suggested(s),
            None => Name::Unknown,
        };
        let (type_entry, _) = self.convert_schema(name, schema)?;

        let type_id = self.assign_type(type_entry);
        let type_entry = self.id_to_entry.get(&type_id).unwrap();
        Ok(type_entry.type_ident_details(self))
    }

    pub fn uses_chrono(&self) -> bool {
        self.uses_chrono
    }

    pub fn uses_uuid(&self) -> bool {
        self.uses_uuid
    }

    pub fn uses_serde_json(&self) -> bool {
        self.uses_serde_json
    }

    pub fn set_type_mod<S: AsRef<str>>(&mut self, type_mod: S) {
        self.type_mod = Some(type_mod.as_ref().to_string());
    }

    pub fn iter_types(&self) -> impl Iterator<Item = &TypeEntry> {
        self.id_to_entry.values()
    }

    // Private interface?

    fn assign(&mut self) -> TypeId {
        let id = TypeId(self.next_id);
        self.next_id += 1;
        id
    }

    fn assign_type(&mut self, ty: TypeEntry) -> TypeId {
        if let TypeDetails::Reference(type_id) = &ty.details {
            // The underlying type is already assigned so we just need to
            // return that type ID.
            type_id.clone()
        } else if let Some(name) = ty.name.clone() {
            // If there's already a type of this name, we make sure it's
            // identical.
            // TODO there are many different choices we might make here that
            // could differ depending on the texture of the schema.
            if let Some(type_id) = self.name_to_id.get(&name) {
                let existing_ty = self.id_to_entry.get(type_id).unwrap();
                assert_eq!(existing_ty, &ty);
                type_id.clone()
            } else {
                let type_id = self.assign();
                self.id_to_entry.insert(type_id.clone(), ty);
                self.name_to_id.insert(name.clone(), type_id.clone());
                type_id
            }
        } else {
            let type_id = self.assign();
            self.id_to_entry.insert(type_id.clone(), ty);
            type_id
        }
    }

    fn id_for_schema<'a>(
        &mut self,
        type_name: Name,
        schema: &'a Schema,
    ) -> Result<(TypeId, &'a Option<Box<Metadata>>)> {
        let (ty, meta) = self.convert_schema(type_name, schema)?;
        let type_id = self.assign_type(ty);
        Ok((type_id, meta))
    }

    fn id_for_option(&mut self, id: &TypeId) -> TypeId {
        if let Some(id) = self.id_to_option_id.get(id) {
            id.clone()
        } else {
            let ty = TypeEntry {
                name: None,
                rename: None,
                description: None,
                details: TypeDetails::Option(id.clone()),
            };
            let type_id = self.assign();
            self.id_to_option_id.insert(id.clone(), type_id.clone());
            self.id_to_entry.insert(type_id.clone(), ty);

            type_id
        }
    }

    fn type_to_option(&mut self, ty: TypeEntry) -> TypeEntry {
        let type_id = self.assign_type(ty);

        // TODO: this is bad b/c I'm not recording this option in `id_to_option`
        TypeEntry {
            name: None,
            rename: None,
            description: None,
            details: TypeDetails::Option(type_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use schemars::{schema_for, JsonSchema};
    use serde::Serialize;
    use serde_json::json;
    use std::collections::HashSet;

    use crate::{Name, TypeDetails, TypeEntry, TypeSpace, VariantDetails};

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema)]
    struct Blah {
        blah: String,
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema)]
    #[serde(rename_all = "camelCase")]
    //#[serde(untagged)]
    //#[serde(tag = "type", content = "content")]
    enum E {
        /// aaa
        A,
        /// bee
        B,
        /// cee
        //C(Vec<String>),
        C(Blah),
        /// dee
        D {
            /// double D
            dd: String,
        },
        // /// eff
        // F(
        //     /// eff.0
        //     u32,
        //     /// eff.1
        //     u32,
        // ),
    }

    #[allow(dead_code)]
    #[derive(JsonSchema)]
    #[serde(rename_all = "camelCase")]
    struct Foo {
        /// this is bar
        #[serde(default)]
        bar: Option<String>,
        baz_baz: i32,
        /// eeeeee!
        e: E,
    }

    #[test]
    fn test_simple() {
        let schema = schema_for!(Foo);
        println!("{:#?}", schema);
        let mut type_space = TypeSpace::default();
        type_space.add_ref_types(schema.definitions).unwrap();
        let (ty, _) = type_space
            .convert_schema_object(Name::Unknown, &schema.schema)
            .unwrap();

        println!("{:#?}", ty);

        let out = ty.output(&type_space);
        println!("{}", out);

        for ty in type_space.id_to_entry.values() {
            println!("{:#?}", ty);
            let out = ty.output(&type_space);
            println!("{}", out);
        }
    }

    #[test]
    fn test_convert_enum_string() {
        #[allow(dead_code)]
        #[derive(JsonSchema)]
        #[serde(rename_all = "camelCase")]
        enum SimpleEnum {
            DotCom,
            Grizz,
            Kenneth,
        }

        let schema = schema_for!(SimpleEnum);
        println!("{:#?}", schema);

        let mut type_space = TypeSpace::default();
        type_space.add_ref_types(schema.definitions).unwrap();
        let (ty, _) = type_space
            .convert_schema_object(Name::Unknown, &schema.schema)
            .unwrap();

        match ty {
            TypeEntry {
                details: TypeDetails::Enum { variants, .. },
                ..
            } => {
                for variant in &variants {
                    assert_eq!(variant.details, VariantDetails::Simple);
                }
                let var_names = variants
                    .iter()
                    .map(|variant| variant.name.clone())
                    .collect::<HashSet<_>>();
                assert_eq!(
                    var_names,
                    ["DotCom", "Grizz", "Kenneth",]
                        .iter()
                        .map(ToString::to_string)
                        .collect::<HashSet<_>>()
                );
            }
            _ => {
                let out = ty.output(&type_space);
                println!("{}", out);
                panic!();
            }
        }
    }

    #[test]
    fn test_string_enum_with_null() {
        let enum_values = vec![
            json!("Shadrach"),
            json!("Meshach"),
            json!("Abednego"),
            json!(null),
        ];

        let mut type_space = TypeSpace::default();
        let (te, _) = type_space
            .convert_enum_string(Name::Unknown, &None, &enum_values)
            .unwrap();

        if let TypeDetails::Option(id) = &te.details {
            let ote = type_space.id_to_entry.get(id).unwrap();
            if let TypeDetails::Enum { variants, .. } = &ote.details {
                let variants = variants
                    .iter()
                    .map(|v| match v.details {
                        VariantDetails::Simple => v.name.clone(),
                        _ => panic!("unexpected variant type"),
                    })
                    .collect::<HashSet<_>>();

                assert_eq!(
                    variants,
                    enum_values
                        .iter()
                        .flat_map(|j| j.as_str().map(ToString::to_string))
                        .collect::<HashSet<_>>()
                );
            } else {
                panic!("not the sub-type we expected {:#?}", te)
            }
        } else {
            panic!("not the type we expected {:#?}", te)
        }
    }
}
