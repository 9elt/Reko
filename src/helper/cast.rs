use serde::{Deserialize, Serialize};

/** takse a generic type (with Serialize and Deserialize traits),
and returns the equivalent serde_json Value */
pub fn to_json_type<'a, T: Serialize + Deserialize<'a>>(v: &T) -> serde_json::Value {
    let s = serde_json::to_string(&v).unwrap();
    serde_json::from_str(&s).unwrap()
}

pub fn _to_json<T: Serialize>(v: &T) -> String {
    serde_json::to_string(&v).unwrap()
}

/** Converts serde_json Value into the desired type (needs Deserialize trait)
* panics when the value cannot be deserialized into the given type */
pub fn from_json<'a, T: Deserialize<'a>>(v: serde_json::Value) -> T {
    <T as Deserialize>::deserialize(v).unwrap()
}