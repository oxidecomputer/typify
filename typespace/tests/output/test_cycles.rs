#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct A {
    #[serde(
        default,
        deserialize_with = "::json_serde::deserialize_some",
        skip_serializing_if = ":: std :: option :: Option::is_none"
    )]
    pub a: ::std::option::Option<::std::boxed::Box<A>>,
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct B {
    #[serde(
        default,
        deserialize_with = "::json_serde::deserialize_some",
        skip_serializing_if = ":: std :: option :: Option::is_none"
    )]
    pub c: ::std::option::Option<C>,
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct C {
    #[serde(
        default,
        deserialize_with = "::json_serde::deserialize_some",
        skip_serializing_if = ":: std :: option :: Option::is_none"
    )]
    pub b: ::std::option::Option<::std::boxed::Box<B>>,
}
