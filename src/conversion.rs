use std::fmt;

use crate::serializer::Serializer;
pub fn convert<U, S1, S2>(
    bytes: &[u8],
    from: &S1,
    to: &S2,
) -> Result<Vec<u8>, ConversionError<S1::Error, S2::Error>>
where
    S1: Serializer<U>,
    S2: Serializer<U>,
{
    let deserialize = from
        .from_bytes(bytes)
        .map_err(ConversionError::SerializerOne)?;
    let serialize = to
        .to_bytes(&deserialize)
        .map_err(ConversionError::SerializerTwo);

    serialize
}

#[derive(Debug)]
pub enum ConversionError<E1, E2> {
    SerializerOne(E1),
    SerializerTwo(E2),
}
impl<E1: fmt::Display, E2: fmt::Display> fmt::Display for ConversionError<E1, E2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::SerializerOne(e) => write!(f, "source serializer error: {}", e),
            ConversionError::SerializerTwo(e) => write!(f, "target serializer error: {}", e),
        }
    }
}

impl<E1: std::error::Error + 'static, E2: std::error::Error + 'static> std::error::Error
    for ConversionError<E1, E2>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConversionError::SerializerOne(e) => Some(e),
            ConversionError::SerializerTwo(e) => Some(e),
        }
    }
}
