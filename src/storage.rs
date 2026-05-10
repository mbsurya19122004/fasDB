use std::collections::HashMap;
use std::sync::{LazyLock,Mutex};

pub static MAIN_STORAGE : LazyLock<Mutex<HashMap<String,String>>> = LazyLock::new(||{
    Mutex::new(HashMap::new())
});