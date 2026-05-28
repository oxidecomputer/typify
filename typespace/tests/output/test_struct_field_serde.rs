pub use conflated_as_absent::*;
pub use conflated_as_null::*;
pub use double_option::*;
pub use custom_type::*;
pub mod conflated_as_absent {
    #[derive(::serde::Deserialize, ::serde::Serialize)]
    pub struct ConflatedAsAbsent {
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub optional_string: Option<String>,
        #[serde(deserialize_with = "Option::deserialize")]
        pub required_option: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub optional_option: Option<String>,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub default_string: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_option: Option<String>,
        #[serde(default = "defaults::conflated_as_absent__peanut_string")]
        pub peanut_string: String,
        #[serde(default = "defaults::conflated_as_absent__peanut_option")]
        pub peanut_option: Option<String>,
    }
    pub mod defaults {
        pub fn conflated_as_absent__peanut_option() -> Option<String> {
            ::serde_json::from_value(::serde_json::Value::String("peanuts".to_string()))
                .expect("invalid default value")
        }
        pub fn conflated_as_absent__peanut_string() -> String {
            ::serde_json::from_value(::serde_json::Value::String("peanuts".to_string()))
                .expect("invalid default value")
        }
    }
}
pub mod conflated_as_null {
    #[derive(::serde::Deserialize, ::serde::Serialize)]
    pub struct ConflatedAsNull {
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub optional_string: Option<String>,
        #[serde(deserialize_with = "Option::deserialize")]
        pub required_option: Option<String>,
        pub optional_option: Option<String>,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub default_string: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_option: Option<String>,
        #[serde(default = "defaults::conflated_as_null__peanut_string")]
        pub peanut_string: String,
        #[serde(default = "defaults::conflated_as_null__peanut_option")]
        pub peanut_option: Option<String>,
    }
    pub mod defaults {
        pub fn conflated_as_null__peanut_option() -> Option<String> {
            ::serde_json::from_value(::serde_json::Value::String("peanuts".to_string()))
                .expect("invalid default value")
        }
        pub fn conflated_as_null__peanut_string() -> String {
            ::serde_json::from_value(::serde_json::Value::String("peanuts".to_string()))
                .expect("invalid default value")
        }
    }
}
pub mod custom_type {
    #[derive(::serde::Deserialize, ::serde::Serialize)]
    pub struct CustomType {
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub optional_string: Option<String>,
        #[serde(deserialize_with = "Option::deserialize")]
        pub required_option: Option<String>,
        #[serde(default, skip_serializing_if = "super::OptionField::is_absent")]
        pub optional_option: super::OptionField<String>,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub default_string: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_option: Option<String>,
        #[serde(default = "defaults::custom_type__peanut_string")]
        pub peanut_string: String,
        #[serde(default = "defaults::custom_type__peanut_option")]
        pub peanut_option: Option<String>,
    }
    pub mod defaults {
        pub fn custom_type__peanut_option() -> Option<String> {
            ::serde_json::from_value(::serde_json::Value::String("peanuts".to_string()))
                .expect("invalid default value")
        }
        pub fn custom_type__peanut_string() -> String {
            ::serde_json::from_value(::serde_json::Value::String("peanuts".to_string()))
                .expect("invalid default value")
        }
    }
}
pub mod double_option {
    #[derive(::serde::Deserialize, ::serde::Serialize)]
    pub struct DoubleOption {
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub optional_string: Option<String>,
        #[serde(deserialize_with = "Option::deserialize")]
        pub required_option: Option<String>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pub optional_option: Option<Option<String>>,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub default_string: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_option: Option<String>,
        #[serde(default = "defaults::double_option__peanut_string")]
        pub peanut_string: String,
        #[serde(default = "defaults::double_option__peanut_option")]
        pub peanut_option: Option<String>,
    }
    pub mod defaults {
        pub fn double_option__peanut_option() -> Option<String> {
            ::serde_json::from_value(::serde_json::Value::String("peanuts".to_string()))
                .expect("invalid default value")
        }
        pub fn double_option__peanut_string() -> String {
            ::serde_json::from_value(::serde_json::Value::String("peanuts".to_string()))
                .expect("invalid default value")
        }
    }
}
