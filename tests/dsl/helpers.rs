
use serialize::json;

use valico::json_dsl;
use valico::common::error;

pub fn test_result(params: &json_dsl::Builder, body: &str) -> json::Json {
    let obj = body.parse::<json::Json>();
    match obj {
        Ok(mut json) => { 
            let state = params.process(&mut json, &None);
            if state.is_valid() {
                return json;
            } else {
                panic!("Errors during process: {:?}", state);
            }
        },
        Err(_) => {
            panic!("Invalid JSON");
        }
    }
}

pub fn get_errors(params: &json_dsl::Builder, body: &str) -> Vec<Box<error::ValicoError>> {
    let obj = body.parse::<json::Json>();
    match obj {
        Ok(mut json) => { 
            let state = params.process(&mut json, &None);
            if state.is_valid() {
                panic!("Success responce when we await some errors");
            } else {
                return state.errors;
            }
        },
        Err(_) => {
            panic!("Invalid JSON");
        }
    }
}

pub fn assert_str_eq(params: &json_dsl::Builder, body: &str, res: &str) {
    assert_eq!(test_result(params, body).to_string(), res.to_string());
}

pub fn assert_path(obj: &json::Json, path: &[&str]) {
    assert!(obj.find_path(path).is_some());
}

#[allow(dead_code)]
pub fn assert_result_key(params: &json_dsl::Builder, body: &str, path: &[&str]) {
    assert_path(&test_result(params, body), path);
}

pub fn assert_error<T: error::ValicoError + Send>(params: &json_dsl::Builder, body: &str, path: &str) {
    let errors = get_errors(params, body);
    let error = errors.iter().find(|&: error| {
        let err = error.downcast_ref::<T>();
        err.is_some() && err.unwrap().get_path() == path
    });

    assert!(error.is_some(), "Can't find error in {}. Errors: {:?}", path, errors)
}