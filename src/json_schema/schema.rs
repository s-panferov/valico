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
    pub scopes: &'a mut collections::HashMap<String, Vec<String>>
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

    pub fn validate(&self, data: &json::Json) -> validators::ValidationState {
        return self.schema.validate_in_scope(data, "", self.scope);
    }

    pub fn validate_in(&self, data: &json::Json, path: &str) -> validators::ValidationState {
        return self.schema.validate_in_scope(data, path, self.scope);
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
    scopes: collections::HashMap<String, Vec<String>>
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

                let scheme = try!(Schema::compile_sub(
                    value.clone(),
                    &mut context,
                    keywords
                ));

                tree.insert(key.clone(), scheme);
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

        let id = try!(helpers::parse_url_key_with_base("id", &def, context.url));
        let ref_ = try!(helpers::parse_url_key_with_base("$ref", &def, context.url));
        let schema = try!(helpers::parse_url_key("$schema", &def));

        let tree = {
            let mut tree = collections::BTreeMap::new();

            if def.is_object() {
                let obj = def.as_object().unwrap();

                for (key, value) in obj.iter() {
                    let mut current_fragment = context.fragment.clone();
                    current_fragment.push(key.clone());

                    let mut context = WalkContext {
                        url: id.as_ref().unwrap_or(context.url),
                        fragment: current_fragment,
                        scopes: context.scopes
                    };

                    let scheme = try!(Schema::compile_sub(
                        value.clone(),
                        &mut context,
                        keywords
                    ));

                    tree.insert(key.clone(), scheme);
                }
            } else if def.is_array() {
                let array = def.as_array().unwrap();

                for (key, value) in array.iter().enumerate() {
                    let mut current_fragment = context.fragment.clone();
                    current_fragment.push(key.to_string().clone());

                    let mut context = WalkContext {
                        url: id.as_ref().unwrap_or(context.url),
                        fragment: current_fragment,
                        scopes: context.scopes
                    };

                    let scheme = try!(Schema::compile_sub(
                        value.clone(),
                        &mut context,
                        keywords
                    ));

                    tree.insert(key.to_string().clone(), scheme);
                }
            }

            tree
        };

        if id.is_some() {
            context.scopes.insert(id.clone().unwrap().serialize(), context.fragment.clone());
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

    pub fn resolve(&self, id: &str) -> Option<&Schema> {
        let path = self.scopes.get(id);
        path.map(|path| {
            let mut schema = self;
            for item in path.iter() {
                schema = schema.tree.get(item).unwrap()
            }
            schema
        })
    }

    pub fn resolve_fragment(&self, fragment: &str) -> Option<&Schema> {
        debug!("Resolve fragment: {}", fragment);
        assert!(fragment.starts_with("/"), "Can't resolve id fragments");

        let mut parts = fragment[1..].split_str("/");
        let mut schema = self;
        for part in parts {
            match schema.tree.get(part) {
                Some(sch) => schema = sch,
                None => return None
            }
        }

        Some(schema)
    }
}

impl Schema {
    fn validate_in_scope(&self, data: &json::Json, path: &str, scope: &scope::Scope) -> validators::ValidationState {
        let mut state = validators::ValidationState::new();

        for validator in self.validators.iter() {
            state.append(&mut validator.validate(data, path, false, scope))
        }

        state
    }
}

pub fn compile(def: json::Json, keywords: &keywords::Keywords) -> Result<Schema, SchemaError> {
    Schema::compile(def, keywords)
}

#[test]
fn schema_doesnt_compile_not_object() {
    assert!(Schema::compile(json::Json::Boolean(true), &keywords::default()).is_err());
}