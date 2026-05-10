use crate::storage;

pub fn write(key : String , val : String){
    storage::MAIN_STORAGE.lock().unwrap()
    .insert(key,val);
}