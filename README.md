# Typify

Compile JSON Schema documents into Rust types. This can be used ...

- via the macro `import_types!("types.json")` to generate Rust types directly
in your program

- via the builder functions to generate Rust types in `build.rs`

- or via the builder functions to generate persistent files e.g. when building
API bindings.


## WIP

This is a work in progress. Here are some clear TODOs:

- The API needs some consideration; the pieces are there, but it could stand
being refined.

- Strings with patterns and max/min lengths aren't carefully considering ATM

Just to not be overwhelmed, but what's not done: there's a lot that's neat!

- Versions of schemars has a behavior (bug?) whereby it spits out `enum`s as
`anyOf` rather than `oneOf`; we detect when all subschemas are mutually
incompatible so that we can treat these as `enum`s.

- All serde enum tagging types are supported.

- Complex tuples are properly handled.

- The test harness is pretty robust, validating that the generated
`TokenStream` *roughly* matches the `TokenStream` of the original item
(enum/struct/type).