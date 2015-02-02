use serialize::json;
use url;
use url::percent_encoding;

use super::schema;

pub const DEFAULT_SCHEMA_ID: &'static str = "json-schema://schema";

pub fn is_default_id(id: &url::Url) -> bool {
    id.scheme == "json-schema" && match id.fragment {
        None => true,
        _ => false
    }
}

/// http://tools.ietf.org/html/draft-ietf-appsawg-json-pointer-07
pub fn encode(string: &str) -> String {
    percent_encoding::percent_encode(
        string.replace("~", "~0").replace("/", "~1").as_bytes(), 
        percent_encoding::FORM_URLENCODED_ENCODE_SET
    )
}

/// Encode and connect
pub fn connect(strings: &[&str]) -> String {
    strings.iter().map(|s| encode(s)).collect::<Vec<String>>().connect("/")
}

macro_rules! url_parser(
    () => (::url::UrlParser::new().scheme_type_mapper($crate::json_schema::helpers::whatwg_extended_scheme_type_mapper))
);

pub fn parse_url_key(key: &str, obj: &json::Json) -> Result<Option<url::Url>, schema::SchemaError> {
    match obj.find(key) {
        Some(value) => {
            match value.as_string() {
                Some(string) => url_parser!()
                                .parse(string)
                                .map(|url| Some(url))
                                .map_err(|err| schema::SchemaError::UrlParseError(err)),
                None => Ok(None)
            }
        },
        None => Ok(None)
    }
}

pub fn parse_url_key_with_base(key: &str, obj: &json::Json, base: &url::Url) -> Result<Option<url::Url>, schema::SchemaError> {
    match obj.find(key) {
        Some(value) => {
            match value.as_string() {
                Some(string) => url_parser!()
                                .base_url(base)
                                .parse(string)
                                .map(|url| Some(url))
                                .map_err(|err| schema::SchemaError::UrlParseError(err)),
                None => Ok(None)
            }
        },
        None => Ok(None)
    }
}

pub fn alter_fragment_path(mut url: url::Url, new_fragment: String) -> url::Url {

    let normalized_fragment = if new_fragment.starts_with("/") {
        &new_fragment[1..]
    } else {
        new_fragment.as_slice()
    };

    let result_fragment = match url.fragment {
        Some(ref fragment) if fragment.len() > 0 => {
            if !fragment.starts_with("/") {
                let mut result_fragment = "".to_string();
                let mut fragment_parts = fragment.split_str("/").map(|s| s.to_string());
                result_fragment.push_str("#");
                result_fragment.push_str(fragment_parts.next().unwrap().as_slice());
                result_fragment.push_str("/");
                result_fragment.push_str(normalized_fragment.as_slice());
                result_fragment
            } else {
                "/".to_string() + normalized_fragment
            }
        },
        _ => "/".to_string() + normalized_fragment
    };
    
    url.fragment = Some(result_fragment);
    url
}

pub fn serialize_schema_path(url: &url::Url) -> (String, Option<String>) {
    match url.fragment.as_ref() {
        Some(fragment) if fragment.len() > 0 => {
            let mut url_str = url.serialize_no_fragment();
            if !fragment.starts_with("/") {
                let fragment_parts = fragment.split_str("/").map(|s| s.to_string()).collect::<Vec<String>>();
                url_str.push_str("#");
                url_str.push_str(fragment_parts[0].as_slice());
                let fragment = if fragment_parts.len() > 1 {
                    Some("/".to_string() + fragment_parts[1..].connect("/").as_slice())
                } else {
                    None
                };
                (url_str, fragment)
            } else {
                (url_str, Some(fragment.clone()))
            }

        },
        _ => (url.serialize_no_fragment(), None)
    }
}

/// Stub function to add our "json-schema" to the url::UrlParser
pub fn whatwg_extended_scheme_type_mapper(scheme: &str) -> url::SchemeType {
    match scheme {
        "file" => url::SchemeType::FileLike,
        "ftp" => url::SchemeType::Relative(21),
        "gopher" => url::SchemeType::Relative(70),
        "http" => url::SchemeType::Relative(80),
        "https" => url::SchemeType::Relative(443),
        "ws" => url::SchemeType::Relative(80),
        "wss" => url::SchemeType::Relative(443),
        "json-schema" => url::SchemeType::Relative(80),
        _ => url::SchemeType::NonRelative,
    }
}