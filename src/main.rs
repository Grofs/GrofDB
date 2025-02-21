mod wal;
mod memtable;
mod sstable;
mod extension;
mod bloomfilter;

use std::{collections::HashMap, fs::File,  sync::Mutex};

fn main(){
    let mut sdb = Mutex::new(HashMap::<u64, u8>::new());
    let _ = memtable::memtable::save(&mut sdb, 10, 0xa);
    let _ = memtable::memtable::update(&mut sdb, 10, 0xb);
    let _ = memtable::memtable::core_update(&mut sdb, 10, 0xb);
    let memtable_search = memtable::memtable::find(&mut sdb, 10).unwrap();
    println!("{:?}", memtable_search);

    let walfile = File::create("wal.md").unwrap();
    let ahead = wal::wal::WAL{file:walfile, file_name:"wal.md"};
    let alog = ahead.new().write(&mut sdb, 10, 0xa);

    let map: HashMap<u64, u8> = bincode::deserialize(&alog).unwrap();
    for (_, (k, v)) in map.iter().enumerate(){
        println!("{:?} {:?}", k, v);
    }

    let _ = extension::extension::select_unique();
}