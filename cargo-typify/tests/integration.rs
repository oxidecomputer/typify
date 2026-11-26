use expectorate::assert_contents;
use newline_converter::dos2unix;
use tempfile::TempDir;

#[test]
fn test_simple() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("simple.json");
    std::fs::copy(input, &input_file).unwrap();

    let output_file = temp.path().join("simple.rs");

    assert_cmd::cargo::cargo_bin_cmd!()
        .args(["typify", input_file.to_str().unwrap()])
        .assert()
        .success();

    let actual = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/builder.rs", &actual);
}

#[test]
fn test_default_output() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new().unwrap();
    let output_file = temp.path().join("output.rs");

    assert_cmd::cargo::cargo_bin_cmd!()
        .args(["typify", input, "--output", output_file.to_str().unwrap()])
        .assert()
        .success();

    let content = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/builder.rs", &content);
}

#[test]
fn test_no_builder_stdout() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let output = assert_cmd::cargo::cargo_bin_cmd!()
        .args(["typify", input, "--no-builder", "--output", "-"])
        .output()
        .unwrap();

    let output_stdout = String::from_utf8(output.stdout).unwrap();
    let actual = dos2unix(&output_stdout);

    assert!(output.status.success());
    assert_contents("tests/outputs/no-builder.rs", &actual);
}

#[test]
fn test_builder() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new().unwrap();
    let output_file = temp.path().join("output.rs");

    assert_cmd::cargo::cargo_bin_cmd!()
        .args([
            "typify",
            input,
            "--builder",
            "--output",
            output_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let actual = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/builder.rs", &actual);
}

#[test]
fn test_derive() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new().unwrap();
    let output_file = temp.path().join("output.rs");

    assert_cmd::cargo::cargo_bin_cmd!()
        .args([
            "typify",
            input,
            "--no-builder",
            "--additional-derive",
            "::extra::ExtraDerive",
            "--output",
            output_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let actual = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/derive.rs", &actual);
}

#[test]
fn test_attr() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new().unwrap();
    let output_file = temp.path().join("output.rs");

    assert_cmd::cargo::cargo_bin_cmd!()
        .args([
            "typify",
            input,
            "--no-builder",
            "--additional-attr",
            "#[extra_attr]",
            "--output",
            output_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let actual = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/attr.rs", &actual);
}

#[test]
fn test_multi_derive() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new().unwrap();
    let output_file = temp.path().join("output.rs");

    assert_cmd::cargo::cargo_bin_cmd!()
        .args([
            "typify",
            input,
            "--no-builder",
            "--additional-derive",
            "::extra::ExtraDerive",
            "--additional-derive",
            "::extra::AnotherDerive",
            "--output",
            output_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let actual = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/multi_derive.rs", &actual);
}

#[test]
fn test_help() {
    let output = assert_cmd::cargo::cargo_bin_cmd!()
        .args(["typify", "--help"])
        .output()
        .unwrap();

    let output_stdout = String::from_utf8(output.stdout).unwrap();
    let actual = dos2unix(&output_stdout);

    assert!(output.status.success());
    assert_contents("tests/outputs/help.txt", &actual);
}

#[test]
fn test_btree_map() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new().unwrap();
    let output_file = temp.path().join("output.rs");

    assert_cmd::cargo::cargo_bin_cmd!()
        .args([
            "typify",
            input,
            "--map-type",
            "::std::collections::BTreeMap",
            "--output",
            output_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let actual = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/custom_btree_map.rs", &actual);
}

#[test]
fn test_invalid_map_type() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let output = assert_cmd::cargo::cargo_bin_cmd!()
        .args([
            "typify",
            input,
            "--map-type",
            "not a valid!!type",
            "--output",
            "-",
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("invalid map type"), "stderr: {stderr}");
}

fn generate_with_date_library(library: Option<&str>) -> String {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("dates.json");
    std::fs::write(
        &input_file,
        r#"{
            "title": "Event",
            "type": "object",
            "properties": {
                "when": { "type": "string", "format": "date-time" }
            },
            "required": ["when"]
        }"#,
    )
    .unwrap();
    let output_file = temp.path().join("dates.rs");

    let mut args = vec![
        "typify".to_string(),
        input_file.to_str().unwrap().to_string(),
        "--output".to_string(),
        output_file.to_str().unwrap().to_string(),
    ];
    if let Some(library) = library {
        args.push("--date-library".to_string());
        args.push(library.to_string());
    }
    assert_cmd::cargo::cargo_bin_cmd!()
        .args(&args)
        .assert()
        .success();

    std::fs::read_to_string(output_file).unwrap()
}

#[test]
fn test_date_library_defaults_to_chrono() {
    let code = generate_with_date_library(None);
    assert!(code.contains("::chrono::DateTime"), "{code}");
}

#[test]
fn test_date_library_chrono() {
    let code = generate_with_date_library(Some("chrono"));
    assert!(code.contains("::chrono::DateTime"), "{code}");
}

#[test]
fn test_date_library_jiff() {
    let code = generate_with_date_library(Some("jiff"));
    assert!(code.contains("::jiff::Timestamp"), "{code}");
    assert!(!code.contains("chrono"), "{code}");
}

#[test]
fn test_date_library_string() {
    let code = generate_with_date_library(Some("string"));
    assert!(code.contains("pub when: ::std::string::String"), "{code}");
    assert!(!code.contains("chrono"), "{code}");
    assert!(!code.contains("jiff"), "{code}");
}

#[test]
fn test_invalid_date_library() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let output = assert_cmd::cargo::cargo_bin_cmd!()
        .args(["typify", input, "--date-library", "nope", "--output", "-"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("date-library"), "stderr: {stderr}");
}
