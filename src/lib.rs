#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate sha1;

#[macro_use]
extern crate derive_builder;

mod model;

pub mod errors;
pub mod api;