use addr::parser::{DomainName, EmailAddress};
use addr::psl::List;
use chrono;
use json_pointer;
use serde_json::Value;
use std::net;
use uritemplate;
use url;
use uuid;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Date;

impl super::Validator for Date {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match chrono::NaiveDate::parse_from_str(string, "%Y-%m-%d") {
            Ok(_) => {
                if string.len() == 10 {
                    super::ValidationState::new()
                } else {
                    val_error!(errors::Format {
                        path: path.to_string(),
                        detail: "Malformed Date".to_string()
                    })
                }
            }
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed date".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct DateTime;

impl super::Validator for DateTime {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match chrono::DateTime::parse_from_rfc3339(string) {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed date time".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Email;

impl super::Validator for Email {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match List.parse_email_address(string) {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed email address".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Hostname;

impl super::Validator for Hostname {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match List.parse_domain_name(string) {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed hostname".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Ipv4;

impl super::Validator for Ipv4 {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<net::Ipv4Addr>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed IP address".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Ipv6;

impl super::Validator for Ipv6 {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<net::Ipv6Addr>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed IP address".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct IRI;

impl super::Validator for IRI {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match url::Url::parse(string) {
            Ok(_) => super::ValidationState::new(),
            Err(err) => val_error!(errors::Format {
                path: path.to_string(),
                detail: format!("Malformed IRI: {}", err)
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct IRIReference;

impl super::Validator for IRIReference {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        let base_url = url::Url::parse("http://example.com/").unwrap();

        match base_url.join(string) {
            Ok(_) => super::ValidationState::new(),
            Err(err) => val_error!(errors::Format {
                path: path.to_string(),
                detail: format!("Malformed IRI reference: {}", err)
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct JsonPointer;

impl super::Validator for JsonPointer {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<json_pointer::JsonPointer<_, _>>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed JSON pointer".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Regex;

impl super::Validator for Regex {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match regex::Regex::new(string) {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed regex".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct RelativeJsonPointer;

impl super::Validator for RelativeJsonPointer {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<json_pointer::JsonPointer<_, _>>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed relative JSON pointer".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Time;

impl super::Validator for Time {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match chrono::NaiveTime::parse_from_str(string, "%H:%M:%S%.f%Z") {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(errors::Format {
                path: path.to_string(),
                detail: "Malformed time".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Uuid;

impl super::Validator for Uuid {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<uuid::Uuid>() {
            Ok(_) => super::ValidationState::new(),
            Err(err) => val_error!(errors::Format {
                path: path.to_string(),
                detail: format!("Malformed UUID: {:?}", err)
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Uri;

impl super::Validator for Uri {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match url::Url::parse(string) {
            Ok(_) => super::ValidationState::new(),
            Err(err) => val_error!(errors::Format {
                path: path.to_string(),
                detail: format!("Malformed URI: {}", err)
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct UriReference;

impl super::Validator for UriReference {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        let base_url = url::Url::parse("http://example.com/").unwrap();

        match base_url.join(string) {
            Ok(_) => super::ValidationState::new(),
            Err(err) => val_error!(errors::Format {
                path: path.to_string(),
                detail: format!("Malformed URI reference: {}", err)
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct UriTemplate;

impl super::Validator for UriTemplate {
    fn validate(&self, val: &Value, _path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        let _ = uritemplate::UriTemplate::new(string);
        super::ValidationState::new()
    }
}
