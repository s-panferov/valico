use rustc_serialize::json;
use regex;

use super::super::errors;

impl super::Validator for regex::Regex {
    fn validate(&self, val: &json::Json, path: &str, strict: bool) -> super::ValidatorResult {

        let string = strict_process!(val.as_string(), path, strict, "The value must be a string");

        if self.is_match(string) {
            Ok(())
        } else {
            Err(vec![
                Box::new(errors::WrongValue {
                    path: path.to_string(),
                    detail: Some("Value is not matched by required pattern".to_string())
                })
            ])
        }
    }
}
