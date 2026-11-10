//! Flattened groups of optional fields that never reject their element.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

/// A group of optional fields that records why it could not be read.
pub(super) trait Lenient: DeserializeOwned {
    /// The empty group, carrying the serde error `why`.
    fn failed(why: String) -> Self;
}

/// Reads the group from the keys its element leaves over.
///
/// A malformed field never rejects the element, so `parse` accepts what it
/// accepted before the group was modelled; the error lands in the group's
/// `unreadable` field instead.
pub(super) fn lenient<'de, D: Deserializer<'de>, T: Lenient>(
    deserializer: D,
) -> Result<T, D::Error> {
    let rest = Map::<String, Value>::deserialize(deserializer)?;
    Ok(T::deserialize(Value::Object(rest)).unwrap_or_else(|err| T::failed(err.to_string())))
}
