#![cfg_attr(test, deny(warnings))]
#![warn(rust_2018_idioms)]

mod cnab;
pub use crate::cnab::*;

#[cfg(test)]
mod tests;
