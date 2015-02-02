use serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Enum {
    pub items: Vec<json::Json>
}

impl super::Validator for Enum {
    fn validate(&self, val: &json::Json, path: &str, _strict: bool, _scope: &scope::Scope) -> super::ValidationState {
        let mut state = super::ValidationState::new();

        let mut contains = false;
        for value in self.items.iter() {
            if val == value {
                contains = true;
                break;
            }
        }

        if !contains {
            state.errors.push(Box::new(
                errors::Enum {
                    path: path.to_string()
                }
            ))  
        }
        
        state
    }
}