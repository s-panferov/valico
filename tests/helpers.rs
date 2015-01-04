
use serialize::json;
use serialize::json::{Json, ToJson, as_pretty_json};

use valico::{
    Builder,
    MutableJson
};

pub fn test_result(params: &Builder, body: &str) -> Json {
    let obj = body.parse::<Json>();
    match obj {
        Some(mut json) => { 
            match params.process(json.as_object_mut().unwrap()) {
                Ok(()) => {
                    return json;
                },
                Err(err) => {
                    panic!("Error during process: {}", as_pretty_json(&err.to_json()).to_string());
                }
            }
        },
        None => {
            panic!("Invalid JSON");
        }
    }
}

pub fn test_error(params: &Builder, body: &str) -> Json {
    let obj = body.parse::<Json>();
    match obj {
        Some(mut json) => { 
            match params.process(json.as_object_mut().unwrap()) {
                Ok(()) => {
                    panic!("Success responce when we await some errors");
                },
                Err(err) => {
                    return err.to_json();
                }
            }
        },
        None => {
            panic!("Invalid JSON");
        }
    }
}

pub fn assert_str_eq(params: &Builder, body: &str, res: &str) {
    assert_eq!(test_result(params, body).to_string(), res.to_string());
}

pub fn assert_path(obj: &Json, path: &[&str]) {
    assert!(obj.find_path(path).is_some());
}

pub fn assert_result_key(params: &Builder, body: &str, path: &[&str]) {
    assert_path(&test_result(params, body), path);
}

pub fn assert_error_key(params: &Builder, body: &str, path: &[&str]) {
    assert_path(&test_error(params, body), path);
}