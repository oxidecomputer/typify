mod append_map;
pub mod bundler;
mod convert;
mod error;
mod namespace;
mod naming;
mod normalizer;
// mod normalizer2;
mod schema;
mod schemalet;
mod typify;

// Re-export the standalone typespace crate under the same path as the
// internal module it replaces.
pub use ::typespace;

pub use typify::TypeId;
pub use typify::Typespace;
pub use typify::Typify;
pub use typify::Typify2NameHint;
pub use typify::TypifySettings;

pub use error::Error;
pub use error::ErrorKind;
