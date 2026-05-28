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
}
