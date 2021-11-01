use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use schemars::schema::{Metadata, Schema};
use thiserror::Error;
use type_entry::{TypeEntry, TypeEntryNewtype};

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

/// Representation of a type which may have a definition or may be built-in.
#[derive(Debug, Clone)]
pub struct Type<'a>(&'a TypeSpace, &'a TypeEntry);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct TypeId(u64);

#[derive(Debug, Clone, PartialEq)]
pub enum Name {
    Required(String),
    Suggested(String),
    Unknown,
}

/// A collection of types.
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
    type_to_id: BTreeMap<TypeEntry, TypeId>,

    name_to_id: BTreeMap<String, TypeId>,
    ref_to_id: BTreeMap<String, TypeId>,

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
            type_to_id: BTreeMap::new(),
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
                // This is effectively a forward declaration so we can discard
                // the TypeEntry without assigning it. We'd see this if there
                // were a cycle in the type graph.
                TypeEntry::Reference(type_id) => TypeEntryNewtype::from_metadata(
                    Name::Required(type_name.to_string()),
                    metadata,
                    type_id,
                ),

                // The types that are already named are good to go.
                TypeEntry::Enum(_) | TypeEntry::Struct(_) | TypeEntry::Newtype(_) => type_entry,

                // For types that don't have names, this is effectively a type
                // alias which we treat as a newtype (though we could probably
                // handle it as a type alias).
                _ => TypeEntryNewtype::from_metadata(
                    Name::Required(type_name.to_string()),
                    metadata,
                    self.assign_type(type_entry),
                ),
            };
            self.definitions.insert(ref_name, schema);
            self.id_to_entry
                .insert(TypeId(base_id + index as u64), type_entry);
        }
        Ok(())
    }

    /// Add a new type and return a type identifier that may be used in
    /// function signatures or embedded within other types.
    pub fn add_type(&mut self, schema: &Schema) -> Result<Type> {
        let (type_entry, _) = self.convert_schema(Name::Unknown, schema)?;

        let type_id = self.assign_type(type_entry);
        let type_entry = self.id_to_entry.get(&type_id).unwrap();
        Ok(Type(self, type_entry))
    }

    /// Add a new type with a name hint and return a the components necessary
    /// to use the type for various components of a function signature.
    pub fn add_type_with_name(
        &mut self,
        schema: &Schema,
        name_hint: Option<String>,
    ) -> Result<Type> {
        let name = match name_hint {
            Some(s) => Name::Suggested(s),
            None => Name::Unknown,
        };
        let (type_entry, _) = self.convert_schema(name, schema)?;

        let type_id = self.assign_type(type_entry);
        let type_entry = self.id_to_entry.get(&type_id).unwrap();
        Ok(Type(self, type_entry))
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

    /// Set the name of the path prefix for types defined in this [TypeSpace].
    pub fn set_type_mod<S: AsRef<str>>(&mut self, type_mod: S) {
        self.type_mod = Some(type_mod.as_ref().to_string());
    }

    /// Iterate over all types including those defined in this [TypeSpace] and
    /// those referred to by those types.
    pub fn iter_types(&self) -> impl Iterator<Item = Type> {
        self.id_to_entry
            .values()
            .map(|type_entry| Type(self, type_entry))
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Allocated the next TypeId.
    fn assign(&mut self) -> TypeId {
        let id = TypeId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Assign a TypeId for a TypeEntry. This handles resolving references,
    /// checking for duplicate type definitions (e.g. to make sure there aren't
    /// two conflicting types of the same name), and deduplicates various
    /// flavors of built-in types.
    fn assign_type(&mut self, ty: TypeEntry) -> TypeId {
        if let TypeEntry::Reference(type_id) = ty {
            type_id
        } else if let Some(name) = ty.name() {
            // If there's already a type of this name, we make sure it's
            // identical.

            // TODO there are many different choices we might make here
            // that could differ depending on the texture of the schema.
            // For example, a schema might use the string "Response" in a
            // bunch of places and if that were the case we might expect
            // them to be different and resolve that by renaming or scoping
            // them in some way.
            if let Some(type_id) = self.name_to_id.get(name) {
                let existing_ty = self.id_to_entry.get(type_id).unwrap();
                assert_eq!(existing_ty, &ty);
                type_id.clone()
            } else {
                let type_id = self.assign();
                self.name_to_id.insert(name.clone(), type_id.clone());
                self.id_to_entry.insert(type_id.clone(), ty);
                type_id
            }
        } else if let Some(type_id) = self.type_to_id.get(&ty) {
            type_id.clone()
        } else {
            let type_id = self.assign();
            self.type_to_id.insert(ty.clone(), type_id.clone());
            self.id_to_entry.insert(type_id.clone(), ty);
            type_id
        }
    }

    /// Convert a schema to a TypeEntry and assign it a TypeId.
    fn id_for_schema<'a>(
        &mut self,
        type_name: Name,
        schema: &'a Schema,
    ) -> Result<(TypeId, &'a Option<Box<Metadata>>)> {
        let (ty, meta) = self.convert_schema(type_name, schema)?;
        let type_id = self.assign_type(ty);
        Ok((type_id, meta))
    }

    /// Create an Option<T> from a pre-assigned TypeId and assign it an ID.
    fn id_to_option(&mut self, id: &TypeId) -> TypeId {
        self.assign_type(TypeEntry::Option(id.clone()))
    }

    // Create an Option<T> from a TypeEntry by assigning it type.
    fn type_to_option(&mut self, ty: TypeEntry) -> TypeEntry {
        TypeEntry::Option(self.assign_type(ty))
    }
}

impl<'a> Type<'a> {
    /// The name of the type as a String.
    pub fn name(&self) -> String {
        let Type(type_space, type_entry) = self;
        type_entry.type_name(type_space)
    }

    /// The identifier for the type as might be used for a function return or
    /// defining the type of a member of a struct..
    pub fn ident(&self) -> TokenStream {
        let Type(type_space, type_entry) = self;
        type_entry.type_ident(type_space, true)
    }

    /// The identifier for the type as might be used for a parameter in a
    /// function signature. In general: simple types are the same as
    /// [Type::ident] and complex types prepend a `&`.
    pub fn parameter_ident(&self) -> TokenStream {
        let Type(type_space, type_entry) = self;
        type_entry.type_parameter_ident(type_space)
    }

    /// The definition for this type. This will be empty for types that are
    /// already defined such as `u32` or `uuid::Uuid`.
    pub fn definition(&self) -> TokenStream {
        let Type(type_space, type_entry) = self;
        type_entry.output(type_space)
    }

    /// A textual description of the type appropriate for debug output.
    pub fn describe(&self) -> String {
        self.1.describe()
    }
}

#[cfg(test)]
mod tests {
    use schema::Schema;
    use schemars::{schema_for, JsonSchema};
    use serde::Serialize;
    use serde_json::json;
    use std::collections::HashSet;

    use crate::{
        test_util::validate_output,
        type_entry::{TypeEntryEnum, VariantDetails},
        Name, TypeEntry, TypeSpace,
    };

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
            TypeEntry::Enum(TypeEntryEnum { variants, .. }) => {
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
            .convert_enum_string(Name::Required("OnTheGo".to_string()), &None, &enum_values)
            .unwrap();

        if let TypeEntry::Option(id) = &te {
            let ote = type_space.id_to_entry.get(id).unwrap();
            if let TypeEntry::Enum(TypeEntryEnum { variants, .. }) = &ote {
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

    #[test]
    fn test_alias() {
        #[derive(JsonSchema, Schema)]
        struct Stuff(Vec<String>);

        #[allow(dead_code)]
        #[derive(JsonSchema, Schema)]
        struct Things {
            a: String,
            b: Stuff,
        }

        validate_output::<Things>();
    }
}
