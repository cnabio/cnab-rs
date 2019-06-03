#![cfg(test)]
#![warn(rust_2018_idioms)]

mod cnab;
pub use crate::cnab::*;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate canonical_json;
extern crate spectral;

#[cfg(test)]
mod tests;
