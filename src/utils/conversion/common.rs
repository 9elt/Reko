use serde::{Deserialize, Serialize};

/** converts a generic type to serde_json Value */
pub fn to_serde_value<T: Serialize>(v: &T) -> serde_json::Value {
    serde_json::from_str(&serde_json::to_string(&v).unwrap()).unwrap()
}

/** Converts serde_json Value into the desired type
* panics when the value cannot be deserialized into the given type */
pub fn from_serde_value<'a, T: Deserialize<'a>>(v: serde_json::Value) -> T {
    <T as Deserialize>::deserialize(v).unwrap()
}