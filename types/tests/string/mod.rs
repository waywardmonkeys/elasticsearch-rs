#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(json_str)]

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::string::mapping::*;
use elastic_types::string::prelude::*;

#[test]
fn serialise_elastic_string() {
	let string: ElasticString<DefaultStringMapping> = ElasticString::new("my string");

	let ser = serde_json::to_string(&string).unwrap();

	assert_eq!(r#""my string""#, ser);
}

#[test]
fn deserialise_elastic_string() {
	let string: ElasticString<DefaultStringMapping> = serde_json::from_str(r#""my string""#).unwrap();

	assert_eq!("my string", string);
}