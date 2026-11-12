mod storage;


#[tokio::main]
async fn main() {
    //USED FOR TESTING TO BE REMOVED IN FUTURE
    storage::write("key1".to_string(),"val1".to_string());
    storage::write("key2".to_string(),"val1".to_string());
    storage::write("key3".to_string(),"val1".to_string());
    storage::write("key4".to_string(),"val1".to_string());  
    let x = storage::read("key$".to_string());  
    let y = storage::read("key4".to_string());

    println!("{:?}",storage::MAIN_STORAGE.lock().unwrap());
    match x{
        Some(val) => println!("{}",val),
        None => println!("NOT Found"),
    }
    match y{
        Some(val) => println!("{}",val),
        None => println!("NOT Found"),
    }    
}

