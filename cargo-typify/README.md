# cargo-typify

Once installed, the following command converts a JSON Schema file into Rust
code:

```console
$ cargo typify my_types.json
```

This is a wrapper around the [`typify`](https://crates.io/crates/typify) crate
for use at the command-line.

## Installation

Install with `cargo install cargo-typify`. This command requires that `rustfmt`
is installed. Install rustfmt with rustup component add rustfmt

## Example

**`$ cat id-or-name.json`**

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "definitions": {
    "IdOrName": {
      "oneOf": [
        {
          "title": "Id",
          "allOf": [
            {
              "type": "string",
              "format": "uuid"
            }
          ]
        },
        {
          "title": "Name",
          "allOf": [
            {
              "$ref": "#/definitions/Name"
            }
          ]
        }
      ]
    },
    "Name": {
      "title": "A name unique within the parent collection",
      "description": "Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.",
      "type": "string",
      "pattern": "^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$",
      "maxLength": 63
    }
  }
}
```

**`$ cargo typify id-or-name.json && cat id-or-name.rs`**

```rust
#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdOrName {
    Id(uuid::Uuid),
    Name(Name),
}
impl From<&IdOrName> for IdOrName {
    fn from(value: &IdOrName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdOrName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Id(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Name(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for IdOrName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for IdOrName {
    fn to_string(&self) -> String {
        match self {
            Self::Id(x) => x.to_string(),
            Self::Name(x) => x.to_string(),
        }
    }
}
impl From<uuid::Uuid> for IdOrName {
    fn from(value: uuid::Uuid) -> Self {
        Self::Id(value)
    }
}
impl From<Name> for IdOrName {
    fn from(value: Name) -> Self {
        Self::Name(value)
    }
}
#[doc = "Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID."]
#[derive(Clone, Debug, Serialize)]
pub struct Name(String);
impl std::ops::Deref for Name {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Name> for String {
    fn from(value: Name) -> Self {
        value.0
    }
}
impl From<&Name> for Name {
    fn from(value: &Name) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Name {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 63usize {
            return Err("longer than 63 characters");
        }
        if regress::Regex::new("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
```

## Options

See *`cargo typify --help`* for a complete list of options.

The `--output` option lets you override the default output file (replacing the
input file extension with `.rs`). Use `-` for stdout.

Use `--no-builder` to disable struct builder generation (`--builder` is the
default). Builder output lets you write code like this:

```rust
let xy: MyStruct = MyStruct::builder().x_coord(x).y_coord(y).try_into();
```

The `--additional-derive` adds the specified derive macro to all generated
types. This may be specified more than once.