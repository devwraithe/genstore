use crate::serializer::Serializer;
use std::io::Error;

pub struct Borsh;

/// Implements serialization and deserialization on `T` using the Borsh format.
impl<T: borsh::BorshSerialize + borsh::BorshDeserialize> Serializer<T> for Borsh {
    type Error = Error;

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error> {
        borsh::to_vec(data)
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        borsh::from_slice(bytes)
    }
}
