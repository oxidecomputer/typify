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
    #[serde(default = "xxx")]
    pub peanut_string: String,
    #[serde(default = "xxx")]
    pub peanut_option: Option<String>,
}
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
    #[serde(default = "xxx")]
    pub peanut_string: String,
    #[serde(default = "xxx")]
    pub peanut_option: Option<String>,
}
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
    #[serde(default = "xxx")]
    pub peanut_string: String,
    #[serde(default = "xxx")]
    pub peanut_option: Option<String>,
}
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
    #[serde(default, skip_serializing_if = "OptionField::is_absent")]
    pub optional_option: OptionField<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub default_string: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_option: Option<String>,
    #[serde(default = "xxx")]
    pub peanut_string: String,
    #[serde(default = "xxx")]
    pub peanut_option: Option<String>,
}
