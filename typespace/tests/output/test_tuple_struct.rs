#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct MyTupleStruct(
    pub ::std::string::String,
    pub u32,
    pub ::std::vec::Vec<::std::string::String>,
);
