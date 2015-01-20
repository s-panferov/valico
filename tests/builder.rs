
use serialize::json::{self, ToJson};

use valico;

use helpers::{
    assert_str_eq, 
    assert_error_key
};

#[test]
fn is_process_empty_builder() {

    let params = valico::Builder::build(|_params| { });

    assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":1}"#);
}

#[test]
fn is_process_simple_require() {

    let params = valico::Builder::build(|params| {
        params.req_defined("a");
    });

    assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":1}"#);
    assert_error_key(&params, r#"{}"#, &["a", "validation"]);
}

#[test]
fn is_process_i64_require() {

    let params = valico::Builder::build(|params| {
        params.req_typed("a", valico::i64());
    });

    assert_str_eq(&params, r#"{"a":"1"}"#, r#"{"a":1}"#);
    
    // truncate!
    assert_str_eq(&params, r#"{"a": 1.112}"#, r#"{"a":1}"#);

    // error because "a" is string that we can't convert
    assert_error_key(&params, r#"{"a": "not-int"}"#, &["a", "coercion"]);
    // error because "a" is object
    assert_error_key(&params, r#"{"a": {"a": 1}}"#, &["a", "coercion"]);
}

#[test]
fn is_process_string_require() {

    let params = valico::Builder::build(|params| {
        params.req_typed("a", valico::string());
    });

    assert_str_eq(&params, r#"{"a":"1"}"#, r#"{"a":"1"}"#);
    assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":"1"}"#);
    assert_str_eq(&params, r#"{"a":1.112}"#, r#"{"a":"1.112"}"#);
    
    // error because "a" is object
    assert_error_key(&params, r#"{"a": {}}"#, &["a", "coercion"]);
    
    // error because "a" is null
    assert_error_key(&params, r#"{"a": null}"#, &["a", "coercion"]);
}

#[test]
fn is_process_boolean_require() {

    let params = valico::Builder::build(|params| {
        params.req_typed("a", valico::boolean());
    });

    assert_str_eq(&params, r#"{"a":true}"#, r#"{"a":true}"#);
    assert_str_eq(&params, r#"{"a":false}"#, r#"{"a":false}"#);
    assert_str_eq(&params, r#"{"a":"true"}"#, r#"{"a":true}"#);
    assert_str_eq(&params, r#"{"a":"false"}"#, r#"{"a":false}"#);

    assert_error_key(&params, r#"{"a": null}"#, &["a", "coercion"]);
    assert_error_key(&params, r#"{"a": 1}"#, &["a", "coercion"]);
    assert_error_key(&params, r#"{"a": "not-bool"}"#, &["a", "coercion"]);
}

#[test]
fn is_process_simple_list_require() {

    let params = valico::Builder::build(|params| {
        params.req_typed("a", valico::list());
    });

    assert_str_eq(&params, r#"{"a":[1,"2",[3]]}"#, r#"{"a":[1,"2",[3]]}"#);

    // error because "a" is object
    assert_error_key(&params, r#"{"a": {}}"#, &["a", "coercion"]);

    // error because "a" is string
    assert_error_key(&params, r#"{"a": "test"}"#, &["a", "coercion"]);
}

#[test]
fn is_process_typed_list_require() {

    let params = valico::Builder::build(|params| {
        params.req_typed("a", valico::list_of(valico::string()));
    });

    // convert all to string
    assert_str_eq(&params, r#"{"a":[1,"2",3.1]}"#, r#"{"a":["1","2","3.1"]}"#);

    // error because "a" is object
    assert_error_key(&params, r#"{"a": {}}"#, &["a", "coercion"]);

    // error because element at index(2) is not coersible to string
    assert_error_key(&params, r#"{"a": [1,2,{}]}"#, &["a", "2", "coercion"]);

}

#[test]
fn is_process_list_with_nested_require() {

    let params = valico::Builder::build(|params| {
        params.req_nested("a", valico::list(), |params| {
            params.req_typed("b", valico::string());
            params.req_typed("c", valico::list_of(valico::u64()))
        });
    });

    assert_str_eq(&params, r#"{"a":[{"b":1,"c":["1"]}]}"#, r#"{"a":[{"b":"1","c":[1]}]}"#);

    // error because element in "a" at index(0) is not coersible to string
    assert_error_key(&params, r#"{"a":[{"b":{},"c":["1"]}]}"#, &["a", "0", "b", "coercion"]);

    // error because element in "a":0:"c":0 is not coersible to string
    assert_error_key(&params, r#"{"a":[{"b":1,"c":[{}]}]}"#, &["a", "0", "c", "0", "coercion"]);

}

#[test]
fn is_process_object_require() {

    let params = valico::Builder::build(|params| {
        params.req_typed("a", valico::object());
    });

    assert_str_eq(&params, r#"{"a":{}}"#, r#"{"a":{}}"#);

    // error because "a" is array, not object
    assert_error_key(&params, r#"{"a":[]}"#, &["a", "coercion"]);

    // error because "a" is string, not object
    assert_error_key(&params, r#"{"a":""}"#, &["a", "coercion"]);

}

#[test]
fn is_process_object_with_nested_require() {

    let params = valico::Builder::build(|params| {
        params.req_nested("a", valico::object(), |params| {
            params.req_typed("b", valico::f64());
            params.req_typed("c", valico::list_of(valico::string()));
        });
    });

    assert_str_eq(&params, r#"{"a":{"b":"1.22","c":[1.112,""]}}"#, r#"{"a":{"b":1.22,"c":["1.112",""]}}"#);

    // error because "a":"b" is not a f64
    assert_error_key(&params, r#"{"a":{"b":"not-f64"},"c":[1.112,""]}"#, &["a", "b", "coercion"]);

    // error because "a":"c":"1" is object and can't be coerced to string
    assert_error_key(&params, r#"{"a":{"b":"1.22","c":[1.112,{}]}}"#, &["a", "c", "1", "coercion"]);

}

#[test]
fn is_process_require_allows_null() {

    let params = valico::Builder::build(|params| {
        params.req("a", |a| {
            a.coerce(valico::string());
        })
    });

    // error because a is not allow null explicitly
    assert_error_key(&params, r#"{"a":null}"#, &["a", "coercion"]);

    let params = valico::Builder::build(|params| {
        params.req("a", |a| {
            a.coerce(valico::string());
            a.allow_null();
        })
    });

    // ok because "a" allows null explicitly
    assert_str_eq(&params, r#"{"a":null}"#, r#"{"a":null}"#);
}


#[test]
fn is_validate_allow_values() {

    let params = valico::Builder::build(|params| {
        params.req("a", |a| {
            a.coerce(valico::string());
            a.allow_values(&["allowed1".to_string(), "allowed2".to_string()])
        })
    });

    assert_str_eq(&params, r#"{"a":"allowed1"}"#, r#"{"a":"allowed1"}"#);
    assert_str_eq(&params, r#"{"a":"allowed2"}"#, r#"{"a":"allowed2"}"#);

    // error because "a" is not in allowed list
    assert_error_key(&params, r#"{"a":"not in allowed"}"#, &["a", "validation"]);

}

#[test]
fn is_validate_reject_values() {

    let params = valico::Builder::build(|params| {
        params.req("a", |a| {
            a.coerce(valico::string());
            a.reject_values(&["rejected1".to_string(), "rejected2".to_string()])
        })
    });

    assert_str_eq(&params, r#"{"a":"some"}"#, r#"{"a":"some"}"#);

    // errors because "a" is in reject list
    assert_error_key(&params, r#"{"a":"rejected1"}"#, &["a", "validation"]);
    assert_error_key(&params, r#"{"a":"rejected2"}"#, &["a", "validation"]);

}

#[test]
fn is_validate_with_function_validator() {

    let params = valico::Builder::build(|params| {
        params.req("a", |a| {
            a.coerce(valico::u64());

            fn validate(val: &json::Json) -> Result<(), String> {
                if *val == 2us.to_json() {
                    Ok(())
                } else {
                    Err("Value is not exactly 2".to_string())
                }
            }

            a.validate_with(validate);
        })
    });

    assert_str_eq(&params, r#"{"a":"2"}"#, r#"{"a":2}"#);
    assert_error_key(&params, r#"{"a":3}"#, &["a", "validation"]);
    assert_error_key(&params, r#"{"a":"3"}"#, &["a", "validation"]);

}

#[test]
fn is_validate_with_regex() {

    let params = valico::Builder::build(|params| {
        params.req("a", |a| {
            a.coerce(valico::string());
            a.regex(regex!("^test$"));
        })
    });

    assert_str_eq(&params, r#"{"a":"test"}"#, r#"{"a":"test"}"#);

    // error because "a" is not match regex
    assert_error_key(&params, r#"{"a":"2"}"#, &["a", "validation"]);
    assert_error_key(&params, r#"{"a":"test "}"#, &["a", "validation"]);

    let params = valico::Builder::build(|params| {
        params.req("a", |a| {
            // regex can't be applied to list, so it will never be valid
            a.coerce(valico::list());
            a.regex(regex!("^test$"));
        })
    });

    // "a" is valid list but it can't pass regex validation
    assert_error_key(&params, r#"{"a":[]}"#, &["a", "validation"]);

}

#[test]
fn is_validate_opt() {

    let params = valico::Builder::build(|params| {
        params.req_defined("a");
        params.opt_typed("b", valico::u64());
    });

    // ok because a is optional
    assert_str_eq(&params, r#"{"a":"test"}"#, r#"{"a":"test"}"#);
    assert_str_eq(&params, r#"{"a":"test","b":"1"}"#, r#"{"a":"test","b":1}"#);

}

#[test]
fn is_validate_opt_with_default() {

    let params = valico::Builder::build(|params| {
        params.opt("a", |a| {
            a.default("default".to_string());
        });
    });

    assert_str_eq(&params, r#"{"a":"test"}"#, r#"{"a":"test"}"#);
    assert_str_eq(&params, r#"{}"#, r#"{"a":"default"}"#);

}

#[test]
fn is_validate_mutually_exclusive() {

    let params = valico::Builder::build(|params| {
        params.opt_defined("a");
        params.opt_defined("b");

        params.mutually_exclusive(&["a", "b"])
    });

    assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":1}"#);
    assert_str_eq(&params, r#"{"b":1}"#, r#"{"b":1}"#);
    assert_str_eq(&params, r#"{}"#, r#"{}"#);

    assert_error_key(&params, r#"{"a":1,"b":1}"#, &["$$0", "validation"]);

}

#[test]
fn is_validate_exactly_one_of() {

    let params = valico::Builder::build(|params| {
        params.opt_defined("a");
        params.opt_defined("b");

        params.exactly_one_of(&["a", "b"])
    });

    assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":1}"#);
    assert_str_eq(&params, r#"{"b":1}"#, r#"{"b":1}"#);

    assert_error_key(&params, r#"{}"#, &["$$0", "validation"]);
    assert_error_key(&params, r#"{"a":1,"b":1}"#, &["$$0", "validation"]);

}

#[test]
fn is_validate_at_least_one_of() {

    let params = valico::Builder::build(|params| {
        params.opt_defined("a");
        params.opt_defined("b");

        params.at_least_one_of(&["a", "b"])
    });

    assert_str_eq(&params, r#"{"a":1}"#, r#"{"a":1}"#);
    assert_str_eq(&params, r#"{"b":1}"#, r#"{"b":1}"#);
    assert_str_eq(&params, r#"{"a":1,"b":1}"#, r#"{"a":1,"b":1}"#);

    assert_error_key(&params, r#"{}"#, &["$$0", "validation"]);

}

#[test]
fn is_validate_with_function() {

    let params = valico::Builder::build(|params| {
        params.opt_defined("a");
        params.opt_defined("b");

        fn validate_params(_: &json::Object) -> Result<(),String> {
            Err("YOU SHALL NOT PASS".to_string())
        }

        params.validate_with(validate_params);
    });

    assert_error_key(&params, r#"{}"#, &["$$0", "validation"]);

}



