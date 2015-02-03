use std::collections;
use regex;
use rustc_serialize::json;

use super::super::schema;
use super::super::validators;
use super::super::helpers;

#[allow(missing_copy_implementations)]
pub struct Properties;
impl super::Keyword for Properties {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let maybe_properties = def.find("properties");
        let maybe_additional = def.find("additionalProperties");
        let maybe_pattern = def.find("patternProperties");

        if !(maybe_properties.is_some() || maybe_additional.is_some() || maybe_pattern.is_some()) {
            return Ok(None)
        }
        
        let properties = if maybe_properties.is_some() {      
            let properties = maybe_properties.unwrap();    
            if properties.is_object() {
                let mut schemes = collections::HashMap::new();  
                let properties = properties.as_object().unwrap();
                for (key, value) in properties.iter() {
                    if value.is_object() {
                        schemes.insert(key.to_string(), 
                            helpers::alter_fragment_path(ctx.url.clone(), [
                                ctx.escaped_fragment().as_slice().as_slice(), 
                                "properties", 
                                helpers::encode(key).as_slice()
                            ].connect("/"))
                        );
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.connect("/"),
                            detail: "Each value of this object MUST be an object".to_string()
                        }) 
                    }
                }
                schemes
            } else {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "The value of `properties` MUST be an object.".to_string()
                }) 
            }
        } else {
            collections::HashMap::new()
        };

        let additional_properties = if maybe_additional.is_some() {
            let additional_val = maybe_additional.unwrap();
            if additional_val.is_boolean() {

                validators::properties::AdditionalKind::Boolean(additional_val.as_boolean().unwrap())

            } else if additional_val.is_object() {

                validators::properties::AdditionalKind::Schema(
                    helpers::alter_fragment_path(ctx.url.clone(), [
                        ctx.escaped_fragment().as_slice().as_slice(), 
                        "additionalProperties"
                    ].connect("/"))
                )

            } else {

                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "The value of `additionalProperties` MUST be a boolean or an object.".to_string()
                }) 

            }
        } else { 
            validators::properties::AdditionalKind::Boolean(true) 
        };

        let patterns = if maybe_pattern.is_some() {
            let pattern = maybe_pattern.unwrap();
            if pattern.is_object() {
                let pattern = pattern.as_object().unwrap();
                let mut patterns = vec![];

                for (key, value) in pattern.iter() {
                    if value.is_object() {

                        match regex::Regex::new(key.as_slice()) {
                            Ok(regex) => {
                                let url = helpers::alter_fragment_path(ctx.url.clone(), [
                                    ctx.escaped_fragment().as_slice().as_slice(), 
                                    "patternProperties", 
                                    helpers::encode(key).as_slice()
                                ].connect("/"));
                                patterns.push((regex, url));
                            },
                            Err(_) => {
                                return Err(schema::SchemaError::Malformed {
                                    path: ctx.fragment.connect("/"),
                                    detail: "Each property name of this object SHOULD be a valid regular expression.".to_string()
                                })
                            }
                        }
                        
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.connect("/"),
                            detail: "Each value of this object MUST be an object".to_string()
                        }) 
                    }
                }

                patterns

            } else {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "The value of `patternProperties` MUST be an object".to_string()
                }) 
            }
        } else { vec![] };

        Ok(Some(Box::new(validators::Properties {
            properties: properties,
            additional: additional_properties,
            patterns: patterns
        })))

    }
}

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;

#[test]
fn validate_properties() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.object("properties", |properties| {
            properties.object("prop1", |prop1| {
                prop1.set("maximum", 10);
            });
            properties.object("prop2", |prop2| {
                prop2.set("minimum", 11);
            });
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 11);
    }).unwrap()).is_valid(), true);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 11);
        obj.set("prop2", 11);
    }).unwrap()).is_valid(), false);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 10);
    }).unwrap()).is_valid(), false);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 11);
        obj.set("prop3", 1000); // not validated
    }).unwrap()).is_valid(), true);
}

#[test]
fn validate_kw_properties() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.object("properties", |properties| {
            properties.object("id", |prop1| {
                prop1.set("maximum", 10);
            });
            properties.object("items", |prop2| {
                prop2.set("minimum", 11);
            });
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("id", 10);
        obj.set("items", 11);
    }).unwrap()).is_valid(), true);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("id", 11);
        obj.set("items", 11);
    }).unwrap()).is_valid(), false);

}

#[test]
fn validate_pattern_properties() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.object("properties", |properties| {
            properties.object("prop1", |prop1| {
                prop1.set("maximum", 10);
            });
        });
        schema.object("patternProperties", |properties| {
            properties.object("prop.*", |prop| {
                prop.set("maximum", 1000);
            });
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 11);
    }).unwrap()).is_valid(), false);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 1000);
    }).unwrap()).is_valid(), true);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 1001);
    }).unwrap()).is_valid(), false);
}

#[test]
fn validate_additional_properties_false() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.object("properties", |properties| {
            properties.object("prop1", |prop1| {
                prop1.set("maximum", 10);
            });
        });
        schema.object("patternProperties", |properties| {
            properties.object("prop.*", |prop| {
                prop.set("maximum", 1000);
            });
        });
        schema.set("additionalProperties", false);
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 1000);
    }).unwrap()).is_valid(), true);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 1000);
        obj.set("some_other", 0);
    }).unwrap()).is_valid(), false);
}

#[test]
fn validate_additional_properties_schema() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.object("properties", |properties| {
            properties.object("prop1", |prop1| {
                prop1.set("maximum", 10);
            });
        });
        schema.object("patternProperties", |properties| {
            properties.object("prop.*", |prop| {
                prop.set("maximum", 1000);
            });
        });
        schema.object("additionalProperties", |additional| {
            additional.set("maximum", 5)
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 1000);
        obj.set("some_other", 5);
    }).unwrap()).is_valid(), true);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 10);
        obj.set("prop2", 1000);
        obj.set("some_other", 6);
    }).unwrap()).is_valid(), false);
}

#[test]
fn malformed() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("properties", false);
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("patternProperties", false);
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.object("patternProperties", |pattern| {
            pattern.set("test", 1)
        });
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.object("patternProperties", |pattern| {
            pattern.object("((", |_malformed| {})
        });
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("additionalProperties", 10);
    }).unwrap()).is_err());
}