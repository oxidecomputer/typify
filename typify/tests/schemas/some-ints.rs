use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize)]
pub struct Sub10Primes(u32);
impl std::ops::Deref for Sub10Primes {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
impl std::convert::TryFrom<&u32> for Sub10Primes {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, &'static str> {
        if ![2_u32, 3_u32, 5_u32, 7_u32].contains(&value) {
            Err("invalid value")
        } else {
            Ok(Self(value))
        }
    }
}
fn main() {}
