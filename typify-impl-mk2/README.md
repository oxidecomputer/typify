# Typify mk. 2

## Naming

There are several ways we might get a name for a type.

- User provided / indicated
- `title` field
- context from parent


## trait impls

What traits should a type implement? We should have a structured list that are
under consideration. For example `::serde::Serialize`, `::std::marker::Copy`,
and `::std::cmp::Ord`. We have some set by default. We let users opt out or
specify their own set. And they can add additional traits to derive.

For type replacements, users can (should?) specify which of the known (or
desired) traits the type implements.

Interesting idea: what if some traits are "as needed"? So for example, if we
have a schema like this:

```json
{
  "type": "array",
  "uniqueItems": true,
  "items": { "$ref": "#/$defs/Foo" }
}
```

We might say "`Foo` has to be `Ord` (or `Hash` + `Eq` depending on the set
type). But perhaps we *only* try to implement `Ord` for types that are used in
the context of a set. For trait propagation we would start with all the desired
traits and then forward-propagate for implied-required traits and
backward-poison for traits that cannot be satisfied (such as `Ord` for a
`f64`).


## Extension notes

I've been thinking about the various ways we might want to annotate a schema.
I'm going to try to use this space to record them.

### `oneOf` → exclusive

For a coherent `enum` type, we would like all the variants to be mutually
exclusive in terms of serialization and deserialization. This is a constraint
beyond what Rust and `serde` enforce, but we believe it's an important one for
generative type modeling. In particular, consumers of generated types should
have exactly one way to express or consume a value. Imagine a given conceptual
value that could appear in more than one variant--this would be confusing and
hard to deal with!

The `oneOf` JSON schema construct validates if *exactly* one subschema
validates. This means that if the subschemas are *not* mutually exclusive then
a value at the intersection is actually not valid. As such, we need to generate
variants for each subschema that excludes all other subschemas. Unfortunately,
this isn't possible in the general case. Consider, for example, a schema like
this:

```json
{
	"oneOf": [
    {
      "type": "string",
      "format": "uuid"
    }, {
      "type": "string",
      "pattern": "^12345678(-1234)*-12345678$"
    }
  ]
}
```

Are these mutually exclusive? They are! I can tell that and one can imagine a
computer program that can tell that, but the general case with two regexes
is--as far as I can tell--intractably hard.

For that reason (and to accommodate imprecise schemas), we need a way to
indicate that all subschemas of a `oneOf` are, in fact, mutually exclusive. We
do this with a JSON schema extension `x-oneOfExclusive` in the same object as
the `oneOf`. It's valid values are `"known"` ("known to be mutually
exclusive"), `"override"` ("treat to be mutually exclusive even if
programmatically we can tell that they are not"), and `"unknown"` ("don't know
if they are mutually exclusive"--the default if no extension is present).

```json
{
	"oneOf": [
    {
      "type": "string",
      "format": "uuid"
    }, {
      "type": "string",
      "pattern": "^12345678(-1234)*-12345678$"
    }
  ],
  "x-oneOfExclusive": "known"
}
```

As with other extensions, we allow users to specify the default disposition.
For example, rather than patching each specific use of `oneOf` a user can
indicate that mutual-exclusivity is implicit for all such constructs.


## Log

Note that much of this was transferred over from the "bundler" prototype. Once
we leave the comment-annotated section, it's after the creation of this "mk2"
fork:

```rust
// 11/26/2024 I'm struggling with a dilemma:
//
// I can model the IR in two distinct ways:
//
// Self-Contained
// --------------
//
// Each Schema would contain SchemaRef notes that, effectively, index into some
// lookup table of resolved IR. To process an IR, I'd make sure its
// dependencies were already resolved (and if not, defer the current IR and
// schedule those). What I'm not sure of is ... then what? Would I try to
// stitch the schemas back together into a deeper form? Does that even make
// sense?
//
// Maybe it makes sense to think about the canonical form I expect to get
// after processing the IR?
//
// Deep Trees
// ----------
//
// The other approach is to more faithfully represent the input tree as a deep
// structure. To turn this into an IR, we'd probably need to chew on it until
// we hit something that wasn't resolved, then back out, scheduled the
// dependent work, and pick it up again. It seems sort of inefficient... but
// maybe it's not so bad.
//
// 5/9/2025
// I made this decision effectively a while ago, but we've picked
// self-contained. In the new iteration I'm trying, I'm calling these
// "Schemalets".
//
// 5/9/2025
// What are the options for dealing with dynamic references? When we walk the
// graph the first time, the context comes along for the ride which might be
// useful for identifying the appropriate dynamic reference target.
// Alternatively, we could record dynamic inputs and outputs, stitching it all
// together later. While I was hoping to have discrete passes with distinct
// functionality, perhaps dealing with dyn refs on the first pass is simplest.

// 5/9/2025
// Starting yet another attempt that I'm hoping can be cleaner and more
// complete. We're going to try to blaze it all the way through to a canonical
// representation and take the shortest route with dynamic references.

// 6/14/2025
// Working on the "schemalet" model and it's ok. I think I've lost track of
// preserving metadata and of simplifying the canonical name of a schema e.g.
// if merging schemas results in exactly one or the other. Some of this might
// be fixable incrementally, so I really need to push through.
// The "CanonicalSchemalet" structure seems really right. I expect that's going
// to be the output and I should spend time making that thing pretty precisely
// what I need.

// 6/21/2025
// Some new thoughts after not working on this for a week. Increasingly, I've
// come to realize that what we have is a graph (potentially, with cycles), and
// I need to think about this as local graph optimization and modification. So
// for each simplification step, I should have access to the full graph. Each
// node needs more than the one distinction I have today (canonical or not).
// For example, when simplifying an object, I want to say "might any of your
// required properties turn out to be non-satisfiable (never)? If not, then I
// don't care that some properties are non-canonical, I can declare you
// canonical." Other criteria might include "do I know your type?". That said,
// I want to burn through this current draft and see if I can actually start
// generating some code, and figure out the right layering of the various
// pieces.
// Also [6/23/2025] we can keep track of back edges to know what updating a
// node might unblock in terms of outstanding work.

// 6/21/2025
// It's not pretty, but everything is in a canonical form. The next step is to
// do what I think of as the work of typify: translating schemas to types.
//
// - Raw JSON schemas -> schema graph of canonical schemalets.
// - schema graph -> IR for Rust types
// - IR -> generated code

// 6/22/2025
// What types deserve to have a name regardless of whether it's necessary?
// Currently, we pull out the contents of definitions. In this new version,
// definitions are just... another path. But perhaps that's still a good way to
// determine when type names are meaningful.

// 6/23/2025
// Let's flesh out this idea of the pipeline / layers. In particular, what do
// we start with? I assume a consumer is going to make a Bundle with some
// document or documents. And then it's going to specify some collection of
// types to generate using that bundle as source information (either by saying
// "this path, and that path" or "all the $defs" or "matching this pattern").
// In the case of progenitor, each added type is going to require some response
// so the generated type can later be used. This seems easy enough by using the
// SchemaRef.
//
// Either we'll take that "bundle + type specifiers" and destructively convert
// it to some sort of collection of types or shove them through one at a
// time--I'm not sure it matters. Either in serial items or serial batches,
// we'll convert from Raw -> canonical -> type IR
//
// The real question I'm noodling on is "can I have some object whose purpose
// is to produce type structures for all of its input schemas?". Like, I build
// it with this graph of canonical schemas and it just chews through them.

// 6/24/2025
// Two neat ideas:
// 1. (simple) make a printer and/or transformer that serializes them into a
// single object i.e. inlining references. This would obviously be a problem
// for circular references... so may be I infer as much from the "Reference"
// node. Anyhow: print it out as one big chunk.
// 2. Figure out a more robust definition of the Reference type. And maybe add
// some sort of "Replace Me" node for situations where a computed type ends up
// resolving down to some known type and there's no additional information
// added.

// 6/27/2025
// Making good progress, but also kicking over a bunch of stuff along the way.
// I'm really trying to put my head down to get some code generated and then go
// back and think about all the problems. Some notes:
//
// [ ] I should get really robust about the preservation of metadata and make
//     it clear in the canonical output how that will be represented.
//
// [ ] I need to figure out the actual mechanism for resolving name conflicts
//     and injecting new names. Like: how do we detect a name conflict? How do
//     we report it to the user? What does the user do specifically to fix it?
//
// [ ] I need to preserve extensions and I have no idea how I'm going to do
//     that. Maybe I **don't** need to at all because I can always go back to
//     the original schema to see if they exist? But How does that work when I
//     merge schemas?
//
// [ ] How do I know when a type is going to have its own name or not? This
//     seems most relevant for enum struct-type variants, but it seems like we
//     could also do something for tuple fields (i.e. inline them if there
//     isn't going to be some good name).
//
// I'd just note the things that this does maybe already address or is on the
// way to addressing:
// - I think I can be a lot more robust and efficient wrt merging
// - I am keeping around a bunch of breadcrumbs that should let me make better
//   errors (I just need to figure out the details)
// - Multiple files
// - Multiple JSON Schema specifications
// - Generalized references (i.e. not just to $defs)

// 7/14/2025
// There's definitely something working here end-to-end! I think the next steps
// are to package it up a little more cleanly. One of the things I'm struggling
// with is the various concepts and naming. In particular, I want to figure out
// what I want to call things externally--I think that's going to unblock me.
//
// Here are the various concepts and phases:
//
// The Bundler is what manages a collection of inputs. I generally think of
// these as JSON, but they'll be derived from yaml in the future as well. This
// concept will be part of the external interface--it might even be a separate
// crate--but it shouldn't be obtrusive if someone just wants to do JSON text
// -> types.
//
// JSON -> IR -> Canonical: For want of a better term, I'll call this next part
// the Optimizer. What I have today is going to need some work, but it's not
// that bad. This builds up a couple of graphs that I think I need to keep
// around until all the types the user wants have been added. It's conceivable
// that this too is outward facing or has its own crate--but that's down the
// track. I'm convinced that this canonical format is going to be useful to
// other code generation tools, but let's prove it for this one first.
//
// The Converter takes this canonical representation and converts them to
// types. I think it's basically a pure function and doesn't require mutability
// or even caching (the caching should--effectively--be done by the Optimizer).
//
// The Typespace is the high-order output of this whole thing: a structure that
// manages types. Users can interrogate the types and emit code. This is
// another subcomponent that feels generic: it could be used for generating
// types derived from some other source e.g. TypeSpec.
//
// Where does this leave us with the name of the WHOLE thing? After a little
// chat with an LLM, I think I'm going to call it "Typify". Also some good
// renaming ideas for concepts above:
//
// - Input aggregator: SchemaBundle
// - Schema → canonical IR: Normalizer
// - Canonical → types: Converter
// - Type registry/codegen: Typespace
// - Top-level user-facing API: Typify
//
// SchemaBundle is going to be public in some fashion--progenitor at least will
// need to interact with it. As will cargo-typify.
// The Normalizer seems useful, but it's fine for it to start private. The only
// interesting public-ish use is going to be in testing where we want to print
// the canonical IR.
// The Converter is 100% the point of typify and is an internal concept.
// Typespace is another external concept. Also something I expect progenitor is
// going to interact with when asking programmatic questions about generated
// types.
```

### 7/21/2025

It would be really cool if we could customize the way we generate types. Facile
in the abstract so let me make it concrete: it would be cool for numbers
(integers or floats) to be able to say: let's use `serde_json::Number` rather
than, say, `f64` or `i64`. In particular I think I want this for type
generation from spec schemas.

### 7/25/2025

Interesting realization: `serdes`'s conflation of absent and `null`-values
means that we've been pretty loose with how we generate code. I think that we
can do much better. It might be worth just implementing that in typify as it
exists today, but I'm inclined to see it through here instead since I'm trying
to get to the future as fast as possible.

There are 3 cases we need to break out:

#### Non-`null` / not required

```rust
struct Foo {
    #[serde(
        default,
        deserialize_with = "deserialize_some",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    foo: Option<String>,
}

fn deserialize_some<'de, T, D>(de: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(de).map(Some)
}
```

If the field is absent we'll get `None` and if the field is present, we'll get
`Some(..)`, but `null` produces an error.

Note that for types such as `Vec` and `BTreeMap` we don't need the `Option`
wrapper. (Though for those types I'm not sure about the situations in which we
skip serializing.)

#### Required / `null` permitted

```rust
struct Foo {
    #[serde(deserialize_with = "Option::deserialize")]
    #[schemars(schema_with = "Option::<String>::json_schema")]
    foo: Option<String>,
}
```

Sort of cute, but simply specifying `deserialize_with` causes the derivation of
`serde::Deserialize` to omit its absent field handling that normally triggers
for `Option`s. We don't need to change serialization in any way because it will
generate a `null` for `None` by default.

#### Distinct absent / `null` / value

Two options here:

```rust
struct Foo {
    #[serde(
        default,
        deserialize_with = "deserialize_some",
        skip_serializing_if = "Option::is_none"
    )]
    foo: Option<Option<String>>,
}
```

Absent: `None`; `null`: `Some(None)`; value: `Some(Some(..))`. No augmentation
of `schemars` is required.

Alternatively:

```rust
#[derive(Default, JsonSchema)]
#[schemars(with = "Option<T>")]
pub enum OptionField<T> {
    #[default]
    Absent,
    Null,
    Value(T),
}

impl<'de, T> Deserialize<'de> for OptionField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match Option::<T>::deserialize(deserializer)? {
            Some(v) => OptionField::Value(v),
            None => OptionField::Null,
        })
    }
}

struct Foo {
    #[serde(default, skip_serializing_if = "OptionField::is_absent")]
    foo: OptionField<String>,
}
```

The `Deserialize` is basically the same as `deserialize_some`, and it's only
invoked if the field is present (because of the `#[serde(default)]` attribute).
We use the schema of `Option<T>` verbatim.

#### Summary

Proper handling of the first two cases isn't awkward to use and only involves
one function and proper annotations. I think we should do this by default. I
could imagine a generation option that means "let's be less fussy and just
great all cases as `Option`" (i.e. as we do today).

For the third case, I think it's a bit fussy to be the default. We might have a
way to opt in generally or per container (by name?). I imagine we'd have
something like:
`null_or_absent = Option | DoubleOption | ::helper_crate::OptionField` (where
`Option<T>` would be the default).
Feels kind of neat: let people specify the
type they want to use (though I guess we'd have to special-case
`Option<Option<T>>` to know that we need the additional annotation).

These concepts feel independent and complementary. This `null_or_absent` feels
right (though not urgent); I'm not sure how (or if) we configure other
concepts.

### 7/31/2025

One more thought about the optional / required / null stuff above. What about
arrays and maps, and what significance do we derive from the presence of
`default`?

#### Arrays and maps

In v1 we would render an optional array like this:

```rust
struct Foo {
    #[serde(default, skip_serialization_if = "Vec::is_empty")]
    bar: Vec<String>
}
```

And that's totally fine, but what if someone wanted to distinguish between the
presence or absence of `bar`? This would be wrong:


```rust
struct Foo {
    #[serde(default, skip_serialization_if = "Option::is_none")]
    bar: Option<Vec<String>>
}
```

In particular, it would be wrong because it permits a `null` value. To correct,
that, we can do this instead:

```rust
struct Foo {
    #[serde(
        default,
        deserialize_with = "deserialize_some",
        skip_serializing_if = "Option::is_none"
    )]
    bar: Option<Vec<String>>
}
```

This is the same as we propose above for optional, non-`null` values. But how
to infer that the presence or absence of the field is significant?

#### Handling of `default`

This brings us to the handling of `default`.

One interpretation of the presence of `default` is that it might indicate that
an absent field is equivalent to a particular value and, therefore, the
presence or absence isn't that important.

This seems to bear out in the 2020-12 meta schema: `properties` has a default
(`{}`) and the absence of that field is not important. Conversely the schemas
for `type` and `prefixItems` do not have a default and its absence clearly is
important.

Unsurprisingly `schemars` doesn't seem to have a particularly consistent
position here.

Perhaps this is just going to be something we can't accurately infer.

### 7/31/2025

#### Current state and what's next

If feels like there's good progress, and also that I'm not sure if what we have
is heading in the right direction. Clearly the normalizer stuff is a bit of a
mess and will need a rewrite, but getting it right was probably not possible
without first getting it wrong. I think the converter and type representations
are probably pretty decent. And the schema ingestion--while incomplete and
verbose--is probably pretty decent.

So what now? I think we can start adding some more schema test cases and unit
tests. Then see where we are.

### 11/14/2025

Trying to get back into it and assess where we are. A lot of this seems pretty
decent. I've been able to add some more test cases and migrate to a
non-bootstrap schema, and things have worked surprisingly well. I've bumped
into some areas that definitely require additional consideration:

#### Naming

Naming continues to be not particularly well thought out. For example, how to
properties within `$defs` get their name? How would we make this configurable?
There are many sources of a name: from the path (i.e. name under `$defs`), from
the `title` field, externally, from an extension field, or relatively e.g. from
a parent type. How do each of these sort? Is--for example--the `$defs` name
mandatory, or can it be overruled by either `title` or some extension? We need
to think this through...

#### Newtypes and tuple structs

Currently `Typespace` doesn't have a `Type` to represent either a newtype
(which we had in typify 1) or a named tuple / tuple-like struct. The former is
particularly useful for applying additional constraints, but has also been used
to give names to types that need them e.g. if an array type is under `$defs`. Would we continue to do something like:

```rust
pub struct Foo(Vec<Bar>):
```

Or would we rather have something like:

```rust
pub type Foo = Vec<Bar>;
```

Named tuples / tuple-like structs give us an opportunity to have custom
serialization / deserialization (e.g. to support "flattened" arrays at the end
of tuples).

In the past we've turned top-level tuple types into types like this:


```rust
pub struct Foo(pub (String, String)):
```

That seems strictly worse than:


```rust
pub struct Foo(pub String, pub String);
```

Yeah: `foo.0.0` is much dumber than `foo.0`.

I'm not clear if we want to represent a tuple and tuple-like struct as two
distinct internal types or as one with, say, the presence of a name to
distinguish. Certainly, the need for custom serialization would also require
the struct-tuple.


#### Normalizer v2

It's a bit hacky; I think I can do better.
