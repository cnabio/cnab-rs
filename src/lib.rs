#![cfg_attr(test, deny(warnings))]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

pub mod cnab;

#[cfg(test)]
mod tests;
