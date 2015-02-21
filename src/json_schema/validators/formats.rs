use rustc_serialize::json;
use std::net;
use uuid;
use url;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Ipv4;

impl super::Validator for Ipv4 {
    fn validate(&self, val: &json::Json, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_string(), path);

        match string.parse::<net::Ipv4Addr>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => {
                val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: "Wrong IP address".to_string()
                    }
                )
            }
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Ipv6;

impl super::Validator for Ipv6 {
    fn validate(&self, val: &json::Json, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_string(), path);

        match string.parse::<net::Ipv6Addr>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => {
                val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: "Wrong IP address".to_string()
                    }
                )
            }
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Uuid;

impl super::Validator for Uuid {
    fn validate(&self, val: &json::Json, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_string(), path);

        match string.parse::<uuid::Uuid>() {
            Ok(_) => super::ValidationState::new(),
            Err(err) => {
                val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: format!("Malformed UUID: {:?}", err)
                    }
                )
            }
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Uri;

impl super::Validator for Uri {
    fn validate(&self, val: &json::Json, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_string(), path);

        match url::Url::parse(string) {
            Ok(_) => super::ValidationState::new(),
            Err(err) => {
                val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: format!("Malformed URI: {}", err)
                    }
                )
            }
        }
    }
}