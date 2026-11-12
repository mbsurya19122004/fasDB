use std::collections::HashMap;
use std::sync::{LazyLock,Mutex};

pub fn write(key : String , val : String){
    MAIN_STORAGE.lock().unwrap()
    .insert(key,val);
}

pub fn read(key : String) -> Option<String> {
    match MAIN_STORAGE.lock().unwrap().get(&key){
        Some(val) => return Some(val.to_string()), //Might cause Memory issues later .... need to check later
        None => None, //Change later 
    }

}

pub static MAIN_STORAGE : LazyLock<Mutex<HashMap<String,String>>> = LazyLock::new(||{
    Mutex::new(HashMap::new())
});