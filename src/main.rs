use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;


//We will use this Struct as method to deserialize the JSON response from the API
#[derive(Deserialize, Debug)]
struct User{
    login: String ,
    id: u32
}

fn main() {
    // Your code here
    println!("hola");
}