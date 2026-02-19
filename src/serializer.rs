// use std::error::Error;

/// Generic trait for serialization and deserialization of type `T`.
pub trait Serializer<T> {
    type Error;

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error>;
    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Self::Error>;
}
