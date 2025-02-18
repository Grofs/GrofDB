use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Read, Write}, sync::Mutex};

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
    pub fn write(&self, db: &mut Mutex<HashMap<u64,u8>>, k: u64, v:u8) -> Vec<u8> {
        let mut wal_map = HashMap::<u64, u8>::new();
        let wal_file = self.new(self.file_name);
        let mut wal_file = File::create(wal_file.file_name).unwrap();

        wal_map.insert(k, v);
        let serialised = bincode::serialize(&wal_map).expect("serialisation failed");
        wal_file.write(&serialised);
        memtable::save(db, k, v);
        serialised
    }

    #[allow(non_snake_case)]
    pub fn replay(&self, db: &mut Mutex<HashMap<u64, u8>>, data: &[u8])->Result<(), bincode::Error>{
        let deserialised: HashMap<u64, u8> = bincode::deserialize(data)?;
        let mut db_lock = db.lock().expect("failed to lock mutex");
        db_lock.extend(deserialised);
        Ok(())
    }
}