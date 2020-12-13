use serde_json::Value;
use std::borrow::Cow;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Contains {
    pub url: url::Url,
}

impl super::Validator for Contains {
    fn validate(&self, val: &Value, path: &str, scope: &scope::Scope) -> super::ValidationState {
        let mut array = Cow::Borrowed(nonstrict_process!(val.as_array(), path));

        let schema = scope.resolve(&self.url);
        let mut state = super::ValidationState::new();

        if let Some(schema) = schema {
            let mut any_matched = false;
            for idx in 0..array.len() {
                let item_path = [path, idx.to_string().as_ref()].join("/");
                let item = &array[idx];
                let mut result = schema.validate_in(item, item_path.as_ref());
                if result.is_valid() {
                    any_matched = true;
                    if let Some(result) = result.replacement.take() {
                        array.to_mut()[idx] = result;
                    }
                    break;
                }
            }

            if !any_matched {
                state.errors.push(Box::new(errors::Contains {
                    path: path.to_string(),
                }))
            }
        } else {
            state.missing.push(self.url.clone());
        }

        state.set_replacement(array);
        state
    }
}
