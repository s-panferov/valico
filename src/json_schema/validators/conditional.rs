use serde_json::Value;
use url;

use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Conditional {
    pub if_: url::Url,
    pub then_: Option<url::Url>,
    pub else_: Option<url::Url>,
}

impl super::Validator for Conditional {
    fn validate(&self, val: &Value, path: &str, scope: &scope::Scope) -> super::ValidationState {
        let mut state = super::ValidationState::new();

        let schema_if_ = scope.resolve(&self.if_);
        if schema_if_.is_some() {
            let schema_if = schema_if_.unwrap();

            // TODO should the validation be strict?
            let if_path = [path, "if"].join("/");
            if schema_if.validate_in(val, if_path.as_ref()).is_valid() {
                if self.then_.is_some() {
                    let schema_then_ = scope.resolve(&self.then_.as_ref().unwrap());

                    if schema_then_.is_some() {
                        let schema_then = schema_then_.unwrap();
                        let then_path = [path, "then"].join("/");
                        state.append(schema_then.validate_in(val, then_path.as_ref()));
                    } else {
                        state.missing.push(self.then_.as_ref().unwrap().clone());
                    }
                }
            } else if self.else_.is_some() {
                let schema_else_ = scope.resolve(&self.else_.as_ref().unwrap());

                if schema_else_.is_some() {
                    let schema_else = schema_else_.unwrap();
                    let else_path = [path, "else"].join("/");
                    state.append(schema_else.validate_in(val, else_path.as_ref()));
                } else {
                    state.missing.push(self.else_.as_ref().unwrap().clone());
                }
            }
        } else {
            state.missing.push(self.if_.clone());
        }
        state
    }
}
