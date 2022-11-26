use serde::{Deserialize, Serialize};

/** converts a generic type to serde_json Value */
pub fn to_serde_value<'a, T: Serialize + Deserialize<'a>>(v: &T) -> serde_json::Value {
    let s = serde_json::to_string(&v).unwrap();
    serde_json::from_str(&s).unwrap()
}

/** Converts serde_json Value into the desired type
* panics when the value cannot be deserialized into the given type */
pub fn from_serde_value<'a, T: Deserialize<'a>>(v: serde_json::Value) -> T {
    <T as Deserialize>::deserialize(v).unwrap()
}