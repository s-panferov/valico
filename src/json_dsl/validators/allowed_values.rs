use rustc_serialize::json;

use super::super::errors;

pub struct AllowedValues {
    allowed_values: Vec<json::Json>
}

impl AllowedValues {
    pub fn new(values: Vec<json::Json>) -> AllowedValues {
        AllowedValues {
            allowed_values: values
        }
    }
}

impl super::Validator for AllowedValues {
    fn validate(&self, val: &json::Json, path: &str, _strict: bool) -> super::ValidatorResult {
        let mut matched = false;
        for allowed_value in self.allowed_values.iter() {
            if val == allowed_value { matched = true; }
        }

        if matched {
            Ok(())
        } else {
            Err(vec![
                Box::new(errors::WrongValue {
                    path: path.to_string(),
                    detail: Some("Value is not among allowed list".to_string())
                })
            ])
        }
    }
}