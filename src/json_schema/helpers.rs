use serialize::json;
use url;

use super::schema;

pub const DEFAULT_SCHEMA_ID: &'static str = "json-schema://schema";

pub fn is_default_id(id: &url::Url) -> bool {
    id.scheme == "json-schema" && match id.fragment {
        None => true,
        _ => false
    }
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