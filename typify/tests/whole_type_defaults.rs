// Copyright 2026 Oxide Computer Company

mod generated {
    typify::import_types!("tests/schemas/types-with-defaults.json");
}

use generated::{SeparatorConfig, SeparatorHolder};

fn assert_separator_eq(actual: &SeparatorConfig, expected: &SeparatorConfig) {
    assert_eq!(actual.line_color, expected.line_color);
    assert_eq!(actual.line_thickness, expected.line_thickness);
}

#[test]
fn whole_type_defaults_match_deserialization() {
    let holder = SeparatorHolder::default();

    let empty: SeparatorConfig = serde_json::from_value(serde_json::json!({})).unwrap();
    assert_separator_eq(&holder.separator_all, &empty);

    let partial: SeparatorConfig =
        serde_json::from_value(serde_json::json!({ "lineThickness": 5 })).unwrap();
    assert_separator_eq(&holder.separator_some, &partial);
    assert_eq!(holder.separator_some.line_thickness, 5);
}
