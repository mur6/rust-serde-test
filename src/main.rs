use reqwest;
use std::env;
use std::process;

//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match env::var("API_KEY") {
        Ok(val) => val,
        Err(err) => {
            println!("API_KEY: {}", err);
            process::exit(1);
        }
    };
    //println!("Hello, world={}", api_key);
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);
    Ok(())
}
