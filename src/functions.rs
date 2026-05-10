use crate::storage;

pub fn write(key : String , val : String){
    storage::MAIN_STORAGE.lock().unwrap()
    .insert(key,val);
}

pub fn read(key : String) -> Option<String> {
    match storage::MAIN_STORAGE.lock().unwrap().get(&key){
        Some(val) => return Some(val.to_string()), //Might cause Memory issues later .... need to check later
        None => None, //Change later 
    }

}