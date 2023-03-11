use std::path::Path;

use assert_fs::prelude::*;
use expectorate::assert_contents;
use predicates::prelude::predicate;
use tempdir::TempDir;

#[test]
fn test_simple() {
    use assert_cmd::Command;

    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = Command::cargo_bin("cargo-typify").unwrap();
    cmd.args([input, "--output", output_file.to_str().unwrap()])
        .assert()
        .success();

    let content = std::fs::read_to_string(output_file).unwrap();

    assert_contents(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/outputs/simple.rs"),
        &content,
    );
}

#[test]
fn test_simple_stdout() {
    use assert_cmd::Command;

    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let mut cmd = Command::cargo_bin("cargo-typify").unwrap();

    let expected = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/outputs/simple.rs"
    ))
    .unwrap();

    cmd.args([input])
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_builder() {
    use assert_cmd::Command;

    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = Command::cargo_bin("cargo-typify").unwrap();
    cmd.args([
        input,
        "--builder",
        "--output",
        output_file.to_str().unwrap(),
    ])
    .assert()
    .success();

    let content = std::fs::read_to_string(output_file).unwrap();

    assert_contents(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/outputs/builder.rs"),
        &content,
    );
}

#[test]
fn test_derive() {
    use assert_cmd::Command;

    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = Command::cargo_bin("cargo-typify").unwrap();
    cmd.args([
        input,
        "--additional-derives",
        "ExtraDerive",
        "--output",
        output_file.to_str().unwrap(),
    ])
    .assert()
    .success();

    let content = std::fs::read_to_string(output_file).unwrap();

    assert_contents(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/outputs/derive.rs"),
        &content,
    );
}

#[test]
fn test_multi_derive() {
    use assert_cmd::Command;

    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = Command::cargo_bin("cargo-typify").unwrap();
    cmd.args([
        input,
        "--additional-derives",
        "ExtraDerive",
        "--additional-derives",
        "AnotherDerive",
        "--output",
        output_file.to_str().unwrap(),
    ])
    .assert()
    .success();

    let content = std::fs::read_to_string(output_file).unwrap();

    assert_contents(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/outputs/multi_derive.rs"),
        &content,
    );
}

#[test]
fn test_type_mod() {
    // TODO: This test passes with a copy of the simple output, so its not actually changing anything
    // If this isn't expected to change anything we should remove the functionality
    use assert_cmd::Command;

    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");

    let temp = TempDir::new("cargo-typify").unwrap();
    let output_file = temp.path().join("output.rs");

    let mut cmd = Command::cargo_bin("cargo-typify").unwrap();
    cmd.args([
        input,
        "--output",
        output_file.to_str().unwrap(),
        "--type-mod",
        "cool_types",
    ])
    .assert()
    .success();

    let content = std::fs::read_to_string(output_file).unwrap();

    assert_contents(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/outputs/type_mod.rs"),
        &content,
    );
}
