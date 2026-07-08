mod storage;
mod functions;


fn main() {
    //USED FOR TESTING TO BE REMOVED IN FUTURE
    let body = functions::get("https://dogapi.dog/api/v2/breeds/3").await?;
    
    println!("{}",body);
    Ok(());
}

