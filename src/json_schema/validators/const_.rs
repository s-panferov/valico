use serde_json::Value;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Const {
    pub item: Value,
}

impl super::Validator for Const {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let mut state = super::ValidationState::new();

        if *val != self.item {
            state.errors.push(Box::new(errors::Const {
                path: path.to_string(),
            }))
        }

        state
    }
}
