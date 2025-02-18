use std::{collections::HashMap, error::Error, sync::Mutex};



type DB = Mutex<HashMap<u64, u8>>;

fn save(db:&mut DB, k:u64, v:u8) -> Result<bool, Box<dyn Error>>{
    let mut db = db.lock().unwrap();
    let savenew = db.insert(k, v);
    let saved = savenew.is_some();
    Ok(saved)
}

fn remove(db:&mut DB, k:u64) -> Result<bool, Box<dyn Error>>{
    let mut db = db.lock().unwrap();
    if !db.contains_key(&k){println!("invalid key")}
    let remove = db.remove(&k);
    let removed = remove.is_none();
    Ok(removed)
}

fn update(db: &mut DB, k:u64, v:u8){
    let mut db = db.lock().unwrap();
    if !db.contains_key(&k){print!("invalid key");}
    *db.get_mut(&k).unwrap() += v
}

fn core_update(db: &mut DB, k: u64, v: u8){
    let mut db = db.lock().unwrap();
    if !db.contains_key(&k){print!("{}", "invalid key");}
    let op_elem = db.get_mut(&k);
    let elem = op_elem.unwrap();
    (*elem) = v;
}

fn find(db: &DB, k:u64) -> Result<u8, Box<dyn Error>>{
    let db = db.lock().unwrap();
    if !db.contains_key(&k) || db.is_empty(){return Err("key does not exist".into());}
    if let Some(&v) = db.get(&k){
        Ok(v)
    } else{
        Err("key does not exist".into())
    }
}

fn wal(){}

fn main(){
    let mut sdb = Mutex::new(HashMap::<u64, u8>::new());
    let _ = save(&mut sdb, 10, 0xa);
    let _ = remove(&mut sdb, 10);
    let _ = update(&mut sdb, 10, 0xb);
    let _ = core_update(&mut sdb, 10, 0xb);
    let _ = find(&mut sdb, 10);
    let _= wal();
}