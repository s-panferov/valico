
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

pub fn assert_path_str(obj: &Json, path: &[&str], res: &str) {
	let path: Vec<String> = path.iter().map(|s| s.to_string()).collect();
	let mut ver_ref = vec![];
	for p in path.iter() {
		ver_ref.push(p);
	}
	assert_eq!(obj.find_path(ver_ref.as_slice()).unwrap().as_string().unwrap().as_slice(), res);
}

pub fn assert_result_path_str(params: &Builder, body: &str, path: &[&str], res: &str) {
	assert_path_str(&test_result(params, body), path, res);
}

pub fn assert_error_path_str(params: &Builder, body: &str, path: &[&str], res: &str) {
	assert_path_str(&test_error(params, body), path, res);
}