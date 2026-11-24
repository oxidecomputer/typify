// Copyright 2026 Oxide Computer Company

//! Runs the generated jiff code, not just compiles it: date-time fields
//! round-trip through serde, and a default date-time parses.

#![cfg(all(feature = "jiff", not(feature = "chrono")))]

#[allow(dead_code, unused_imports)]
#[path = "schemas/jiff/types-with-defaults.rs"]
mod generated;

use generated::EventWithDefault;

#[test]
fn default_date_time_parses() {
    let event = EventWithDefault::default();
    assert_eq!(event.when.to_string(), "1970-01-01T00:00:00Z");
}

#[test]
fn date_time_normalizes_offsets_to_utc() {
    let event: EventWithDefault =
        serde_json::from_str(r#"{"when": "2024-03-05T12:34:56+02:00"}"#).unwrap();
    assert_eq!(event.when.to_string(), "2024-03-05T10:34:56Z");
}

#[test]
fn date_time_round_trips() {
    let json = r#"{"when":"2024-03-05T12:34:56Z"}"#;
    let event: EventWithDefault = serde_json::from_str(json).unwrap();
    assert_eq!(serde_json::to_string(&event).unwrap(), json);
}

#[test]
fn invalid_date_time_is_rejected() {
    assert!(serde_json::from_str::<EventWithDefault>(r#"{"when": "not-a-date"}"#).is_err());
}
