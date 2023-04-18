use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::block::Block;

const BLOCKCHAIN_FILE: &str = "test.dat";

#[derive(Debug)]
pub struct BlockChain {
    len: u64,
    file: File,
}

impl BlockChain {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(&[0; 8])?;

        Ok(Self { len: 0, file })
    }

    pub fn from(path: &str) -> Result<Self, io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;
        let mut buf = [0; 8];
        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut buf)?;
        let len = u64::from_le_bytes(buf);

        Ok(Self { len, file })
    }

    pub fn push<T> (&mut self, data: T) -> Result<(), io::Error> 
    where
        T: Serialize + DeserializeOwned
    {
        let last_address = self.file.seek(SeekFrom::End(0))?;
        let block = Block::new(last_address, data);
        
        match (block.serialize(), bincode::serialize(&(self.len + 1))) {
            (Ok(serialized), Ok(new_len)) => {
                self.len += 1;
                self.file.write_all(&serialized)?;
                self.file.seek(SeekFrom::Start(0))?;
                self.file.write_all(&new_len)
            },
            _ => panic!("oporra")
        }
    }

    pub fn get<T> (&mut self, index: usize) -> Result<Option<T>, io::Error>
    where
        T: Sized + Debug + Clone + Serialize + DeserializeOwned
    {
        let size = Block::<T>::size();
        let mut buf = vec![0; size];

        if self.len > 0 {
            let ptr = size * index + 8;

            self.file.seek(SeekFrom::Start(ptr as u64))?;
            self.file.read_exact(&mut buf)?;

            let block = Block::<T>::deserialize(&buf).unwrap();
            
            Ok(Some(block.data.clone()))
        } else {
            Ok(None)
        }
    }
}

pub fn run() -> Result<(), io::Error> {
    let mut block_chain = BlockChain::new(BLOCKCHAIN_FILE)?;

    for i in (0..10).rev() {
        block_chain.push(i)?;
    }

    for i in 20..24 {
        block_chain.push(i)?;
    }

    println!("{:?}", block_chain);
    println!("{:?}", block_chain.get::<i32>(1)?);
    println!("{:?}", block_chain.get::<i32>(12)?);
    Ok(())
}