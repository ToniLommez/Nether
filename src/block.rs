use bincode::ErrorKind;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Block<T> {
    pub address: u64,
    pub data: T,
}

impl<'a, T> Block<T> 
where
    T: Serialize + DeserializeOwned
{
    pub fn new(address: u64, data: T) -> Self {
        Self { address, data }
    }

    pub fn deserialize(buf: &'a [u8]) -> Result<Self, Box<ErrorKind>> {
        bincode::deserialize::<'a, Self>(buf)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Box<ErrorKind>> {
        bincode::serialize(&self)
    }

    pub fn size() -> usize {
        std::mem::size_of::<u64>() + std::mem::size_of::<T>()
    }
}
