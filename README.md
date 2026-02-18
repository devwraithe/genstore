# Gen Store

A generic, format-agnostic storage mechanism in Rust. Data is serialized into bytes on save and deserialized back on load — with the serialization format swappable at the type level via a common `Serializer` trait. Supported formats: **Borsh**, **Wincode**, and **Serde JSON**.

## Project Structure

```
gen-ser/
├── src/
│   ├── serializers/
│   │   ├── borsh.rs        # Borsh serializer implementation
│   │   ├── serde_json.rs   # Serde JSON serializer implementation
│   │   ├── wincode.rs      # Wincode serializer implementation
│   │   └── mod.rs          # Re-exports all serializers
│   ├── lib.rs              # Crate root, module declarations
│   ├── models.rs           # Example data model (Person)
│   ├── serializer.rs       # Generic Serializer trait definition
│   └── storage.rs          # Generic Storage wrapper
├── tests/
│   └── genstore.rs         # Integration tests
├── Cargo.toml
└── README.md
```

## Serializer Trait

The `Serializer` trait is the core abstraction. Any type implementing it can be plugged into `Storage` to control how data is encoded and decoded.

```rust
pub trait Serializer<T> {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>>;
    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>>;
}
```

## Serializers

Each serializer is a zero-sized marker struct that implements `Serializer<T>` with the appropriate trait bounds for its format.

### Borsh

A direct 1:1 mapping to the two operations performed. `BorshSerialize` lets `to_vec` walk `T`'s fields and write them as bytes; `BorshDeserialize` lets `from_slice` reconstruct a `T` from those bytes.

```rust
pub struct Borsh;

impl<T: borsh::BorshSerialize + borsh::BorshDeserialize> Serializer<T> for Borsh {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
        borsh::to_vec(data).map_err(Into::into)
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>> {
        borsh::from_slice(bytes).map_err(Into::into)
    }
}
```

### Wincode

`DefaultConfig` pins the encoding rules (endianness, alignment, etc.). `Src = T` and `Dst = T` are associated type constraints that guarantee you're writing and reading back the same type `T`, not some intermediate. The `for<'a>` is a higher-ranked trait bound (HRTB) — it says `T` must be readable for _any_ lifetime `'a`, which is necessary because the byte slice passed to `from_bytes` can have any lifetime the caller provides.

```rust
pub struct Wincode;

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
```

### Serde JSON

`Serialize` is straightforward. The notable bound is `DeserializeOwned` rather than `Deserialize<'de>` — this is because `serde_json::from_slice` returns a fully owned value that borrows nothing from the input buffer. `DeserializeOwned` is shorthand for `for<'de> Deserialize<'de>`, enforcing that `T` owns all its data after deserialization.

```rust
pub struct SerdeJson;

impl<T: serde::Serialize + serde::de::DeserializeOwned> Serializer<T> for SerdeJson {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
        serde_json::to_vec(data).map_err(Into::into)
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Box<dyn Error>> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }
}
```

## Storage

`Storage<T, S>` is a generic in-memory store that holds serialized bytes and delegates encode/decode to a provided `Serializer`.

### Struct

```rust
pub struct Storage<T, S: Serializer<T>> {
    data: Option<Vec<u8>>,
    serializer: S,
    _marker: PhantomData<T>,
}
```

### API

```rust
// Create a new Storage with a given serializer
pub fn new(serializer: S) -> Self

// Serialize and store a value. No-ops if data is already stored.
pub fn save(&mut self, value: &T)

// Deserialize and return the stored value.
pub fn load(&self) -> Result<T, Box<dyn Error>>

// Returns true if data has been saved.
pub fn has_data(&self) -> bool
```

## Test Model

`Person` is the example model used across tests. It derives all necessary traits to be compatible with every serializer.

```rust
#[derive(
    BorshSerialize, BorshDeserialize,
    SchemaWrite, SchemaRead,
    Serialize, Deserialize,
    PartialEq, Debug,
)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
```

## Usage

```rust
let person = Person { name: "devwraithe".to_string(), age: 30 };

// Borsh
let mut storage = Storage::new(Borsh);
storage.save(&person);
let saved_person: Person = storage.load().unwrap();

// Wincode
let mut storage = Storage::new(Wincode);
storage.save(&person);
let saved_person: Person = storage.load().unwrap();

// Serde JSON
let mut storage = Storage::new(SerdeJson);
storage.save(&person);
let saved_person: Person = storage.load().unwrap();
```

## Run Tests

```bash
cargo test
```

## Takeaways

- **Trait-based polymorphism over enums:** Using a generic `Serializer<T>` trait means the format is selected at compile time with zero runtime overhead, rather than branching on an enum at runtime.
- **PhantomData for type safety:** `Storage<T, S>` uses `PhantomData<T>` to associate the value type with the storage without actually holding a `T`, keeping the struct lean while preserving type-checked load/save operations.
- **Decoupled concerns:** Each serializer lives in its own file and only knows about its own format. `Storage` knows nothing about serialization formats — it only depends on the trait.
- **HRTBs enable lifetime-flexible APIs:** Wincode's `for<'a>` bound means the impl works regardless of how long the input byte slice lives, which is what makes a generic `from_bytes(&[u8])` signature possible.
- **Swapping serializers swaps the entire contract:** when you change the serializer in `Storage`, you change the trait bounds `T` must satisfy. If `T` doesn't meet them, it won't compile — a stronger guarantee than a runtime format error.
