
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

use helpers::{assert_str_eq, assert_result_path_str, assert_error_path_str};

#[test]
fn is_process_simple_require() {

	let params = Builder::from_function(|params: &mut Builder| {
		params.req("name");
	});

	assert_str_eq(&params, r#"{"name":1}"#, r#"{"name":1}"#);
	assert_error_path_str(&params, r#"{}"#, vec!["name", "type"], "validation");
}