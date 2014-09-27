
use serialize::json;
use serialize::json::{Json, ToJson};

use valico::{
	Builder,
	MutableJson
};

pub fn test_result(params: &Builder, body: &str) -> Json {
	let mut obj = json::from_str(body);
	match obj {
		Ok(mut json) => { 
			match params.process(json.as_object_mut().unwrap()) {
				Ok(()) => {
					return json;
				},
				Err(err) => {
					fail!("Error during process: {}", err.to_json().to_pretty_str());
				}
			}
		},
		Err(_) => {
			fail!("Invalid JSON");
		}
	}
}

pub fn test_error(params: &Builder, body: &str) -> Json {
	let mut obj = json::from_str(body);
	match obj {
		Ok(mut json) => { 
			match params.process(json.as_object_mut().unwrap()) {
				Ok(()) => {
					fail!("Success responce when we await some errors");
				},
				Err(err) => {
					return err.to_json();
				}
			}
		},
		Err(_) => {
			fail!("Invalid JSON");
		}
	}
}

pub fn assert_str_eq(params: &Builder, body: &str, res: &str) {
	assert_eq!(test_result(params, body).to_string(), res.to_string());
}

pub fn assert_path_str(params: &Builder, body: &str, res: &str) {
	assert_eq!(test_result(params, body).to_string(), res.to_string());
}