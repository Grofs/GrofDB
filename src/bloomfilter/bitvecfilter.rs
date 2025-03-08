use std::hash::{Hash, Hasher};
use fnv::FnvHasher;
use bitvec::prelude::*;
use std::collections::hash_map::DefaultHasher;

// these values are customisable to suit bloom
// filter sets algorithms and usage
const BIT_ARRAY_SIZE: usize = 1000;
const HASH_FUNCTIONS:usize = 5;

// custom bloomfilter implementation using bitvec
// and fowler noll vol hashing mechanisms
#[allow(non_camel_case_types)]
pub struct bloom_Filter_Bitvec{
    // bitvec is an efficient data structure for 
    // holding vector values of data bits
    pub bit_array: BitVec
}

impl bloom_Filter_Bitvec {
    pub fn new() -> Self {
        // the bloom filter basically works by using multiple
        // hash functions to map a key to a bit array of bool values
        bloom_Filter_Bitvec{bit_array:bitvec![0;BIT_ARRAY_SIZE]}
    }

    // hash the item to be inserted or retrieved with a variable
    // seed value multiple times based on the hash function
    pub fn hash<T:Hash>(&self, items:&T, seed:u64) -> usize {
        let mut hasher = FnvHasher::default();
        items.hash(&mut hasher);
        seed.hash(&mut hasher);
        (hasher.finish() as usize) % BIT_ARRAY_SIZE
    }

    pub fn dhash<T:Hash>(&self, item_to_hash:T, seed:u64) -> usize {
        let default_hasher = DefaultHasher::new();
        item_to_hash.hash(&mut default_hasher);
        seed.hash(&mut default_hasher);
        (default_hasher.finish() as usize) % BIT_ARRAY_SIZE
    }

    // add a new item to the bit vec to enable us map
    // data for bloom filter retrieval mechanism
    pub fn insert<T:Hash>(&mut self, items:&T){
        for i in 0..HASH_FUNCTIONS{
            let index = self.hash(items, i as u64);
            self.bit_array.set(index, true);
        }
    }

    // fetch the item from the bit array if it exists
    // or return false if the value for an index does not exist
    pub fn contains<T:Hash>(&self, item:&T) -> bool {
        for i in 0..HASH_FUNCTIONS{
            let index = self.hash(items, i as u64);
            if !self.bit_array.get(index){return false;}
        }
        true
    }
}