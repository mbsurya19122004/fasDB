mod storage;
mod functions;

fn main() {
    //USED FOR TESTING TO BE REMOVED IN FUTURE
    functions::write("key1".to_string(),"val1".to_string());
    functions::write("key2".to_string(),"val1".to_string());
    functions::write("key3".to_string(),"val1".to_string());
    functions::write("key4".to_string(),"val1".to_string());    
    println!("{:?}",storage::MAIN_STORAGE.lock().unwrap());
}

