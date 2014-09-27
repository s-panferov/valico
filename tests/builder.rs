
use valico::{
	Builder,
	Coercer
};

use helpers::{assert_str_eq, assert_result_path_str, assert_error_path_str};

#[test]
fn is_process_simple_require() {

	let params = Builder::from_function(|params: &mut Builder| {
		params.req("a");
	});

	assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":1}"#);
	assert_error_path_str(&params, r#"{}"#, ["a", "type"], "validation");
}

#[test]
fn is_process_i64_require() {

	let params = Builder::from_function(|params: &mut Builder| {
		params.req_type("a", Builder::i64());
	});

	assert_str_eq(&params, r#"{"a":"1"}"#, r#"{"a":1}"#);
	
	// truncate!
	assert_str_eq(&params, r#"{"a": 1.112}"#, r#"{"a":1}"#);

	// error because a is string that we can't convert
	assert_error_path_str(&params, r#"{"a": "not-int"}"#, ["a", "type"], "coercion");
	// error because a is object
	assert_error_path_str(&params, r#"{"a": {"a": 1}}"#, ["a", "type"], "coercion");
}

#[test]
fn is_process_string_require() {

	let params = Builder::from_function(|params: &mut Builder| {
		params.req_type("a", Builder::string());
	});

	assert_str_eq(&params, r#"{"a":"1"}"#, r#"{"a":"1"}"#);
	assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":"1"}"#);
	assert_str_eq(&params, r#"{"a":1.112}"#, r#"{"a":"1.112"}"#);
	
	// error because a is object
	assert_error_path_str(&params, r#"{"a": {}}"#, ["a", "type"], "coercion");
	
	// error because a is null
	assert_error_path_str(&params, r#"{"a": null}"#, ["a", "type"], "coercion");
}