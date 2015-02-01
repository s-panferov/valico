use std::collections;
use serialize::json;
use url;

use super::super::errors;
use super::super::scope;

#[derive(Debug)]
pub enum DepKind {
    Schema(url::Url),
    Property(Vec<String>)
}

#[allow(missing_copy_implementations)]
pub struct Dependencies {
    pub items: collections::HashMap<String, DepKind>
}

impl super::Validator for Dependencies {
    fn validate(&self, object: &json::Json, path: &str, strict: bool, scope: &scope::Scope) -> super::ValidationState {

        if !object.is_object() {
            return if strict {
                super::ValidationState::new()
            } else {
                val_error!(
                    errors::WrongType {
                        path: path.to_string(),
                        detail: "The value must be an object".to_string()
                    }
                )
            }
        }

        let mut state = super::ValidationState::new();

        for (key, dep) in self.items.iter() {
            if object.find(key.as_slice()).is_some() {
                match dep {
                    &DepKind::Schema(ref url) => {
                        let schema = scope.resolve(url);
                        if schema.is_some() {
                            state.append(&mut schema.unwrap().validate_in(object, path));
                        } else {
                            state.missing.push(url.clone())
                        }
                    },
                    &DepKind::Property(ref keys) => {
                        for key in keys.iter() {
                            if !object.find(key.as_slice()).is_some() {
                                state.errors.push(Box::new(
                                    errors::Required {
                                        path: [path, key.as_slice()].connect("/")
                                    }
                                ))          
                            }
                        }
                    }
                }
            }
        }

        state
    }
}