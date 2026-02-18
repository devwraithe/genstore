#[cfg(test)]
mod tests {

    use gen_ser::models::Person;
    use gen_ser::serializer;
    use gen_ser::serializers::{Borsh, Json, Wincode};
    use gen_ser::storage::Storage;

    /// Generic test function for any serializer that implements `Serializer<Person>`.
    fn test_serializer<S>(serializer: S)
    where
        S: serializer::Serializer<Person>,
    {
        let person = Person {
            name: "Devwraithe".to_string(),
            age: 30,
        };

        let mut storage = Storage::new(serializer);

        storage.save(&person);
        assert!(storage.has_data());

        let saved_person: Person = storage.load().unwrap();
        assert_eq!(saved_person, person);
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
        test_serializer(Json);
    }
}
