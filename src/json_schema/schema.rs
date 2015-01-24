use url;
use std::collections;
use serialize::json::{self};

use super::helpers;
use super::scope;
use super::keywords;
use super::validators;

#[derive(Debug)]
pub struct WalkContext<'a> {
    pub url: &'a url::Url,
    pub fragment: Vec<String>,
    pub scopes: &'a mut collections::HashMap<url::Url, Vec<String>>
}

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum SchemaError {
    IdConflicts,
    NotAnObject,
    UrlParseError(url::ParseError),
    Malformed {
        path: String,
        detail: String
    }
}

#[derive(Debug)]
pub struct ScopedSchema<'a> {
    scope: &'a scope::Scope,
    schema: &'a Schema
}

impl<'a> ScopedSchema<'a> {
    pub fn new(scope: &'a scope::Scope, schema: &'a Schema) -> ScopedSchema<'a> {
        ScopedSchema {
            scope: scope,
            schema: schema
        }
    }

    pub fn validate(&self, data: &json::Json) -> super::ValidationResult {
        return self.schema.validate_in_scope(data, self.scope);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Schema {
    pub id: Option<url::Url>,
    ref_: Option<url::Url>,
    schema: Option<url::Url>,
    original: json::Json,
    tree: collections::BTreeMap<String, Schema>,
    validators: validators::Validators,
    scopes: collections::HashMap<url::Url, Vec<String>>
}

impl Schema {
    fn compile(def: json::Json, keywords: &keywords::Keywords) -> Result<Schema, SchemaError> {
        if !def.is_object() {
            return Err(SchemaError::NotAnObject)
        }

        let id = try!(helpers::parse_url_key("id", &def));
        let ref_ = try!(helpers::parse_url_key("$ref", &def));
        let schema = try!(helpers::parse_url_key("$schema", &def));

        let context_url = id.clone().unwrap_or_else(|| url_parser!().parse("json-schema://schema#").ok().unwrap());

        let (tree, mut scopes) = {
            let mut tree = collections::BTreeMap::new();
            let obj = def.as_object().unwrap();

            let mut scopes = collections::HashMap::new();

            for (key, value) in obj.iter() {
                let mut context = WalkContext {
                    url: &context_url,
                    fragment: vec![key.clone()],
                    scopes: &mut scopes
                };
                if value.is_object() {
                    let scheme = try!(Schema::compile_sub(
                        value.clone(),
                        &mut context,
                        keywords
                    ));
                    tree.insert(key.clone(), scheme);
                }
            }

            (tree, scopes)
        };

        let validators = try!(Schema::compile_keywords(&def, &WalkContext {
            url: &context_url,
            fragment: vec![],
            scopes: &mut scopes,
        }, keywords));

        let schema = Schema {
            id: id,
            ref_: ref_,
            schema: schema,
            original: def,
            tree: tree,
            validators: validators,
            scopes: scopes
        };

        Ok(schema)
    }

    fn compile_keywords(def: &json::Json, context: &WalkContext, keywords: &keywords::Keywords) -> Result<validators::Validators, SchemaError> {
        let mut validators = vec![];

        for keyword in keywords.iter() {
            let maybe_validator = try!(keyword.compile(def, context));
            if maybe_validator.is_some() {
                validators.push(maybe_validator.unwrap())
            }
        }

        Ok(validators)
    }

    fn compile_sub(def: json::Json, context: &mut WalkContext, keywords: &keywords::Keywords) -> Result<Schema, SchemaError> {
        assert!(def.is_object());

        let id = try!(helpers::parse_url_key_with_base("id", &def, context.url));
        let ref_ = try!(helpers::parse_url_key_with_base("$ref", &def, context.url));
        let schema = try!(helpers::parse_url_key("$schema", &def));

        let tree = {
            let mut tree = collections::BTreeMap::new();
            let obj = def.as_object().unwrap();

            for (key, value) in obj.iter() {
                let mut current_fragment = context.fragment.clone();
                current_fragment.push(key.clone());

                let mut context = WalkContext {
                    url: id.as_ref().unwrap_or(context.url),
                    fragment: current_fragment,
                    scopes: context.scopes
                };

                if value.is_object() {
                    let value = value.as_object().unwrap();
                    let scheme = try!(Schema::compile_sub(
                        json::Json::Object(value.clone()),
                        &mut context,
                        keywords
                    ));
                    tree.insert(key.clone(), scheme);
                }
            }

            tree
        };

        if id.is_some() {
            context.scopes.insert(id.clone().unwrap(), context.fragment.clone());
        }

        let validators = try!(Schema::compile_keywords(&def, context, keywords));

        let schema = Schema {
            id: id,
            ref_: ref_,
            schema: schema,
            original: def,
            tree: tree,
            validators: validators,
            scopes: collections::HashMap::new()
        };

        Ok(schema)
    }

    pub fn resolve(&self, id: &url::Url) -> Option<&Schema> {
        let path = self.scopes.get(id);
        path.map(|path| {
            let mut schema = self;
            for item in path.iter() {
                schema = schema.tree.get(item).unwrap()
            }
            schema
        })
    }
}

impl Schema {
    fn validate_in_scope(&self, data: &json::Json, scope: &scope::Scope) -> super::ValidationResult {
        let mut error = validators::ValidatorError {
            errors: vec![],
            missing: vec![]
        };

        for validator in self.validators.iter() {
            match validator.validate(data, "", false, scope) {
                Err(mut err) => {
                    error.append(&mut err)
                },
                Ok(()) => ()
            }
        }

        let validators::ValidatorError{errors, missing} = error;

        return super::ValidationResult {
            valid: errors.len() == 0,
            errors: errors,
            missing: missing
        }
    }
}

pub fn compile(def: json::Json, keywords: &keywords::Keywords) -> Result<Schema, SchemaError> {
    Schema::compile(def, keywords)
}

#[test]
fn schema_doesnt_compile_not_object() {
    assert!(Schema::compile(json::Json::Boolean(true), &keywords::default()).is_err());
}