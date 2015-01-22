
use serialize::json::{self, ToJson};
use std::collections;

use mutable_json::MutableJson;
use helpers;

#[allow(dead_code)]
#[derive(Copy)]
pub enum PrimitiveType {
    String, 
    I64, 
    U64,
    F64,
    Boolean,
    Null,
    Array,
    Object,
    // Reserved for future use in Rustless
    File
}

pub trait Coercer: Send + Sync {
    fn get_primitive_type(&self) -> PrimitiveType;
    fn coerce(&self, &mut json::Json) -> ::ValicoResult<Option<json::Json>>;
}

#[derive(Copy)]
pub struct StringCoercer;

impl Coercer for StringCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::String }
    fn coerce(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_string() {
            Ok(None)
        } else if val.is_number() {
            Ok(Some(val.to_string().to_json()))
        } else {
            Err(
                helpers::coerce_error(format!("Can't coerce object {} to string", val))
            )
        }
    }
}

#[derive(Copy)]
pub struct I64Coercer;

impl Coercer for I64Coercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::I64 }
    fn coerce(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_i64() {
            return Ok(None)
        } else if val.is_u64() {
            let val = val.as_u64().unwrap();
            return Ok(Some((val as i64).to_json()));
        } else if val.is_f64() {
            let val = val.as_f64().unwrap();
            return Ok(Some((val as i64).to_json()));
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            let converted: Option<i64> = val.parse();
            match converted {
                Some(num) => Ok(Some(num.to_json())),
                None => Err(helpers::coerce_error(format!("Can't coerce string value {} to i64", val)))
            }
        } else {
            Err(helpers::coerce_error(format!("Can't coerce object {} to i64", val)))
        }
    }
}

#[derive(Copy)]
pub struct U64Coercer;

impl Coercer for U64Coercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::U64 }
    fn coerce(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_u64() {
            return Ok(None)
        } else if val.is_i64() {
            let val = val.as_i64().unwrap();
            return Ok(Some((val as u64).to_json()));
        } else if val.is_f64() {
            let val = val.as_f64().unwrap();
            return Ok(Some((val as u64).to_json()));
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            let converted: Option<u64> = val.parse();
            match converted {
                Some(num) => Ok(Some(num.to_json())),
                None => Err(helpers::coerce_error(format!("Can't coerce string value {} to u64", val)))
            }
        } else {
            Err(helpers::coerce_error(format!("Can't coerce object {} to u64", val)))
        }
    }
}

#[derive(Copy)]
pub struct F64Coercer;

impl Coercer for F64Coercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::F64 }
    fn coerce(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_f64() {
            return Ok(None)
        } else if val.is_i64() {
            let val = val.as_i64().unwrap();
            return Ok(Some((val as f64).to_json()));
        } else if val.is_u64() {
            let val = val.as_u64().unwrap();
            return Ok(Some((val as f64).to_json()));
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            let converted: Option<f64> = val.parse();
            match converted {
                Some(num) => Ok(Some(num.to_json())),
                None => Err(helpers::coerce_error(format!("Can't coerce string value {} to f64", val)))
            }
        } else {
            Err(helpers::coerce_error(format!("Can't coerce object {} to f64", val)))
        }
    }
}

#[derive(Copy)]
pub struct BooleanCoercer;

impl Coercer for BooleanCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Boolean }
    fn coerce(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_boolean() {
            Ok(None)
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            if val == "true" {
                Ok(Some(true.to_json()))
            } else if val == "false" {
                Ok(Some(false.to_json()))
            } else {
                Err(helpers::coerce_error(format!("Can't coerce string value {} to boolean. Correct values is 'true' and 'false'", val)))
            }
        } else {
            Err(helpers::coerce_error(format!("Can't coerce object {} to boolean", val)))
        }
    }
}

#[derive(Copy)]
pub struct NullCoercer;

impl Coercer for NullCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Null }
    fn coerce(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_null() {
            Ok(None)
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            if val == "" {
                Ok(Some(json::Json::Null))
            } else {
                Err(helpers::coerce_error(format!("Can't coerce string value {} to null. Correct value is only empty string", val)))
            }
        } else {
            Err(helpers::coerce_error(format!("Can't coerce object {} to null", val)))
        }
    }
}

pub struct ArrayCoercer {
    sub_coercer: Option<Box<Coercer + Send + Sync>>
}

impl ArrayCoercer {
    pub fn new() -> ArrayCoercer {
        ArrayCoercer {
            sub_coercer: None
        }
    }

    pub fn of_type(sub_coercer: Box<Coercer + Send + Sync>) -> ArrayCoercer {
        ArrayCoercer {
            sub_coercer: Some(sub_coercer)
        }
    }
}

impl Coercer for ArrayCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Array }
    fn coerce(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_array() {
            let array = val.as_array_mut().unwrap();
            if self.sub_coercer.is_some() {
                let sub_coercer = self.sub_coercer.as_ref().unwrap();
                let mut errors = collections::BTreeMap::new();
                for i in range(0, array.len()) {
                    match sub_coercer.coerce(&mut array[i]) {
                        Ok(Some(value)) => {
                            array.remove(i);
                            array.insert(i, value);
                        },
                        Ok(None) => (),
                        Err(err) => {
                            errors.insert(i.to_string(), err.to_json());
                        }
                    }
                }

                if errors.len() == 0 {
                    Ok(None)
                } else {
                    Err(errors)
                }
            } else {
                Ok(None)
            }
        } else {
            Err(helpers::coerce_error(format!("Can't coerce object {} to array", val)))
        }
    }
}

#[derive(Copy)]
pub struct ObjectCoercer;

impl Coercer for ObjectCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Object }
    fn coerce(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_object() {
            Ok(None)    
        } else {
            Err(helpers::coerce_error(format!("Can't coerce non-object value {} to object", val)))
        }
    }
}
