use url;
use std::collections;
use serialize::json;

use super::schema;
use super::keywords;
use super::helpers;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Scope {
    pub keywords: keywords::Keywords,
    schemes: collections::HashMap<url::Url, schema::Schema>,
}

#[allow(dead_code)]
impl Scope {
    pub fn new() -> Scope {
        Scope {
            keywords: keywords::default(),
            schemes: collections::HashMap::new()
        }
    }

    pub fn compile(&mut self, def: json::Json) -> Result<(), schema::SchemaError> {
        let schema = try!(schema::compile(def, &self.keywords));
        self.add(schema)
    }

    pub fn compile_and_return<'a>(&'a mut self, def: json::Json) -> Result<schema::ScopedSchema<'a>, schema::SchemaError> {
        let schema = try!(schema::compile(def, &self.keywords));
        self.add_and_return(schema)
    }

    fn add(&mut self, schema: schema::Schema) -> Result<(), schema::SchemaError> {
        let id = if schema.id.is_some() {
            schema.id.clone().unwrap()
        } else {
            url_parser!().parse(helpers::DEFAULT_SCHEMA_ID).ok().unwrap()
        };

        if !self.schemes.contains_key(&id) {
            self.schemes.insert(id, schema);
            Ok(())
        } else {
            Err(schema::SchemaError::IdConflicts)
        }
    }

    fn add_and_return<'a>(&'a mut self, schema: schema::Schema) -> Result<schema::ScopedSchema<'a>, schema::SchemaError> {
        let id = if schema.id.is_some() {
            schema.id.clone().unwrap()
        } else {
            url_parser!().parse(helpers::DEFAULT_SCHEMA_ID).ok().unwrap()
        };

        if !self.schemes.contains_key(&id) {
            self.schemes.insert(id.clone(), schema);
            Ok(schema::ScopedSchema::new(self, self.schemes.get(&id).unwrap()))
        } else {
            Err(schema::SchemaError::IdConflicts)
        }
    }

    pub fn resolve(&self, id: &url::Url) -> Option<schema::ScopedSchema<'a>> {
        let schema = self.schemes.get(id).or_else(|:| {
            // Searching for inline schema in O(N)
            for (_, schema) in self.schemes.iter() {
                let internal_schema = schema.resolve(id);
                if internal_schema.is_some() {
                    return internal_schema
                }
            }

            None
        });

        match id.fragment {
            Some(fragment) => {
                
            },
            None => Some(schema::ScopedSchema::new(self, schema)
        }
        schema.resolve_fragment()
    }
}