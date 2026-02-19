#[cfg(test)]
mod tests {

    use gen_ser::conversion::convert;
    use gen_ser::models::Person;
    use gen_ser::serializer::{self, Serializer};
    use gen_ser::serializers::{Borsh, SerdeJson, Wincode};
    use gen_ser::storage::Storage;

    /// Generic test function for any serializer that implements `Serializer<Person>`.
    fn test_serializer<S>(serializer: S)
    where
        S: serializer::Serializer<Person>,
        S::Error: std::fmt::Debug,
    {
        let person = Person {
            name: "devwraithe".to_string(),
            age: 25,
        };

        let mut storage = Storage::new(serializer);

        let _ = storage.save(&person);
        assert!(storage.has_data());

        let saved_person: Person = storage.load().unwrap();
        assert_eq!(saved_person, person);
    }

    fn test_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let person = Person {
            name: "devwraithe".to_string(),
            age: 25,
        };

        let borsh_bytes = Borsh.to_bytes(&person).unwrap();
        let serde_json_bytes = SerdeJson.to_bytes(&person).unwrap();

        let converted_bytes = convert::<Person, _, _>(&borsh_bytes, &Borsh, &SerdeJson)?;
        assert_eq!(converted_bytes, serde_json_bytes);

        Ok(())
    }

    #[test]
    fn test_borsh() {
        test_serializer(Borsh);
    }

    #[test]
    fn test_wincode() {
        test_serializer(Wincode);
    }

    #[test]
    fn test_json() {
        test_serializer(SerdeJson);
    }

    #[test]
    fn test_convert() {
        let _ = test_conversion();
    }
}
