#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct TupleA(pub ::std::string::String, pub u32);
///use of 'rest' field
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct TupleB(pub ::std::string::String, pub u32, pub ::std::string::String);
