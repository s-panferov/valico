use std::{borrow::Cow, collections::HashSet};

use crate::json_schema::errors;

pub enum UnevaluatedItems {
    Bool(bool),
    Schema(url::Url),
}

impl super::Validator for UnevaluatedItems {
    fn validate(
        &self,
        val: &serde_json::Value,
        path: &str,
        scope: &crate::json_schema::scope::Scope,
        state: &super::ValidationState,
    ) -> super::ValidationState {
        let evaluated_children: HashSet<_> = state
            .evaluated
            .iter()
            .filter(|i| i.starts_with(path))
            .collect();

        let mut array = Cow::Borrowed(nonstrict_process!(val.as_array(), path));
        let mut state = super::ValidationState::new();

        for idx in 0..array.len() {
            let item_path = [path, idx.to_string().as_ref()].join("/");
            let item = &array[idx];
            if evaluated_children.contains(&item_path) {
                continue;
            }

            match self {
                UnevaluatedItems::Bool(allow_unevaluated) => {
                    if !allow_unevaluated {
                        state.errors.push(Box::new(errors::Items {
                            path: item_path,
                            detail: "Unevaluated items are not allowed".to_string(),
                        }));
                    } else {
                        state.evaluated.insert(item_path);
                    }
                }
                UnevaluatedItems::Schema(ref url) => {
                    let schema = scope.resolve(url);
                    if let Some(schema) = schema {
                        let mut result = schema.validate_in(item, item_path.as_ref());
                        if result.is_valid() {
                            state.evaluated.insert(item_path);
                            if result.replacement.is_some() {
                                array.to_mut()[idx] = result.replacement.take().unwrap();
                            }
                        }

                        state.append(result);
                    } else {
                        state.missing.push(url.clone())
                    }
                }
            }
        }

        state
    }
}
