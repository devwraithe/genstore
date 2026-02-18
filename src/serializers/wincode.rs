use crate::serializer::Serializer;
use std::error::Error;
use wincode::{SchemaRead, SchemaWrite, config::DefaultConfig};

pub struct Wincode;

/// Implements serialization and deserialization on `T` using the Wincode format.
impl<T: SchemaWrite<DefaultConfig, Src = T> + for<'a> SchemaRead<'a, DefaultConfig, Dst = T>>
    Serializer<T> for Wincode
{
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
        wincode::serialize(data).map_err(Into::into)
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>> {
        wincode::deserialize(bytes).map_err(Into::into)
    }
}
