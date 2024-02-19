use std::{collections::HashMap, fmt::Display};

use serde_derive::{Deserialize, Serialize};

use crate::error::Error;

/// A list of JsonElem type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum JsonElem {
    Integer(i32),
    Float(f64),
    Bool(bool),
    String(String),
    Vec(Vec<JsonElem>),
    HashMap(HashMap<String, JsonElem>),
}

impl JsonElem {
    /// Converts from any struct T that implements serde::Serialize trait into
    /// a JsonElem type.
    pub fn convert_from<T: serde::Serialize>(value: &T) -> Result<Self, Error> {
        let val = serde_json::to_string(&value).map_err(Error::SerdeJson)?;
        let val: JsonElem = serde_json::from_str(&val).map_err(Error::SerdeJson)?;
        Ok(val)
    }
    /// Converts from any JsonElem to any T that implements serde::Deserialize trait.
    pub fn convert_to<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        let val = serde_json::to_string(&self).map_err(Error::SerdeJson)?;
        let val: T = serde_json::from_str(&val).map_err(Error::SerdeJson)?;
        Ok(val)
    }
}

/// Converts from String to JsonElem
impl TryFrom<&str> for JsonElem {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(Error::SerdeJson)
    }
}

/// Converts from Bytes to JsonElem
impl TryFrom<&[u8]> for JsonElem {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(value).map_err(Error::SerdeJson)
    }
}

/// Converts from JsonElem to String
impl TryInto<String> for JsonElem {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        serde_json::to_string(&self).map_err(Error::SerdeJson)
    }
}

/// Converts from JsonElem to Bytes
impl TryInto<Vec<u8>> for JsonElem {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(&self).map_err(Error::SerdeJson)
    }
}

impl Display for JsonElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonElem::Integer(val) => write!(f, "{}", val),
            JsonElem::Float(val) => write!(f, "{}", val),
            JsonElem::Bool(val) => write!(f, "{}", val),
            JsonElem::String(val) => write!(f, "{}", val),
            JsonElem::Vec(val) => write!(f, "{:?}", val),
            JsonElem::HashMap(val) => write!(f, "{:?}", val),
        }
    }
}
