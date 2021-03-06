extern crate elastic_codegen;
extern crate serde_json;

use std::fs::File;
use elastic_codegen::api::ast::*;
use elastic_codegen::api::parse;

#[test]
fn can_parse_http_method() {
	let raw_methods = vec![
		"HEAD",
		"GET",
		"POST",
		"PUT",
		"DELETE"
	];

	let expected_methods = vec![
		HttpVerb::Head,
		HttpVerb::Get,
		HttpVerb::Post,
		HttpVerb::Put,
		HttpVerb::Delete
	];

	let methods: Vec<HttpVerb> = raw_methods.iter().map(|m| HttpVerb::parse(m)).collect();

	let mut success = true;
	for i in 0..methods.len() {
		if expected_methods[i] != methods[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn can_parse_type() {
	struct TypeRef {
		name: &'static str,
		opts: Option<Vec<String>>
	}

	let raw_types: Vec<TypeRef> = vec![
		TypeRef { name: "string", opts: None },
		TypeRef { name: "boolean", opts: None },
		TypeRef { name: "time", opts: None },
		TypeRef { name: "list", opts: None },
		TypeRef { name: "number", opts: None },
		TypeRef { name: "long", opts: None },
		TypeRef { name: "integer", opts: None },
		TypeRef { name: "short", opts: None },
		TypeRef { name: "byte", opts: None },
		TypeRef { name: "float", opts: None },
		TypeRef { name: "double", opts: None },
		TypeRef { name: "enum", opts: Some(vec!("OpA".to_string(), "OpB".to_string(), "OpC".to_string())) },
		TypeRef { name: "stuff", opts: None }
	];

	let opts = Some(vec!("OpA".to_string(), "OpB".to_string(), "OpC".to_string()));
	let expected_types = vec![
		Type::Str,
		Type::Bool,
		Type::Time,
		Type::List,
		Type::Number(NumberKind::Long),
		Type::Number(NumberKind::Long),
		Type::Number(NumberKind::Int),
		Type::Number(NumberKind::Short),
		Type::Number(NumberKind::Byte),
		Type::Number(NumberKind::Float),
		Type::Number(NumberKind::Double),
		Type::Enum(&opts),
		Type::Other("stuff")
	];

	let types: Vec<Type> = raw_types.iter().map(|t| Type::parse(t.name, &t.opts)).collect();

	let mut success = true;
	for i in 0..types.len() {
		if expected_types[i] != types[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn can_parse_param_default() {
	//A string value
	let str_param = Param::string(false, Some("op1".to_string()));

	assert_eq!("op1".to_string(), str_param.get_default::<String>().unwrap());

	//A bool value
	let bool_param = Param::bool(false, Some(false));

	assert_eq!(false, bool_param.get_default::<bool>().unwrap());
}

#[test]
fn can_parse_from_file() {
	let mut f = File::open("spec/api/bulk.json").unwrap();
	let parsed = parse::from_reader(&mut f).unwrap();

	assert!(parsed.name.unwrap() == "bulk".to_string());
}

#[test]
fn can_parse_all_in_dir() {
	let parsed = parse::from_dir("spec/api");

	assert!(parsed.is_ok());
}