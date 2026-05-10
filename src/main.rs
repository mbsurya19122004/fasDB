mod storage;

fn main() {
    //USED FOR TESTING TO BE REMOVED IN FUTURE
    {
        let mut map = storage::MAIN_STORAGE.lock().unwrap();
        map.insert("key".to_string(),"val".to_string());
        map.insert("key2".to_string(),"val2".to_string());
        map.insert("key3".to_string(),"val3".to_string());
    }
    
    println!("{:?}",storage::MAIN_STORAGE.lock().unwrap());
}

