// Copyright 2025 Oxide Computer Company

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
    #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
    pub struct CustomMap<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> CustomMap<K, V> {
        fn is_empty(&self) -> bool {
            false
        }
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

// ========================================================================
// Runtime serde integration tests for our fork fixes
// ========================================================================

mod int_or_str {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/codegen_int_or_str.rs"));
}

mod required_defaults {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/codegen_required_defaults.rs"));
}

mod dscp {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/codegen_dscp.rs"));
}

mod small_range {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/codegen_small_range.rs"));
}

mod comparator {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/codegen_comparator.rs"));
}

mod any_of_mixed {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/codegen_any_of_mixed.rs"));
}

mod not_types {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/codegen_not_types.rs"));
}

// --- PR #991: Integer before Number in untagged enums ---

#[test]
fn test_int_or_str_integer_deserialization() {
    let v: int_or_str::IntOrStr = serde_json::from_str("42").unwrap();
    assert!(matches!(v, int_or_str::IntOrStr::Integer(42)));
}

#[test]
fn test_int_or_str_string_deserialization() {
    let v: int_or_str::IntOrStr = serde_json::from_str(r#""hello""#).unwrap();
    assert!(matches!(v, int_or_str::IntOrStr::String(_)));
}

#[test]
fn test_int_or_str_roundtrip() {
    let original = int_or_str::IntOrStr::Integer(99);
    let json = serde_json::to_string(&original).unwrap();
    let back: int_or_str::IntOrStr = serde_json::from_str(&json).unwrap();
    assert!(matches!(back, int_or_str::IntOrStr::Integer(99)));
}

// --- PR #918: Default impl for required fields with defaults ---

#[test]
fn test_required_with_defaults_default_impl() {
    let d = required_defaults::RequiredWithDefaults::default();
    assert_eq!(d.name, "unnamed");
    assert_eq!(d.count, 0);
    assert!(d.label.is_none());
}

#[test]
fn test_required_with_defaults_deserialize_empty() {
    let v: required_defaults::RequiredWithDefaults = serde_json::from_str("{}").unwrap();
    assert_eq!(v.name, "unnamed");
    assert_eq!(v.count, 0);
}

#[test]
fn test_required_with_defaults_deserialize_partial() {
    let v: required_defaults::RequiredWithDefaults =
        serde_json::from_str(r#"{"name": "foo"}"#).unwrap();
    assert_eq!(v.name, "foo");
    assert_eq!(v.count, 0);
}

// --- PR #986: TryFrom for bounded integers ---

#[test]
fn test_dscp_try_from_valid() {
    assert!(dscp::Dscp::try_from(42u8).is_ok());
    assert_eq!(*dscp::Dscp::try_from(42u8).unwrap(), 42);
}

#[test]
fn test_dscp_try_from_boundary() {
    assert!(dscp::Dscp::try_from(0u8).is_ok());
    assert!(dscp::Dscp::try_from(63u8).is_ok());
    assert!(dscp::Dscp::try_from(64u8).is_err());
    assert!(dscp::Dscp::try_from(255u8).is_err());
}

#[test]
fn test_dscp_deserialize_valid() {
    let d: dscp::Dscp = serde_json::from_str("42").unwrap();
    assert_eq!(*d, 42);
}

#[test]
fn test_dscp_deserialize_out_of_range() {
    assert!(serde_json::from_str::<dscp::Dscp>("64").is_err());
    assert!(serde_json::from_str::<dscp::Dscp>("255").is_err());
}

// --- PR #975: Integer width selection ---

#[test]
fn test_small_range_uses_narrow_type() {
    // [1..32] should use NonZeroU8, not NonZeroU64
    let v = small_range::SmallRange::try_from(std::num::NonZeroU8::new(1).unwrap());
    assert!(v.is_ok());
    let v = small_range::SmallRange::try_from(std::num::NonZeroU8::new(32).unwrap());
    assert!(v.is_ok());
    let v = small_range::SmallRange::try_from(std::num::NonZeroU8::new(33).unwrap());
    assert!(v.is_err());
}

#[test]
fn test_small_range_deserialize() {
    let v: small_range::SmallRange = serde_json::from_str("16").unwrap();
    assert_eq!(v.get(), 16);
    assert!(serde_json::from_str::<small_range::SmallRange>("0").is_err());
    assert!(serde_json::from_str::<small_range::SmallRange>("33").is_err());
}

// --- PR #948: Special char variant names ---

#[test]
fn test_comparator_deserialize() {
    let v: comparator::Comparator = serde_json::from_str(r#""=""#).unwrap();
    assert!(matches!(v, comparator::Comparator::Eq));

    let v: comparator::Comparator = serde_json::from_str(r#"">=""#).unwrap();
    assert!(matches!(v, comparator::Comparator::GtEq));

    let v: comparator::Comparator = serde_json::from_str("\"≥\"").unwrap();
    assert!(matches!(v, comparator::Comparator::Gte));

    let v: comparator::Comparator = serde_json::from_str("\"≠\"").unwrap();
    assert!(matches!(v, comparator::Comparator::Neq));

    let v: comparator::Comparator = serde_json::from_str(r#""!=""#).unwrap();
    assert!(matches!(v, comparator::Comparator::BangEq));
}

#[test]
fn test_comparator_roundtrip() {
    for json in [
        r#""=""#, r#"">""#, r#""<""#, "\"≥\"", r#"">=""#, "\"≤\"", r#""<=""#, "\"≠\"", r#""!=""#,
    ] {
        let v: comparator::Comparator = serde_json::from_str(json).unwrap();
        let back = serde_json::to_string(&v).unwrap();
        assert_eq!(json, back);
    }
}

// --- PR #414: anyOf overhaul (no more panic on primitives) ---

#[test]
fn test_any_of_mixed_object() {
    let v: any_of_mixed::AnyOfMixed = serde_json::from_str(r#"{"value": "test"}"#).unwrap();
    assert!(matches!(v, any_of_mixed::AnyOfMixed::Object { .. }));
}

#[test]
fn test_any_of_mixed_string() {
    let v: any_of_mixed::AnyOfMixed = serde_json::from_str(r#""hello""#).unwrap();
    assert!(matches!(v, any_of_mixed::AnyOfMixed::String(_)));
}

#[test]
fn test_any_of_mixed_integer() {
    let v: any_of_mixed::AnyOfMixed = serde_json::from_str("42").unwrap();
    assert!(matches!(v, any_of_mixed::AnyOfMixed::Integer(42)));
}

// --- PR #954: not schema types don't panic ---

#[test]
fn test_not_object_accepts_primitives() {
    // not: { type: "object" } falls back to serde_json::Value
    let _: not_types::NotObject = serde_json::from_str("42").unwrap();
    let _: not_types::NotObject = serde_json::from_str(r#""hello""#).unwrap();
    let _: not_types::NotObject = serde_json::from_str("true").unwrap();
}

#[test]
fn test_array_non_objects() {
    let v: not_types::ArrayNonObjects = serde_json::from_str(r#"[1, "two", true]"#).unwrap();
    assert_eq!(v.len(), 3);
}
