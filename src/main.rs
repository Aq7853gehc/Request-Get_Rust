use std::io::Read;



fn main()  {
    let mut res = reqwest::blocking::get("https://dummyjson.com/users/1").unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}",res.status());
    // println!("Header: {:#?}",res.headers());
    println!("Body:\n{}",body);
}
