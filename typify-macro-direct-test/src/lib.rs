typify_macro::import_types!("../typify-macro-test/schema.json");

#[cfg(test)]
mod tests {
    use super::MacroDependencies;

    #[test]
    fn retains_direct_dependency_paths() {
        let value: Option<MacroDependencies> = None;
        assert!(value.is_none());

        let _ = typify_disabled::TypeSpace::default();
    }
}
