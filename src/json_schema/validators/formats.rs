use rustc_serialize::json;
use std::old_io::net::ip;
use uuid;
use url;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Ipv4;

impl super::Validator for Ipv4 {
    fn validate(&self, val: &json::Json, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_string(), path);

        match string.parse::<ip::IpAddr>() {
            Ok(ip) => match ip {
                ip::IpAddr::Ipv4Addr(..) => super::ValidationState::new(),
                ip::IpAddr::Ipv6Addr(..) => val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: "Wrong IP address type".to_string()
                    }
                )
            },
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

        match string.parse::<ip::IpAddr>() {
            Ok(ip) => match ip {
                ip::IpAddr::Ipv6Addr(..) => super::ValidationState::new(),
                ip::IpAddr::Ipv4Addr(..) => val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: "Wrong IP address type".to_string()
                    }
                )
            },
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