use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, educe :: Educe)]
#[educe(Default)]
#[serde(untagged)]
pub enum IpNet {
    #[educe(Default)]
    V4(Ipv4Net),
    V6(Ipv4Net),
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Ipv4Net(pub String);
impl std::ops::Deref for Ipv4Net {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Ipv6Net(pub String);
impl std::ops::Deref for Ipv6Net {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, educe :: Educe)]
#[educe(Default)]
#[serde(untagged)]
pub enum OneOfTypes {
    #[educe(Default)]
    Variant0 {
        bar: i64,
    },
    Variant1 {
        foo: String,
    },
}
fn main() {}
