use std::error::Error;

/// Generic trait for serialization and deserialization of type `T`.
pub trait Serializer<T> {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>>;
    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>>;
}
