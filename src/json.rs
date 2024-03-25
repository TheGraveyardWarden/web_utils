use serde::Serialize;
use serde_json::{self, Map, Value};
use crate::err::Result;

pub trait ToJson: Serialize {
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

impl ToJson for Map<String, Value> {}
impl ToJson for Vec<Map<String, Value>> {}
