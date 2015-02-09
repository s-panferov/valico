use url;
use std::collections;
use rustc_serialize::json;

use super::schema;
use super::keywords;
use super::helpers;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Scope {
    keywords: keywords::Keywords,
    schemes: collections::HashMap<String, schema::Schema>,
}

#[allow(dead_code)]
impl Scope {
    pub fn new() -> Scope {
        Scope {
            keywords: keywords::default(),
            schemes: collections::HashMap::new()
        }
    }

    pub fn compile(&mut self, def: json::Json) -> Result<url::Url, schema::SchemaError> {
        let schema = try!(schema::compile(def, &self.keywords));
        
        let id = if schema.id.is_some() {
            schema.id.clone().unwrap()
        } else {
            url_parser!().parse(helpers::DEFAULT_SCHEMA_ID).ok().unwrap()
        };

        try!(self.add(&id, schema));
        Ok(id)
    }

    pub fn compile_with_id(&mut self, id: &url::Url, def: json::Json) -> Result<(), schema::SchemaError> {
        let schema = try!(schema::compile(def, &self.keywords));
        self.add(id, schema)
    }

    pub fn compile_and_return<'a>(&'a mut self, def: json::Json) -> Result<schema::ScopedSchema<'a>, schema::SchemaError> {
        let schema = try!(schema::compile(def, &self.keywords));

        let id = if schema.id.is_some() {
            schema.id.clone().unwrap()
        } else {
            url_parser!().parse(helpers::DEFAULT_SCHEMA_ID).ok().unwrap()
        };

        self.add_and_return(&id, schema)
    }

    pub fn compile_and_return_with_id<'a>(&'a mut self, id: &url::Url, def: json::Json) -> Result<schema::ScopedSchema<'a>, schema::SchemaError> {
        let schema = try!(schema::compile(def, &self.keywords));
        self.add_and_return(id, schema)
    }

    fn keywords(&self) -> &keywords::Keywords {
        &self.keywords
    }

    fn add_keyword<T>(&mut self, keyword: T) where T: keywords::Keyword + 'static {
        self.keywords.push(Box::new(keyword));
    }

    fn add(&mut self, id: &url::Url, schema: schema::Schema) -> Result<(), schema::SchemaError> {
        let (id_str, fragment) = helpers::serialize_schema_path(id);

        match fragment {
            Some(_) => return Err(schema::SchemaError::WrongId),
            None => ()
        }

        if !self.schemes.contains_key(&id_str) {
            self.schemes.insert(id_str, schema);
            Ok(())
        } else {
            Err(schema::SchemaError::IdConflicts)
        }
    }

    fn add_and_return<'a>(&'a mut self, id: &url::Url, schema: schema::Schema) -> Result<schema::ScopedSchema<'a>, schema::SchemaError> {
        let (id_str, fragment) = helpers::serialize_schema_path(id);

        match fragment {
            Some(_) => return Err(schema::SchemaError::WrongId),
            None => ()
        }

        if !self.schemes.contains_key(&id_str) {
            self.schemes.insert(id_str.clone(), schema);
            Ok(schema::ScopedSchema::new(self, self.schemes.get(&id_str).unwrap()))
        } else {
            Err(schema::SchemaError::IdConflicts)
        }
    }

    pub fn resolve<'a>(&'a self, id: &url::Url) -> Option<schema::ScopedSchema<'a>> {
        let (schema_path, fragment) = helpers::serialize_schema_path(id);

        let schema = self.schemes.get(&schema_path).or_else(|| {
            // Searching for inline schema in O(N)
            for (_, schema) in self.schemes.iter() {
                let internal_schema = schema.resolve(schema_path.as_slice());
                if internal_schema.is_some() {
                    return internal_schema
                }
            }

            None
        });

        schema.and_then(|schema| {
            match fragment {
                Some(ref fragment) => {
                    schema.resolve_fragment(fragment.as_slice()).map(|schema| {
                        schema::ScopedSchema::new(self, schema)
                    })
                },
                None => Some(schema::ScopedSchema::new(self, schema))
            }
        })        
    }
}

#[cfg(test)]
use jsonway;

#[test]
fn lookup() {
    let mut scope = Scope::new();
    
    scope.compile(jsonway::object(|schema| {
        schema.set("id", "http://example.com/schema".to_string())
    }).unwrap()).ok();

    scope.compile(jsonway::object(|schema| {
        schema.set("id", "http://example.com/schema#sub".to_string());
        schema.object("subschema", |subschema| {
            subschema.set("id", "#subschema".to_string());
        })
    }).unwrap()).ok();

    assert!(scope.resolve(&url::Url::parse("http://example.com/schema").ok().unwrap()).is_some());
    assert!(scope.resolve(&url::Url::parse("http://example.com/schema#sub").ok().unwrap()).is_some());
    assert!(scope.resolve(&url::Url::parse("http://example.com/schema#sub/subschema").ok().unwrap()).is_some());
    assert!(scope.resolve(&url::Url::parse("http://example.com/schema#subschema").ok().unwrap()).is_some());
}