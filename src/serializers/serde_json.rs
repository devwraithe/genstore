use std::error::Error;

use crate::serializer::Serializer;

pub struct SerdeJson;

/// Implements serialization and deserialization on `T` using the Serde JSON format.
impl<T: serde::Serialize + serde::de::DeserializeOwned> Serializer<T> for SerdeJson {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
        serde_json::to_vec(data).map_err(Into::into)
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }
}
