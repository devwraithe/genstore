use std::error::Error;
use std::marker::PhantomData;

use crate::serializer::Serializer;

/// In-memory byte store
pub struct Storage<T, S: Serializer<T>> {
    data: Option<Vec<u8>>,
    serializer: S,
    _marker: PhantomData<T>,
}

impl<T, S: Serializer<T>> Storage<T, S> {
    pub fn new(serializer: S) -> Self {
        Self {
            data: None,
            serializer,
            _marker: PhantomData,
        }
    }

    /// Serialize and store value if not already
    pub fn save(&mut self, value: &T) {
        if self.data.is_none() {
            self.data = Some(
                self.serializer
                    .to_bytes(value)
                    .expect("serialization failed"),
            );
        }
    }

    /// Deserialize and return the stored value.
    pub fn load(&self) -> Result<T, Box<dyn Error>> {
        let bytes = self.data.as_deref().ok_or("no data stored")?;
        self.serializer.from_bytes(bytes)
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}
