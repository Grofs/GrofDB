use std::{collections::HashMap, fs::{self, File, OpenOptions}, io::{Read, Write}, sync::Mutex};

use bincode;

use crate::memtable::memtable;

#[allow(non_camel_case_types)]
type string = String;

pub struct WAL{
    pub file: File,
    pub file_name: &'static str
}

impl WAL {
    #[allow(non_snake_case)]
    pub fn new(&self) -> WAL {
        let file = OpenOptions::new().create(true).append(true).open(self.file_name).unwrap();
        WAL {file, file_name:self.file_name}
    }

    #[allow(non_snake_case)]
    pub fn write(&self, db: &mut Mutex<HashMap<u64,u8>>, k: u64, v:u8) -> Vec<u8> {
        let mut wal_map = HashMap::<u64, u8>::new();
        let wal_file = self.new();
        let mut wal_file = File::create(wal_file.file_name).unwrap();

        wal_map.insert(k, v);
        let serialised = bincode::serialize(&wal_map).expect("serialisation failed");
        wal_file.write(&serialised).expect("write error");
        memtable::save(db, k, v).expect("cannot save to memtable");
        serialised
    }

    #[allow(non_snake_case)]
    pub fn replay(&self, db: &mut Mutex<HashMap<u64, u8>>, data: &[u8])->Result<(), bincode::Error>{
        let deserialised: HashMap<u64, u8> = bincode::deserialize(data)?;
        let mut db_lock = db.lock().expect("failed to lock mutex");
        db_lock.extend(deserialised);
        Ok(())
    }

    pub fn wal(&self){
        let mut walfile = File::open(self.file_name).expect("cannot open file");
        let mut waloutput = string::new();
        walfile.read_to_string(&mut waloutput).expect("reading error");
        for line in waloutput.lines(){
            let line = line.trim();
            print!("{}\n", line);
        }
    }

    pub fn delete(&self){
        fs::remove_file(self.file_name).expect("error removing file");
    }
}