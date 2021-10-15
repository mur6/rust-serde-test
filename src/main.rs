use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::process;
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }
#[derive(Serialize, Deserialize, Debug)]
struct Info {
    title: String,
    explanation: String,
    date: String,
    url: String,
    hdurl: String,
    media_type: String,
    service_version: String,
    copyright: String,
}

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
    //api_key=DEMO_KEY
    let url = "https://api.nasa.gov/planetary/apod";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .query(&[("api_key", api_key), ("count", "1".to_string())])
        .send()
        .await?;
    //let body = reqwest::get().await?.text().await?;
    let b = resp.json::<Vec<Info>>().await?;
    println!("body = {:?}", b);
    Ok(())
}
