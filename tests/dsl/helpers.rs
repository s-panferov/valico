
use serialize::json;

use valico::json_dsl;
use valico::json_schema;
use valico::common::error;

pub fn test_result(params: &json_dsl::Builder, scope: Option<&json_schema::Scope>, body: &str) -> json::Json {
    let obj = body.parse::<json::Json>();
    match obj {
        Ok(mut json) => { 
            let state = params.process(&mut json, &scope);
            if state.is_strictly_valid() {
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

pub fn get_errors(params: &json_dsl::Builder, scope: Option<&json_schema::Scope>, body: &str) -> Vec<Box<error::ValicoError>> {
    let obj = body.parse::<json::Json>();
    match obj {
        Ok(mut json) => { 
            let state = params.process(&mut json, &scope);
            if state.is_strictly_valid() {
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

pub fn assert_str_eq_with_scope(params: &json_dsl::Builder, scope: Option<&json_schema::Scope>, body: &str, res: &str) {
    assert_eq!(test_result(params, scope, body).to_string(), res.to_string());
}

pub fn assert_error_with_scope<T: error::ValicoError + Send>(params: &json_dsl::Builder, scope: Option<&json_schema::Scope>, body: &str, path: &str) {
    let errors = get_errors(params, scope, body);
    let error = errors.iter().find(|&: error| {
        let err = error.downcast_ref::<T>();
        err.is_some() && err.unwrap().get_path() == path
    });

    assert!(error.is_some(), "Can't find error in {}. Errors: {:?}", path, errors)
}

pub fn assert_str_eq(params: &json_dsl::Builder, body: &str, res: &str) {
    assert_str_eq_with_scope(params, None, body, res);
}

pub fn assert_error<T: error::ValicoError + Send>(params: &json_dsl::Builder, body: &str, path: &str) {
    assert_error_with_scope::<T>(params, None, body, path);
}