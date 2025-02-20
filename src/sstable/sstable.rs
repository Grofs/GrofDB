use std::{collections::{BTreeMap, HashMap}, error::Error, fs::File, io::Read, sync::Mutex};
use std::io::Result;
use serde::Deserialize;

use crate::memtable::memtable;

type string = String;
#[allow(non_camel_case_types)]
pub struct sorted_Entry<T, U>
where
    T: Eq + Ord,
    U: Eq + Ord,
{
    sorted_key: T,
    sorted_val: U,
}

#[allow(non_camel_case_types)]
pub struct s_Table<T, U>
where
    T: Eq + Ord,
    U: Eq + Ord,
{
    sentry_index: BTreeMap<&'static str, sorted_Entry<T, U>>,
}

#[allow(non_camel_case_types)]
pub struct sorted_Table<T, U>
where
    T: Eq + Ord,
    U: Eq + Ord,
{
    sstable: Vec<s_Table<T, U>>
}

impl <T,U> sorted_Entry<T, U>
where 
    T: Eq + Ord,
    U: Eq + Ord,
{
    pub fn new(val:&Vec<u8>)->Result<bool>{
        let stable_name = "sstable_";
        let mut sdb = Mutex::new(HashMap::<T, U>::new());
        let mtable_dump = memtable::dump(&mut sdb, &stable_name, val)?.is_some();
        Ok(mtable_dump)
    }

    pub fn read<T, U>(file:&str)->Result<BTreeMap<T, U>, Box<dyn Error>>
    where T: for <'de> Deserialize<'de> + Ord + Eq + Hash, U: for <'de> Deserialize<'de>{
        let mut access_file = File::open(file)?;
        let mut string_buffer = Vec::new();
        access_file.read_to_end(&mut string_buffer)?;
        let deserialise:BTreeMap<T, U> = bincode::deserialize(&string_buffer)?;
        Ok(deserialise)
    }
}