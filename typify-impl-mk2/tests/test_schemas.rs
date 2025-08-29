use std::{collections::HashSet, path::PathBuf};

use anyhow::bail;
use serde::Deserialize;
use syn::parse_quote;
use typify_impl_mk2::{
    bundler::{Bundle, FileMapLoader},
    typespace::TypespaceSettings,
    Typify,
};
use url::Url;

#[test]
fn test_schemas() -> anyhow::Result<()> {
    let mut errors = false;
    for entry in std::fs::read_dir("tests/schemas/input")? {
        let entry = entry?; // Handle potential I/O errors per entry
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();
        println!("{file_name}");

        let file_type = entry.file_type()?;
        let result = if file_type.is_file() {
            // Right now we expect only JSON; that might change in the future
            // to permit YML.
            assert!(file_name.ends_with(".json"), "{}", path.display());
            test_schemas_json(&path)
        } else if file_type.is_dir() {
            test_schemas_directory(&path)
        } else {
            panic!(
                "unexpected directory entry {}",
                path.canonicalize()?.display(),
            );
        };

        if let Err(e) = result {
            errors = true;
            println!("{file_name} .. ❌ {e}");
        } else {
            println!("{file_name} .. ✅");
        }
    }

    if errors {
        panic!("encountered errors");
    }

    let xxx = trybuild::TestCases::new();
    xxx.pass("tests/schemas/rust/*.rs");

    Ok(())
}

fn test_schemas_json(path: &PathBuf) -> anyhow::Result<()> {
    let mut bundle = Bundle::default();
    let root_content = std::fs::read_to_string(path)?;
    let context = bundle.add_content(root_content).expect("invalid content");
    let mut typify = Typify::new_with_bundle(bundle);
    let _type_id = typify
        .add_type_by_id(&context.location.to_string())
        .unwrap();

    validate_output(path, TypespaceSettings::default(), typify);

    Ok(())
}

#[derive(Deserialize)]
struct TestJson {
    #[serde(default)]
    settings: TypespaceSettings,
    #[serde(rename = "root-schema")]
    root_schema: TestJsonEntry,
    files: Vec<TestJsonEntry>,
}

#[derive(Deserialize)]
struct TestJsonEntry {
    #[serde(rename = "$id")]
    id: Url,
    path: String,
}

#[derive(Deserialize)]
struct JsonId {
    #[serde(rename = "$id")]
    id: Url,
}

fn test_schemas_directory(path: &PathBuf) -> anyhow::Result<()> {
    let test_json_path = path.join("test.json");

    let test_json_content = std::fs::read_to_string(test_json_path)?;

    let test_json = serde_json::from_str::<TestJson>(&test_json_content)?;

    let mut known_files = test_json
        .files
        .iter()
        .map(|TestJsonEntry { path, .. }| path.as_str())
        .collect::<HashSet<_>>();
    known_files.insert("test.json");
    known_files.insert(test_json.root_schema.path.as_str());

    for entry in std::fs::read_dir(&path)? {
        let entry = entry?.path();
        let name = entry.file_name().unwrap().to_string_lossy();
        let file_name = name.as_ref();
        if !known_files.contains(file_name) {
            println!("{file_name}");
            panic!()
        }
    }

    let mut loader = FileMapLoader::default();
    for file in test_json.files {
        let xxx = path.join(&file.path);
        confirm_id(xxx, &file.id)?;

        println!("adding {} {}", file.id, file.path);
        loader = loader.add(file.id, path.join(file.path));
    }

    let mut bundle = Bundle::new(loader);
    let root_content = std::fs::read_to_string(path.join(&test_json.root_schema.path))?;

    let context = bundle.add_content(root_content).expect("invalid content");

    // Validate that the $id matches; this is more for sanity and consistency
    // than for anything critical.
    let Some(actual_id) = bundle
        .resolve_root(&context.location)
        .unwrap()
        .value
        .get("$id")
    else {
        bail!(
            "{} didn't contain an '$id' field",
            test_json.root_schema.path,
        );
    };
    let Some(actual_id) = actual_id.as_str() else {
        bail!(
            "value of $id in {} is not a string",
            test_json.root_schema.path,
        );
    };
    if actual_id != test_json.root_schema.id.as_str() {
        bail!(
            "value of $id in {} was not correct (was {}; expected {})",
            test_json.root_schema.path,
            &test_json.root_schema.id,
            actual_id
        )
    }

    let mut typify = Typify::new_with_bundle(bundle);
    let _type_id = typify
        .add_type_by_id(&context.location.to_string())
        .unwrap();

    validate_output(path, test_json.settings, typify);

    Ok(())
}

fn validate_output(path: &PathBuf, settings: TypespaceSettings, typify: Typify) {
    let canonical_out = typify.canonical_output();

    let typespace = typify.into_typespace(settings);

    let tokens = typespace.render();

    let doc_str = format!(" Code generated from {}", path.display());

    let file = parse_quote! {
        #![doc = #doc_str]

        #tokens

        fn main() {}
    };
    let out = prettyplease::unparse(&file);

    let output_root = path.file_name().unwrap();
    let base = path.parent().unwrap().parent().unwrap();

    let canonical_path = base
        .to_path_buf()
        .join("canonical")
        .join(output_root)
        .with_extension("json");

    let rust_path = base
        .to_path_buf()
        .join("rust")
        .join(output_root)
        .with_extension("rs");

    println!(
        "path = {} {} {}",
        path.display(),
        canonical_path.display(),
        rust_path.display(),
    );

    expectorate::assert_contents(canonical_path, &canonical_out);
    expectorate::assert_contents(rust_path, &out);
}

fn confirm_id(path: PathBuf, id: &Url) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(&path)?;
    let json_id = serde_json::from_str::<JsonId>(&content)?;
    assert_eq!(&json_id.id, id, "id mismatch for {}", path.display());
    Ok(())
}
