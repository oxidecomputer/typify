// Copyright 2026 Oxide Computer Company

//! The `date_library` option of `import_types!` selects the date types in the
//! expanded code, and two invocations in one build keep their own choices.

mod chrono_types {
    typify::import_types!(
        schema = "tests/schemas/types-with-defaults.json",
        date_library = Chrono,
    );
}

mod jiff_types {
    typify::import_types!(
        schema = "tests/schemas/types-with-defaults.json",
        date_library = Jiff,
    );
}

mod default_types {
    typify::import_types!("tests/schemas/types-with-defaults.json");
}

#[test]
fn chrono_option_generates_chrono_types() {
    let event = chrono_types::EventWithDefault::default();
    let _: chrono::DateTime<chrono::Utc> = event.when;
}

#[test]
fn jiff_option_generates_jiff_types() {
    let event = jiff_types::EventWithDefault::default();
    let _: jiff::Timestamp = event.when;
}

#[test]
fn omitted_option_generates_chrono_types() {
    let event = default_types::EventWithDefault::default();
    let _: chrono::DateTime<chrono::Utc> = event.when;
}
