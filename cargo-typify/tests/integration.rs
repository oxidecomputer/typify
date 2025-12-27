use expectorate::assert_contents;
use newline_converter::dos2unix;
use tempdir::TempDir;

#[test]
fn test_simple() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let input_file = temp.path().join("simple.json");
    std::fs::copy(input, &input_file).unwrap();

    let output_file = temp.path().join("simple.rs");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cargo-typify");
    cmd.args(["typify", input_file.to_str().unwrap()])
        .assert()
        .success();

    let actual = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/builder.rs", &actual);
}

#[test]
fn test_default_output() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cargo-typify");
    cmd.args(["typify", input, "--output", output_file.to_str().unwrap()])
        .assert()
        .success();

    let content = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/builder.rs", &content);
}

#[test]
fn test_no_builder_stdout() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cargo-typify");

    let output = cmd
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

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cargo-typify");
    cmd.args([
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

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cargo-typify");
    cmd.args([
        "typify",
        input,
        "--no-builder",
        "--additional-derive",
        "ExtraDerive",
        "--output",
        output_file.to_str().unwrap(),
    ])
    .assert()
    .success();

    let actual = std::fs::read_to_string(output_file).unwrap();

    assert_contents("tests/outputs/derive.rs", &actual);
}

#[test]
fn test_multi_derive() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cargo-typify");
    cmd.args([
        "typify",
        input,
        "--no-builder",
        "--additional-derive",
        "ExtraDerive",
        "--additional-derive",
        "AnotherDerive",
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
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cargo-typify");

    let output = cmd.args(["typify", "--help"]).output().unwrap();

    let output_stdout = String::from_utf8(output.stdout).unwrap();
    let actual = dos2unix(&output_stdout);

    assert!(output.status.success());
    assert_contents("tests/outputs/help.txt", &actual);
}

#[test]
fn test_btree_map() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cargo-typify");
    cmd.args([
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
