typify_renamed::import_types!("schema.json");

#[cfg(test)]
mod tests {
    use super::MacroDependencies;

    #[test]
    fn generates_root_type() {
        let value: Option<MacroDependencies> = None;
        assert!(value.is_none());
    }
}
