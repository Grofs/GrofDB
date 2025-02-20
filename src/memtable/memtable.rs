
use std::{collections::{BTreeMap, HashMap}, error::Error, fs::{metadata, File}, hash::Hash, io::Write, sync::Mutex};
use serde::{Deserialize, Serialize};
use derive_more::derive;

#[allow(non_snake_case)]
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct DB<'a, T, U>{
    DBKey: T,
    DBval: U
}


pub fn dump<'a, T, U>(db: &mut Mutex<HashMap<DB<'a, T, U>,DB<'a, T, U>>>, sfile: &'a str, data: &'a Vec<u8>) -> Result<Option<bool>, Box<dyn Error>>
where 
    T: Eq + Ord + Serialize + serde::de::Deserialize<'a> + Hash + Display, 
    U:Eq + Ord + Serialize + serde::de::Deserialize<'a> + Display,
{
    let threshold = 1024 * 1024 as u64;
    let map = db.lock().unwrap();
    for (k,_) in map.iter(){
        match metadata(sfile){
            Ok(metadata)=>{
                let filesize = metadata.len();
                if filesize > threshold{
                    let filename = format!("sstable-{}",k);
                    let mut sstable = File::create(&filename)?;
                    let deserialised:HashMap<T, U> = bincode::deserialize(data)?;
                    let serialised = bincode::serialize(&deserialised)?;
                    sstable.write_all(&serialised).unwrap();
                }
            },
            Err(e) =>{eprintln!("error reading metadata : {}", e)}
        }
    }
    Ok(Some(true))
}

 

pub fn remove<'a, T, U>(db:&mut Mutex<HashMap<DB<'a, T, U>,DB<'a, T, U>>>, k:DB<'a, T, U>) -> Result<bool, Box<dyn Error>>
where 
    T: Eq + Hash + Ord + Serialize + serde::de::Deserialize<'a>, 
    U:Eq + Hash+  Ord + Serialize + serde::de::Deserialize<'a>,
{
    let mut db = db.lock().unwrap();
    if !db.contains_key(&k){return Ok(false)}
    let remove = db.remove(&k);
    let removed = remove.is_some();
    Ok(removed)
}

pub fn update<'a,T, U>(db: &mut Mutex<HashMap<DB<'a, T, U>,DB<'a, T, U>>>, k:DB<'a, T, U>, v:DB<'a, T,U>)
where 
    T: Eq + Hash + Ord + Serialize + serde::de::Deserialize<'a>, 
    U:Eq + Hash+  Ord + Serialize + serde::de::Deserialize<'a>,
{
    let mut db = db.lock().unwrap();
    if !db.contains_key(&k){print!("invalid key");}
    let _ = db.get_mut(&k).unwrap();
}

pub fn core_update<'a,T, U>(db: &mut Mutex<HashMap<DB<'a, T, U>,DB<'a, T, U>>>, k:DB<'a, T, U>, v:DB<'a, T,U>)
where 
    T: Eq + Hash + Ord + Serialize + serde::de::Deserialize<'a>, 
    U:Eq + Hash+  Ord + Serialize + serde::de::Deserialize<'a>,
{
    let mut db = db.lock().unwrap();
    if !db.contains_key(&k){print!("{}", "invalid key");}
    let op_elem = db.get_mut(&k);
    let elem = op_elem.unwrap();
    (*elem) = v;
}

pub fn find<'a, T, U>(db: &mut Mutex<HashMap<DB<'a, T, U>,DB<'a, T, U>>>, k:DB<'a, T, U>) -> Result<U, Box<dyn Error>>
where 
    T: Eq + Hash + Ord + Serialize + serde::de::Deserialize<'a>, 
    U:Eq + Hash+  Ord + Serialize + serde::de::Deserialize<'a> + Copy,
{
    let db = db.lock().unwrap();
    if !db.contains_key(&k) || db.is_empty(){return Err("key does not exist".into());}
    let someval = db.get(&k);
    if someval.is_some(){Ok(someval.unwrap().DBval)}else{
        Err("key does not exist".into())
    }
}
