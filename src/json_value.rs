use std::{collections::HashMap, fmt::Display};

use serde_derive::{Deserialize, Serialize};

use crate::error::Error;

/// A list of JsonValue type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum JsonValue {
    Integer(i32),
    Float(f64),
    Bool(bool),
    String(String),
    Vec(Vec<JsonValue>),
    HashMap(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// Converts from any struct T that implements serde::Serialize trait into
    /// a JsonValue type.
    pub fn convert_from<T: serde::Serialize>(value: &T) -> Result<Self, Error> {
        let val = serde_json::to_string(&value).map_err(Error::SerdeJson)?;
        let val: JsonValue = serde_json::from_str(&val).map_err(Error::SerdeJson)?;
        Ok(val)
    }
    /// Converts from any JsonValue to any T that implements serde::Deserialize trait.
    pub fn convert_to<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        let val = serde_json::to_string(&self).map_err(Error::SerdeJson)?;
        let val: T = serde_json::from_str(&val).map_err(Error::SerdeJson)?;
        Ok(val)
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Integer(val) => write!(f, "{}", val),
            JsonValue::Float(val) => write!(f, "{}", val),
            JsonValue::Bool(val) => write!(f, "{}", val),
            JsonValue::String(val) => write!(f, "{}", val),
            JsonValue::Vec(val) => write!(f, "{:?}", val),
            JsonValue::HashMap(val) => write!(f, "{:?}", val),
        }
    }
}
