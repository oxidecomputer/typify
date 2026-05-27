fn wrapper() {
    {
        let __snapshot_path = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/output/missing.rs");
        let __content: ::std::string::String = {
            let __output_tokens = ts.render();
            let __file: ::syn::File = ::syn::parse2(__output_tokens)
                .expect("failed to parse rendered output as Rust file");
            ::prettyplease::unparse(&__file)
        };
        if let Some(parent) = __snapshot_path.parent() {
            ::std::fs::create_dir_all(parent).ok();
        }
        ::std::fs::write(&__snapshot_path, &__content)
            .expect("failed to write snapshot");
        panic!("snapshot file created, run tests again: {}", __snapshot_path.display());
    }
}
