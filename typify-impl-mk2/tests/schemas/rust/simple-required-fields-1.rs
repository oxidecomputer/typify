#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct SchemaRoot {
    #[serde(
        default,
        deserialize_with = "::json_serde::deserialize_some",
        skip_serializing_if = ":: std :: option :: Option::is_none"
    )]
    pub foo: ::std::option::Option<::std::string::String>,
}
fn main() {}
