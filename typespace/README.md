# typespace

A crate for modeling Rust types for code generation. Consumers build up a
`TypespaceBuilder`, by inserting named `Type<Id>` values, call `finalize()` to
break cycles and propagate trait requirements, then call `render(settings)` to
emit a `TokenStream` of Rust type definitions.

## TODO

### Naming

- **`__default_` helper functions** — default-value serde helpers are named
  `__default_{StructName}_{field}`. This scheme collides if two structs share a field
  name in the same output and is visually noisy. Consider a short hash suffix or a
  per-struct private module.

### Settings / Configurability

- **Trait derivation** — `#[derive(Serialize, Deserialize)]` (and `Clone`, `Debug` on
  unit structs) are hardcoded in every render method. `TypespaceTrait` and
  `TypespaceTraitSet` already exist; wire them into `TypespaceSettings` so callers
  control which traits appear in the output.

- **Map / Set concrete types** — `Type::Map` always renders as `BTreeMap` and
  `Type::Set` as `BTreeSet`. Add a `TypespaceSettings` field to choose between
  `BTreeMap`/`HashMap` and `BTreeSet`/`HashSet`.

- **Finalize-time trait configuration** — trait inclusion and propagation (which traits
  are *required* of generated types, driving the push/poison walk in `finalize`) should
  be configurable at finalize time, distinct from the render-time setting of which
  traits to emit.

### Cleanup

- **`TypeCommon::default`** — the `default: Option<JsonValue>` field on `TypeCommon`
  is never populated or rendered; it should be used to generate a `Default` impl.

- **`TypespaceTrait` / `TypespaceTraitSet` in lib.rs** — currently defined but only used
  inside `push_traits`. Once trait configurability is added these become part of the
  public settings surface; until then, consider whether they belong in the public API.

- Add a proper error type and make relevant methods fallible.

### Test coverage

The following `Type` variants have no render test:
- `TypeEnum` — all four tag types (External, Internal, Adjacent, Untagged)
- `TypeNewtypeStruct`
- `TypeTypeAlias`
- `TypeNative`
- `Type::Map` (valid cases), `Type::Set`, `Type::Array`, `Type::Tuple`
- `Type::Vec`, `Type::Box` as top-level output
- Scalar types as struct/enum fields: `Boolean`, `Integer`, `Float`, `String`,
  `JsonValue`
- `StructPropertySerde::Rename` and `StructPropertySerde::Flatten`
- Trait validation error cases: Float/JsonValue used as map keys
