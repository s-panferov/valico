
use serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

use mutable_json::MutableJson;
use helpers::{coerce_error};
use ValicoResult;

pub trait Coercer: Send + Sync {
    fn coerce(&self, &mut Json) -> ValicoResult<Option<Json>>;
}

#[deriving(Copy)]
pub struct StringCoercer;

impl Coercer for StringCoercer {
    fn coerce(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
        if val.is_string() {
            Ok(None)
        } else if val.is_number() {
            Ok(Some(val.to_string().to_json()))
        } else {
            Err(
                coerce_error(format!("Can't coerce object {} to string", val))
            )
        }
    }
}

#[deriving(Copy)]
pub struct I64Coercer;

impl Coercer for I64Coercer {
    fn coerce(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
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
                None => Err(coerce_error(format!("Can't coerce string value {} to i64", val)))
            }
        } else {
            Err(coerce_error(format!("Can't coerce object {} to i64", val)))
        }
    }
}

#[deriving(Copy)]
pub struct U64Coercer;

impl Coercer for U64Coercer {
    fn coerce(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
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
                None => Err(coerce_error(format!("Can't coerce string value {} to u64", val)))
            }
        } else {
            Err(coerce_error(format!("Can't coerce object {} to u64", val)))
        }
    }
}

#[deriving(Copy)]
pub struct F64Coercer;

impl Coercer for F64Coercer {
    fn coerce(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
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
                None => Err(coerce_error(format!("Can't coerce string value {} to f64", val)))
            }
        } else {
            Err(coerce_error(format!("Can't coerce object {} to f64", val)))
        }
    }
}

#[deriving(Copy)]
pub struct BooleanCoercer;

impl Coercer for BooleanCoercer {
    fn coerce(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
        if val.is_boolean() {
            Ok(None)
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            if val == "true" {
                Ok(Some(true.to_json()))
            } else if val == "false" {
                Ok(Some(false.to_json()))
            } else {
                Err(coerce_error(format!("Can't coerce string value {} to boolean. Correct values is 'true' and 'false'", val)))
            }
        } else {
            Err(coerce_error(format!("Can't coerce object {} to boolean", val)))
        }
    }
}

#[deriving(Copy)]
pub struct NullCoercer;

impl Coercer for NullCoercer {
    fn coerce(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
        if val.is_null() {
            Ok(None)
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            if val == "" {
                Ok(Some(Json::Null))
            } else {
                Err(coerce_error(format!("Can't coerce string value {} to null. Correct value is only empty string", val)))
            }
        } else {
            Err(coerce_error(format!("Can't coerce object {} to null", val)))
        }
    }
}

pub struct ListCoercer {
    sub_coercer: Option<Box<Coercer + Send + Sync>>
}

impl ListCoercer {
    pub fn new() -> ListCoercer {
        ListCoercer {
            sub_coercer: None
        }
    }

    pub fn of_type(sub_coercer: Box<Coercer + Send + Sync>) -> ListCoercer {
        ListCoercer {
            sub_coercer: Some(sub_coercer)
        }
    }
}

impl Coercer for ListCoercer {
    fn coerce(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
        if val.is_array() {
            let array = val.as_array_mut().unwrap();
            if self.sub_coercer.is_some() {
                let sub_coercer = self.sub_coercer.as_ref().unwrap();
                let mut errors = BTreeMap::new();
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
            Err(coerce_error(format!("Can't coerce object {} to array", val)))
        }
    }
}

#[deriving(Copy)]
pub struct ObjectCoercer;

impl Coercer for ObjectCoercer {
    fn coerce(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
        if val.is_object() {
            Ok(None)    
        } else {
            Err(coerce_error(format!("Can't coerce non-object value {} to object", val)))
        }
    }
}
