#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(json_str)]

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::number::prelude::*;
use ::number_fixtures::*;

#[test]
fn serialise_elastic_numbers() {
	let ser = vec![
		{
			let num = ElasticInteger::<MyIntegerMapping>::new(1i32);
			serde_json::to_string(&num).unwrap()
		},
		{
			let num = ElasticLong::<MyLongMapping>::new(1i64);
			serde_json::to_string(&num).unwrap()
		},
		{
			let num = ElasticShort::<MyShortMapping>::new(1i16);
			serde_json::to_string(&num).unwrap()
		},
		{
			let num = ElasticByte::<MyByteMapping>::new(1i8);
			serde_json::to_string(&num).unwrap()
		},
		{
			let num = ElasticFloat::<MyFloatMapping>::new(1.01f32);
			serde_json::to_string(&num).unwrap()
		},
		{
			let num = ElasticDouble::<MyDoubleMapping>::new(1.01f64);
			serde_json::to_string(&num).unwrap()
		}
	];

	let expected_ser = vec![
		"1",
		"1",
		"1",
		"1",
		"1.01",
		"1.01"
	];

	let mut success = true;
	for i in 0..ser.len() {
		if expected_ser[i] != &ser[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn deserialise_elastic_numbers() {
	let int_de: ElasticInteger<MyIntegerMapping> = serde_json::from_str("1").unwrap();
	let long_de: ElasticLong<MyLongMapping> = serde_json::from_str("1").unwrap();
	let short_de: ElasticShort<MyShortMapping> = serde_json::from_str("1").unwrap();
	let byte_de: ElasticByte<MyByteMapping> = serde_json::from_str("1").unwrap();
	let float_de: ElasticFloat<MyFloatMapping> = serde_json::from_str("1.01").unwrap();
	let double_de: ElasticDouble<MyDoubleMapping> = serde_json::from_str("1.01").unwrap();

	assert_eq!(
		(1i32, 1i64, 1i16, 1i8, 1.01f32, 1.01f64),
		(int_de.get(), long_de.get(), short_de.get(), byte_de.get(), float_de.get(), double_de.get())
	);
}