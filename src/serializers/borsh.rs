use crate::serializer::Serializer;
use std::error::Error;

pub struct Borsh;

/// Implements serialization and deserialization on `T` using the Borsh format.
impl<T: borsh::BorshSerialize + borsh::BorshDeserialize> Serializer<T> for Borsh {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
        borsh::to_vec(data).map_err(Into::into)
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>> {
        borsh::from_slice(bytes).map_err(Into::into)
    }
}
