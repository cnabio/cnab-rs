#![cfg_attr(test, deny(warnings))]
#![warn(rust_2018_idioms)]

mod cnab;
pub use crate::cnab::*;
mod claim;
pub use crate::claim::*;

// Re-export Ulid for convenience
pub use ulid::Ulid;

#[cfg(test)]
mod tests;
