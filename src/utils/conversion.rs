use serde::Deserialize;

/** Converts serde_json Value into the desired type
* panics when the value cannot be deserialized into the given type */
pub fn from_json<'a, T: Deserialize<'a>>(v: serde_json::Value) -> T {
    <T as Deserialize>::deserialize(v).unwrap()
}