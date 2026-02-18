use borsh::{BorshDeserialize, BorshSerialize, from_slice, to_vec};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::marker::PhantomData;
use wincode::{SchemaRead, SchemaWrite, config::DefaultConfig};

fn main() {
    println!("Hello, world!");

    let borsh2 = Borsh;
    let wincode2 = Wincode;
    let json2 = Json;

    // Testing borsh
    let data = String::from("Hola");
    let ser = borsh2.to_bytes(&data).unwrap();
    println!("SerData: {:?}", ser);
    let deser: String = borsh2.from_bytes(&ser).unwrap();
    println!("DeserData: {:?}", deser);

    // Testing wincode
    let data2 = String::from("Hola");
    let ser2 = wincode2.to_bytes(&data2).unwrap();
    println!("SerData: {:?}", ser2);
    let deser2: String = wincode2.from_bytes(&ser2).unwrap();
    println!("DeserData: {:?}", deser2);

    // Testing json
    let data3 = String::from("Hola");
    let ser3 = json2.to_bytes(&data3).unwrap();
    println!("SerData: {:?}", ser3);
    let deser3: String = json2.from_bytes(&ser3).unwrap();
    println!("DeserData: {:?}", deser3);
}

// TRAIT
// Generic trait with T
trait Serial<T> {
    // Converts data to bytes (from T to Vec)
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>>;

    // Converts bytes to data (from Vec to T)
    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>>;
}

// STRUCT
struct Borsh; // zero-sized structs, hold no data
struct Wincode;
struct Json;

// STRUCT IMPL
// Implement borsh original traits on generic Serial trait for custom borsh struct
impl<T: BorshSerialize + BorshDeserialize> Serial<T> for Borsh {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
        let s = to_vec(data);
        s.map_err(|err| err.into())
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>> {
        let d = from_slice::<T>(bytes);
        d.map_err(|err| err.into())
    }
}

// SchemaWrite - wincodee's generic traait
// Locks wincode Src type to type T
impl<T: SchemaWrite<DefaultConfig, Src = T> + for<'a> SchemaRead<'a, DefaultConfig, Dst = T>>
    Serial<T> for Wincode
{
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
        wincode::serialize(data).map_err(|e| e.into())
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>> {
        wincode::deserialize(bytes).map_err(|e| e.into())
    }
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> Serial<T> for Json {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
        let serde = serde_json::to_vec(&data);
        serde.map_err(|e| e.into())
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>> {
        let deser = serde_json::from_slice(bytes);
        deser.map_err(|e| e.into())
    }
}

// STORAGE
struct Storage<T, S: Serial<T>> {
    data: Option<Vec<u8>>,
    serializer: S,
    _marker: PhantomData<T>,
}
impl<T, S: Serial<T>> Storage<T, S> {
    fn new(serializer: S) -> Self {
        Storage {
            data: None,
            serializer,
            _marker: PhantomData,
        }
    }

    fn save(&mut self, value: &T) {
        if !self.has_data() {
            let ser_data = self.serializer.to_bytes(value).unwrap();
            self.data = Some(ser_data);
        }
    }

    fn load(&self) -> Result<T, Box<dyn Error>> {
        // match to handle the Option type
        match &self.data {
            Some(bytes) => {
                let deser = self.serializer.from_bytes(bytes);
                deser
            }
            None => Err("No data to load".into()),
        }
    }

    fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

// TEST DATA
#[derive(
    BorshSerialize,
    BorshDeserialize,
    SchemaWrite,
    SchemaRead,
    Serialize,
    Deserialize,
    PartialEq,
    Debug,
)]
struct Person {
    name: String,
    age: u8,
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borsh() {
        let person = Person {
            name: "Devwraithe".to_string(),
            age: 30,
        };

        let mut storage = Storage::new(Borsh);

        // Save person in storage
        storage.save(&person);
        assert_eq!(storage.has_data(), true);

        // Load saved data
        let saved_person: Person = storage.load().unwrap();
        assert_eq!(saved_person, person);
    }

    #[test]
    fn test_wincode() {
        let person = Person {
            name: "Devwraithe".to_string(),
            age: 30,
        };

        let mut storage = Storage::new(Wincode);

        // Save person in storage
        storage.save(&person);
        assert_eq!(storage.has_data(), true);

        // Load saved data
        let saved_person: Person = storage.load().unwrap();
        assert_eq!(saved_person, person);
    }

    #[test]
    fn test_json() {
        let person = Person {
            name: "Devwraithe".to_string(),
            age: 30,
        };

        let mut storage = Storage::new(Json);

        // Save person in storage
        storage.save(&person);
        assert_eq!(storage.has_data(), true);

        // Load saved data
        let saved_person: Person = storage.load().unwrap();
        assert_eq!(saved_person, person);
    }
}
