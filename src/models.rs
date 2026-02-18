use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use wincode::{SchemaRead, SchemaWrite};

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
pub struct Person {
    pub name: String,
    pub age: u8,
}
