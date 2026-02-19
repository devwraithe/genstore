use crate::serializer::Serializer;

pub struct SerdeJson;

/// Implements serialization and deserialization on `T` using the Serde JSON format.
impl<T: serde::Serialize + serde::de::DeserializeOwned> Serializer<T> for SerdeJson {
    type Error = serde_json::Error;

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(data)
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        serde_json::from_slice(bytes)
    }
}
