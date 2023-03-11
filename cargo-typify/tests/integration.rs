use std::path::Path;

use assert_fs::prelude::*;
use expectorate::assert_contents;
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
