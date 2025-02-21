

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// a bloom filter uses multiple hash functions to
// map a key to a bit array
const BIT_ARRAY_SIZE: usize = 1000;
const HASH_FUNCTIONS: usize = 3;

#[allow(non_camel_case_types)]
pub struct bloom_Filter{
    bit_array: Vec<bool>
}

impl bloom_Filter{
    pub fn new() -> Self {
        bloom_Filter{bit_array: vec![false;BIT_ARRAY_SIZE]}
    }

    // hash an item of any type and multiple number
    // of times using a pre-defined seed value
    pub fn hash<T:Hash>(&self, item:&T, seed: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        (hasher.finish() as usize) % BIT_ARRAY_SIZE
    }

    pub fn insert<T:Hash>(&mut self, item: &T) {
        // remember that we use multiple hash functions to
        // map a key into a bit array for the insert method
        for i in 0..HASH_FUNCTIONS{
            let index = self.hash(item, i as u64);
            self.bit_array[index] = true;
        }
    }

    // check if a certain item exists in the set 
    // we generate the hash of the item deterministically
    pub fn contains<T:Hash>(&self, item: &T) -> bool {
        for i in 0..HASH_FUNCTIONS{
            let index = self.hash(item, i as u64);
            if !self.bit_array[index]{
                return false;
            }
        }
        true
    }
}