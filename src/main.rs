use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/uuid")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("Random UUID: {:#?}", resp);
    Ok(())
}
