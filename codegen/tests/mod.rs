#![allow(unused_attributes)]

#![feature(quote, rustc_private)]

extern crate syntax;
extern crate serde_json;
extern crate elastic_codegen;

pub mod api_parse;
pub mod api_gen;
pub mod rust_gen;
pub mod rust_emit;