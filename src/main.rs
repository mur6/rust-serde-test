use std::env;
use std::process;

fn main() {
    let api_key = match env::var("API_KEY") {
        Ok(val) => val,
        Err(err) => {
            println!("API_KEY: {}", err);
            process::exit(1);
        }
    };
    //println!("Hello, world={}", api_key);
}
