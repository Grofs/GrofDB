use std::{collections::{BTreeMap, HashMap}, sync::Mutex};

use crate::memtable::memtable;

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
    pub fn new(val:Vec<U>){
        let stable_name = "sstable_";
        let mut sdb = Mutex::new(HashMap::<T, U>::new());
        let mtable_dump = memtable::dump(&mut sdb, &stable_name, val);
    }
}