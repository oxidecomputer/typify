#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct All {
    pub a_bool: bool,
    pub an_int: u32,
    pub a_float: f64,
    pub a_string: String,
    pub a_json: ::serde_json::Value,
    pub a_vec: Vec<String>,
    pub a_map: ::std::collections::BTreeMap<String, u32>,
    pub a_set: Vec<String>,
    pub an_array: [u32; 3usize],
    pub a_tuple: (String, u32),
    pub a_box_string: Box<String>,
    pub a_box_vec: Box<Vec<String>>,
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct Defaults {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub a_bool: bool,
    #[serde(default)]
    pub an_int: u32,
    #[serde(default)]
    pub a_float: f64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub a_string: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub a_vec: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub a_map: ::std::collections::BTreeMap<String, u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub a_set: Vec<String>,
    #[serde(default)]
    pub an_array: [u32; 3usize],
    #[serde(default)]
    pub a_tuple: (String, u32),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub an_option: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub a_box_string: Box<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub a_box_vec: Box<Vec<String>>,
}
