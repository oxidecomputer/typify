// Copyright 2026 Oxide Computer Company

// Include the generated code to make sure it compiles.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {}

#[test]
fn test_with_set() {
    // Validate that a set is currently represented as a Vec
    // See type_entry.rs
    let _ = WithSet { set: Vec::new() };
}

#[test]
fn test_ipnetwork() {
    // ipnetwork::IpNetwork is a moderately complex type for us to handle. In
    // particular it's a oneOf with both variants as strings, but with mutually
    // incompatible patterns. This tests that our generated Deserialize impl
    // does the right thing.
    assert!(Ipv4Network::try_from("192.168.0.0/24").is_ok());
    assert!(Ipv6Network::try_from("192.168.0.0/24").is_err());
    assert!(Ipv6Network::try_from("fc00::/7").is_ok());
    assert!(Ipv4Network::try_from("fc00::/7").is_err());

    let v4: IpNetwork = serde_json::from_str(r#""192.168.0.0/24""#).unwrap();
    assert!(matches!(v4, IpNetwork::V4(_)));
    let v6: IpNetwork = serde_json::from_str(r#""fc00::/7""#).unwrap();
    assert!(matches!(v6, IpNetwork::V6(_)));

    let v4 = IpNetwork::try_from("192.168.0.0/24").unwrap();
    assert!(matches!(v4, IpNetwork::V4(_)));
    let v6 = IpNetwork::try_from("fc00::/7").unwrap();
    assert!(matches!(v6, IpNetwork::V6(_)));
}

#[test]
fn test_string_constraints() {
    assert!(LoginName::try_from("").is_err());
    assert!(LoginName::try_from("abcdefghi").is_err());
    assert!(LoginName::try_from("offby1").is_err());
    assert!(LoginName::try_from("ahl").is_ok());
}

#[test]
fn test_string_constraints_for_non_ascii_chars() {
    assert!(NonAsciiChars::try_from("🍔🍔🍔🍔🍔🍔🍔🍔").is_ok());
    assert!(NonAsciiChars::try_from("🍔").is_err());
}

#[test]
fn test_unknown_format() {
    // An unknown format string should just render as a string.
    let _ = UnknownFormat {
        pancakes: String::new(),
    };
}

#[test]
fn test_triple_pattern() {
    // Must satisfy all three patterns simultaneously:
    //   1. ^[a-z].+$        — starts with lowercase
    //   2. ^.{4,8}$         — 4–8 characters long
    //   3. .+[a-z]$         — ends with lowercase

    // Valid: 4 lowercase letters
    assert!(TriplePattern::try_from("abcd").is_ok());
    // Valid: 6 lowercase letters
    assert!(TriplePattern::try_from("abcdef").is_ok());

    // Fails: starts with uppercase
    assert!(TriplePattern::try_from("Abcd").is_err());
    // Fails: ends with uppercase
    assert!(TriplePattern::try_from("abcD").is_err());
    // Fails: too short
    assert!(TriplePattern::try_from("abc").is_err());
    // Fails: too long
    assert!(TriplePattern::try_from("abcdefghijkl").is_err());
}

mod hashmap {
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/codegen_hashmap.rs"));

    #[test]
    fn test_with_map() {
        // Validate that a map is currently represented as a HashMap by default.
        let _ = WithMap {
            map: std::collections::HashMap::new(),
        };
    }
}

mod custom_map {
    #![allow(dead_code)]

    #[allow(private_interfaces)]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct CustomMap<K, V> {
        key: K,
        value: V,
    }

    include!(concat!(env!("OUT_DIR"), "/codegen_custommap.rs"));

    #[test]
    fn test_with_map() {
        // Validate that a map is represented as an CustomMap when requested.
        let _ = WithMap {
            map: CustomMap {
                key: String::new(),
                value: String::new(),
            },
        };
    }
}
