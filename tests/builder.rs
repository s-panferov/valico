
use valico::{
	Builder,
	Coercer,
	StringCoercer,
	I64Coercer,
	U64Coercer,
	F64Coercer,
	BooleanCoercer,
	NullCoercer,
	ListCoercer,
	ObjectCoercer,
	MutableJson
};

use helpers::{assert_str_eq, test_error};

#[test]
fn is_process_simple_require() {

	let params = Builder::from_function(|params: &mut Builder| {
		params.req("name");
	});

	assert_str_eq(&params, r#"{"name":1}"#, r#"{"name":1}"#);
	assert_eq!(test_error(&params, r#"{}"#).find_path([&"name".to_string(), &"type".to_string()]).unwrap().as_string().unwrap(), "validation");
}