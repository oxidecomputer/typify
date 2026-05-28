# typespace

A crate for modeling Rust types for code generation. Consumers build up a
`TypespaceBuilder`, by inserting named `Type<Id>` values, call
`finalize(settings, make_box_id)` to break cycles and propagate trait
requirements, then call `render()` to emit a `TokenStream` of Rust type
definitions, or `to_codespace()` to get a structured `codespace::Codespace`
that can be merged with other output before rendering.

Serde default-value helpers are emitted into a `pub mod defaults` submodule
and named `{snake_case_struct}__{field}` (double underscore separator to avoid
collisions between e.g. `TypeName::foo` and `Type::name_foo`).

## TODO

### Correctness

- **`push_traits` incomplete** — `UnitStruct`, `TupleStruct`, and `TypeAlias`
  all hit `todo!()`, so any type graph that routes through those during
  finalization will panic. `Display`/`FromStr` requirements on container types
  (`Vec`, `Array`, `Tuple`) also hit `todo!()`.

- **`StructPropertyState::Default` mostly unimplemented** — only `Option`,
  `Vec`, `Map`, `Set`, and `String` are handled; `Boolean`, `Integer`, `Float`,
  `JsonValue`, `Enum`, `Struct`, `Box`, `Array`, and `Tuple` all panic with
  `todo!()`.

- **`TypeNewtypeConstraints` is unrendered** — the enum is accepted by
  constructors but ignored entirely by `render()`; callers passing real
  constraints get no effect and no error.

### Settings / Configurability

- **Trait derivation** — `#[derive(Serialize, Deserialize)]` (and `Clone`,
  `Debug` on unit structs) are hardcoded in every render method with
  inconsistent strategies across variants. `TypespaceTrait` and
  `TypespaceTraitSet` already exist; wire them into `TypespaceSettings` so
  callers control which traits appear in the output.

- **Map / Set concrete types** — `Type::Map` always renders as `BTreeMap` and
  `Type::Set` as `BTreeSet` (currently renders as `Vec` with a TODO). Add a
  `TypespaceSettings` field to choose between `BTreeMap`/`HashMap` and
  `BTreeSet`/`HashSet`.

- **Finalize-time trait configuration** — trait inclusion and propagation
  (which traits are *required* of generated types, driving the push/poison walk
  in `finalize`) should be configurable at finalize time, distinct from the
  render-time setting of which traits to emit.

### Cleanup

- **`TypeCommon::default`** — the `default: Option<JsonValue>` field on
  `TypeCommon` is never rendered; it should drive a generated `Default` impl.

- **`TypespaceTrait` / `TypespaceTraitSet` in lib.rs** — currently defined but
  only used inside `push_traits`. The API is incomplete (no `remove`, no set
  operations, no `Display`). Once trait configurability is added these become
  part of the public settings surface; until then, consider keeping them
  `pub(crate)`.

- **Add a proper error type** — `finalize` returns `Result<_, ()>` and several
  methods panic instead of returning errors. Introduce a `TypespaceError` enum.

### Test coverage

- `Type::Vec` and `Type::Box` as top-level named output (currently only tested
  as struct fields)
- `StructPropertyState::Default` field combinations once implemented
- Additional trait validation error cases beyond Float-as-map-key
