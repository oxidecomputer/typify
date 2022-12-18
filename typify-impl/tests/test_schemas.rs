use quote::quote;
use schemars::schema::RootSchema;

use typify_impl::TypeSpace;

#[test]
fn test_enum_default_generation() {
    let schema_json = r##"
        {
            "definitions": {
                "test-enum": {
                    "type": "string",
                    "enum": ["failure", "skipped", "success"],
                    "default": "failure"
                }
            }
        }
        "##;

    let schema: RootSchema = serde_json::from_str(schema_json).unwrap();

    let mut type_space = TypeSpace::default();
    type_space.add_ref_types(schema.definitions).unwrap();

    let actual = type_space.to_stream();

    let expected = quote! {
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum TestEnum {
        #[serde(rename = "failure")]
        Failure,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "success")]
        Success,
    }
    impl ToString for TestEnum {
        fn to_string(&self) -> String {
            match *self {
                Self::Failure => "failure".to_string(),
                Self::Skipped => "skipped".to_string(),
                Self::Success => "success".to_string(),
            }
        }
    }
    impl std::str::FromStr for TestEnum {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            match value {
                "failure" => Ok(Self::Failure),
                "skipped" => Ok(Self::Skipped),
                "success" => Ok(Self::Success),
                _ => Err("invalid value"),
            }
        }
    }
    impl Default for TestEnum {
        fn default() -> Self {
            TestEnum::Failure
        }
    }};

    assert_eq!(expected.to_string(), actual.to_string());
}
