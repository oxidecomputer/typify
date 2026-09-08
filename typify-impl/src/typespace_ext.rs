// Copyright 2026 Oxide Computer Company

//! Interface gaps between typify and the typespace crate.
//!
//! This file used to shim proposed typespace interfaces (a `TypeExt`
//! trait with `name`/`set_default`/`dedup_key`, a local `DedupKey`
//! mirror, `EnumVariant::json_name`, and `Enum::all_simple_variants`);
//! typespace now provides all of them and the shims are gone. What
//! remains is the record of gaps that cannot be shimmed from outside
//! the crate; each is annotated at its call site as well.
//!
//! 1. INTERFACE GAP: value-list newtype constraints.
//!    `NewtypeConstraints` is non_exhaustive, so new variants cannot
//!    be added here. typify supports newtypes constrained to a fixed
//!    set of permitted values (`enum` with a non-string type) or a set
//!    of rejected values (`not`/`enum`), rendered as a TryFrom
//!    constructor plus a bespoke Deserialize impl. Proposed:
//!
//!    ```text
//!    pub enum NewtypeConstraints {
//!        None,
//!        EnumValues(Vec<JsonValue>),
//!        DenyValues(Vec<JsonValue>),
//!        String { ... },
//!        Array { ... },
//!    }
//!    ```
//!
//!    Call sites: `TypeEntryNewtype::from_metadata_with_enum_values`
//!    and `from_metadata_with_deny_values` in type_entry.rs, which
//!    currently construct `NewtypeConstraints::None` and drop the
//!    validation.
//!
//! 2. INTERFACE GAP (narrowed): native capability markers.
//!    `TypespaceTrait::Default` now exists and typify maps declared
//!    `Default` impls onto it. The remaining piece is
//!    `FromStringIrrefutable`: typespace tracks that capability
//!    crate-privately (fed by `Native::new_string_like`), so a
//!    consumer-declared irrefutable-FromStr native (from x-rust-type
//!    or a conversion policy) has no typespace home. typify keeps its
//!    `Vec<TypeSpaceImpl>` side channel on `TypeEntry` for that one
//!    marker. With typify's own `has_impl` machinery deleted and the
//!    structural-dedup lookup that used to pair this with typespace's
//!    `DedupKey` also removed, the side channel is unconsumed data
//!    retained at the collection sites; no query can see it. The
//!    query-side loss is recorded in gap 7.
//!
//! 3. INTERFACE GAP (narrowed): rendering of extra derives and
//!    attributes. Every path into typespace exists: the settings-wide
//!    derives go to typespace's `with_derive` (see
//!    `TypeSpace::typespace_settings`), and a
//!    `TypeSpacePatch`'s `with_derive` / `with_attr` values reach the
//!    named shapes' `extra_derives` / `extra_attrs` (see `TypePatch`
//!    in util.rs and its three call sites in type_entry.rs). What
//!    remains is on typespace's side: it renders only the
//!    settings-wide derives, so the settings-wide attrs and both
//!    per-type lists are stored and never emitted.
//!
//! 4. INTERFACE GAP (latent): container-level serde rename. typify1's
//!    renderer supported `#[serde(rename = "...")]` on the container,
//!    though nothing ever populated the field (the patch mechanism
//!    renames the type without preserving the serialized name). The
//!    shapes have `name` but no `rename`; if typespace wants to fix
//!    the underlying bug--a renamed type should keep its wire
//!    name--the shape metadata is where the rename belongs. Call
//!    sites: the INTERFACE GAP comments in `TypeEntry*::from_metadata`
//!    in type_entry.rs.
//!
//! 5. CLOSED (was: pre-finalization identifier rendering). typify's
//!    `TypeEntry::type_ident` / `type_parameter_ident` are gone;
//!    identifier queries go through typespace's view API on the
//!    finalized typespace (`ident`, `ident_in`, `parameter_ident`,
//!    `parameter_ident_in`, `parameter_ident_with_lifetime`), with
//!    scope as a per-query argument. The `type_mod` setting is gone
//!    with them: callers pass the scope where they ask for
//!    identifiers. Behavioral differences from typify1's renderer:
//!    maps render as BTreeMap until `with_map_type` is forwarded to
//!    typespace settings (a separate, gated change); typespace passes
//!    `String` parameters by value where typify1 used `&str`; and
//!    typespace passes all enums by reference where typify1 passed
//!    all-simple-variant enums by value.
//!
//! 6. CLOSED (was: fallible finalization at render time).
//!    `TypeSpace::to_stream` and `TypeSpace::to_typespace` return
//!    `Result`; typespace finalization errors surface as
//!    `Error::Typespace` instead of a panic. Consumers (typify-macro,
//!    cargo-typify, build scripts) propagate or unwrap them.
//!
//! 7. INTERFACE GAP: queries the deleted typify facade answered that
//!    typespace's view API cannot. `view::Type::has_impl` takes
//!    typespace's `TypeSpaceImpl` (Display/FromStr/Eq/Ord/Hash), so
//!    there is no way to ask for `Default` (though
//!    `TypespaceTrait::Default` exists and propagates) or
//!    `FromStringIrrefutable` (crate-private; see gap 2). typify's
//!    enum-analysis answers (all-simple-variant enums implement
//!    Display/FromStr, untagged newtype-variant proxies) went with
//!    `has_impl` and have no typespace equivalent until the bespoke
//!    impls render. Also gone with no replacement: `Type::describe`
//!    (debug summary string) and `Type::builder` (the
//!    `mod builder` ident for struct_builder settings; typespace does
//!    not render builders at all).
