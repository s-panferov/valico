use serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct UniqueItems;
impl super::Validator for UniqueItems {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidatorResult {
        let array = strict_process!(val.as_array(), path, strict, "The value must be an array");

        // TODO we need some quicker algorithm for this

        let mut unique = true;
        'main: for (idx, item_i) in array.iter().enumerate() {
            for item_j in array[..idx].iter() {
                if item_i == item_j {
                    unique = false;
                    break 'main;
                }
            }

            for item_j in array[(idx + 1)..].iter() {
                if item_i == item_j {
                    unique = false;
                    break 'main;
                }
            }
        }

        if unique {
            Ok(())
        } else {
            Err(val_error!(
                errors::UniqueItems {
                    path: path.to_string()
                }
            ))
        }
    }
}