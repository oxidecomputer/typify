fn wrapper() {
    {
        let _ = include_str!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/", "tests/output/my_type.rs")
        );
        let __snapshot_path = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/output/my_type.rs");
        let __content: ::std::string::String = {
            let __output_tokens = ts.render();
            let __file: ::syn::File = ::syn::parse2(__output_tokens)
                .expect("failed to parse rendered output as Rust file");
            ::prettyplease::unparse(&__file)
        };
        let __needs_update = match ::std::fs::read_to_string(&__snapshot_path) {
            Ok(ref existing) => existing != &__content,
            Err(_) => true,
        };
        if __needs_update {
            if let Some(parent) = __snapshot_path.parent() {
                ::std::fs::create_dir_all(parent).ok();
            }
            ::std::fs::write(&__snapshot_path, __content)
                .expect("failed to write snapshot");
            panic!("snapshot updated, run tests again: {}", __snapshot_path.display());
        }
        mod import {
            use super::*;
            pub struct MyType(pub String);
        }
        let value = import::MyType("hello".to_string());
        assert_eq!(value.0, "hello");
    }
}
