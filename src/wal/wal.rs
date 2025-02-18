use std::{collections::HashMap, fs::{File, OpenOptions}, io::Write, sync::Mutex};

use bincode;

use crate::memtable::memtable;

pub struct WAL{
    pub file: File,
    pub file_name: &'static str
}

impl WAL {
    #[allow(non_snake_case)]
    pub fn new(&self,Log: &'static str) -> WAL {
        let file = OpenOptions::new().create(true).append(true).open(Log).unwrap();
        WAL {file, file_name:&Log}
    }

    #[allow(non_snake_case)]
    pub fn write(&self, db: &mut Mutex<HashMap<u64,u8>>, k: &str, v:&str) {
        let mut wal_map = HashMap::<&str, &str>::new();
        let wal_file = self.new(self.file_name);
        let mut wal_file = File::create(wal_file.file_name).unwrap();

        wal_map.insert(k, v);
        let serialised = bincode::serialize(&wal_map).expect("serialisation failed");
        wal_file.write(&serialised);
        let Fku64 = k.parse::<u64>().expect("invalid u64 conversion");
        let Fku8 = v.parse::<u8>().expect("invalid value conversion to u8");
        memtable::save(db, Fku64, Fku8);
    }
}