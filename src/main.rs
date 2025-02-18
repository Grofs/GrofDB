mod wal;
mod memtable;

use std::{collections::HashMap,sync::Mutex};

fn main(){
    let mut sdb = Mutex::new(HashMap::<u64, u8>::new());
    let _ = memtable::memtable::save(&mut sdb, 10, 0xa);
    let _ = memtable::memtable::remove(&mut sdb, 10);
    let _ = memtable::memtable::update(&mut sdb, 10, 0xb);
    let _ = memtable::memtable::core_update(&mut sdb, 10, 0xb);
    let _ = memtable::memtable::find(&mut sdb, 10);
    let _= wal::wal::wal();
}