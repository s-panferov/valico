
use valico::{
	Builder,
	Coercer
};

use helpers::{
	test_error,
	assert_str_eq, 
	assert_result_path_str, 
	assert_error_path_str};

#[test]
fn is_process_simple_require() {

	let params = Builder::build(|params| {
		params.req("a");
	});

	assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":1}"#);
	assert_error_path_str(&params, r#"{}"#, ["a", "type"], "validation");
}

// #[test]
// fn is_process_i64_require() {

// 	let params = Builder::build(|params| {
// 		params.req_type("a", Builder::i64());
// 	});

// 	assert_str_eq(&params, r#"{"a":"1"}"#, r#"{"a":1}"#);
	
// 	// truncate!
// 	assert_str_eq(&params, r#"{"a": 1.112}"#, r#"{"a":1}"#);

// 	// error because "a" is string that we can't convert
// 	assert_error_path_str(&params, r#"{"a": "not-int"}"#, ["a", "type"], "coercion");
// 	// error because "a" is object
// 	assert_error_path_str(&params, r#"{"a": {"a": 1}}"#, ["a", "type"], "coercion");
// }

// #[test]
// fn is_process_string_require() {

// 	let params = Builder::build(|params| {
// 		params.req_type("a", Builder::string());
// 	});

// 	assert_str_eq(&params, r#"{"a":"1"}"#, r#"{"a":"1"}"#);
// 	assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":"1"}"#);
// 	assert_str_eq(&params, r#"{"a":1.112}"#, r#"{"a":"1.112"}"#);
	
// 	// error because "a" is object
// 	assert_error_path_str(&params, r#"{"a": {}}"#, ["a", "type"], "coercion");
	
// 	// error because "a" is null
// 	assert_error_path_str(&params, r#"{"a": null}"#, ["a", "type"], "coercion");
// }

// #[test]
// fn is_process_simple_list_require() {

// 	let params = Builder::build(|params| {
// 		params.req_type("a", Builder::list());
// 	});

// 	assert_str_eq(&params, r#"{"a":[1,"2",[3]]}"#, r#"{"a":[1,"2",[3]]}"#);

// 	// error because "a" is object
// 	assert_error_path_str(&params, r#"{"a": {}}"#, ["a", "type"], "coercion");

// 	// error because "a" is string
// 	assert_error_path_str(&params, r#"{"a": "test"}"#, ["a", "type"], "coercion");
// }

// #[test]
// fn is_process_typed_list_require() {

// 	let params = Builder::build(|params| {
// 		params.req_type("a", Builder::list_of(Builder::string()));
// 	});

// 	// convert all to string
// 	assert_str_eq(&params, r#"{"a":[1,"2",3.1]}"#, r#"{"a":["1","2","3.1"]}"#);

// 	// error because "a" is object
// 	assert_error_path_str(&params, r#"{"a": {}}"#, ["a", "type"], "coercion");

// 	// error because element at index(2) is not coersible to string
// 	assert_error_path_str(&params, r#"{"a": [1,2,{}]}"#, ["a", "2", "type"], "coercion");

// }

// #[test]
// fn is_process_list_with_nested_require() {

// 	let params = Builder::build(|params| {
// 		params.req_nest("a", Builder::list(), |params| {
// 			params.req_type("b", Builder::string());
// 			params.req_type("c", Builder::list_of(Builder::u64()))
// 		});
// 	});

// 	assert_str_eq(&params, r#"{"a":[{"b":1,"c":["1"]}]}"#, r#"{"a":[{"b":"1","c":[1]}]}"#);

// 	// error because element in "a" at index(0) is not coersible to string
// 	assert_error_path_str(&params, r#"{"a":[{"b":{},"c":["1"]}]}"#, ["a", "0", "b", "type"], "coercion");

// 	// error because element in "a":0:"c":0 is not coersible to string
// 	assert_error_path_str(&params, r#"{"a":[{"b":1,"c":[{}]}]}"#, ["a", "0", "c", "0", "type"], "coercion");

// }

// #[test]
// fn is_process_object_require() {

// 	let params = Builder::build(|params| {
// 		params.req_type("a", Builder::object());
// 	});

// 	assert_str_eq(&params, r#"{"a":{}}"#, r#"{"a":{}}"#);

// 	// error because "a" is array, not object
// 	assert_error_path_str(&params, r#"{"a":[]}"#, ["a", "type"], "coercion");

// 	// error because "a" is string, not object
// 	assert_error_path_str(&params, r#"{"a":""}"#, ["a", "type"], "coercion");

// }

// #[test]
// fn is_process_object_with_nested_require() {

// 	let params = Builder::build(|params| {
// 		params.req_nest("a", Builder::object(), |params| {
// 			params.req_type("b", Builder::f64());
// 			params.req_type("c", Builder::list_of(Builder::string()));
// 		});
// 	});

// 	assert_str_eq(&params, r#"{"a":{"b":"1.22","c":[1.112,""]}}"#, r#"{"a":{"b":1.22,"c":["1.112",""]}}"#);

// 	// error because "a":"b" is not a f64
// 	assert_error_path_str(&params, r#"{"a":{"b":"not-f64"},"c":[1.112,""]}"#, ["a", "b", "type"], "coercion");

// 	// error because "a":"c":"1" is object and can't be coerced to string
// 	assert_error_path_str(&params, r#"{"a":{"b":"1.22","c":[1.112,{}]}}"#, ["a", "c", "1", "type"], "coercion");

// }