mod storage;
mod functions;

fn main() {
    //USED FOR TESTING TO BE REMOVED IN FUTURE
    functions::write("key1".to_string(),"val1".to_string());
    functions::write("key2".to_string(),"val1".to_string());
    functions::write("key3".to_string(),"val1".to_string());
    functions::write("key4".to_string(),"val1".to_string());  
    let x = functions::read("key$".to_string());  
    let y = functions::read("key4".to_string());

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

