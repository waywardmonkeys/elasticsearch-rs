#![cfg_attr(feature = "nightly", feature(plugin, custom_derive))]
#![cfg_attr(feature = "nightly", plugin(json_str))]

#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate json_str;

use json_str::parse::*;

#[test]
fn can_generate_json() {
	let j = json_str!({
		"a": 7,
		"b": { "c": "some stuff" },
		"data": [
			{ "id": 1, "name": "stuff" },
			{ "id": 2, "name": "stuff" }
		]
	});

	assert_eq!("{\"a\":7,\"b\":{\"c\":\"some stuff\"},\"data\":[{\"id\":1,\"name\":\"stuff\"},{\"id\":2,\"name\":\"stuff\"}]}", j);
}

#[test]
fn can_generate_quasi_json() {
	let j = json_str!({
		a: 7,
		b: { c: "some stuff" },
		data: [
			{ id: 1, name: "stuff" },
			{ id: 2, name: "stuff" }
		]
	});

	assert_eq!("{\"a\":7,\"b\":{\"c\":\"some stuff\"},\"data\":[{\"id\":1,\"name\":\"stuff\"},{\"id\":2,\"name\":\"stuff\"}]}", j);
}

#[test]
fn sanitisation_removes_whitespace() {
	let j = "\n{ \"a\" : \"stuff\", \"b\":{  \"c\":[ 0, \r\n1 ] }		,\"d\":14 }";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff\",\"b\":{\"c\":[0,1]},\"d\":14}", &sanitised);
}

#[test]
fn sanitisation_does_not_affect_strings() {
	let j = "\n{ \"a\" : \"stuff and data.\n 	More.\"}";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff and data.\n 	More.\"}", &sanitised);
}

#[test]
fn sanitisation_standardises_quotes() {
	let j = "{ 'a' : \"stuff\", \"b\":{  \"c\":[ '0', 1 ] },\"d\":14 }";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff\",\"b\":{\"c\":[\"0\",1]},\"d\":14}", &sanitised);
}

#[test]
fn sanitisation_quotes_unquoted_keys() {
	let j = "{ a : \"stuff\", \"b\":{  c:[ 0, 1 ] },d:14 }";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff\",\"b\":{\"c\":[0,1]},\"d\":14}", &sanitised);
}

#[test]
fn sanitisation_does_not_quote_special_values() {
	let j = "{ \"a\": \"stuff\", \"b\": true, \"c\": false, \"d\": null }";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff\",\"b\":true,\"c\":false,\"d\":null}", &sanitised);
}
