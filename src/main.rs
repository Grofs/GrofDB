mod wal;
mod memtable;

use std::{collections::HashMap, fs::File, hash::Hash, sync::Mutex};

fn main(){
    let mut sdb = Mutex::new(HashMap::<u64, u8>::new());
    // let _ = memtable::memtable::save(&mut sdb, 10, 0xa);
    // let _ = memtable::memtable::remove(&mut sdb, 10);
    // let _ = memtable::memtable::update(&mut sdb, 10, 0xb);
    // let _ = memtable::memtable::core_update(&mut sdb, 10, 0xb);
    // let _ = memtable::memtable::find(&mut sdb, 10);

    let F = File::create("wal.md").unwrap();
    let ahead = wal::wal::WAL{file:F, file_name:"wal.md"};
    let alog = ahead.new().write(&mut sdb, 10, 0xa);

    let map: HashMap<u64, u8> = bincode::deserialize(&alog).unwrap();
    let resv = map.iter().take(1);
    print!("{:?}", resv);
}