#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate sha1;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate derive_builder;

pub mod errors;

pub mod api;