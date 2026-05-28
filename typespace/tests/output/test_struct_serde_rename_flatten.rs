#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct Inner {
    pub value: u32,
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct Outer {
    #[serde(rename = "my-field")]
    pub my_field: String,
    #[serde(flatten)]
    pub inner: Inner,
}
