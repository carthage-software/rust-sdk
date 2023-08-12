use carthage_rust_sdk::Client;
use reqwest;

#[tokio::main]
async fn main() {
    let reqwest_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let client = Client::new_with_client("https://localhost", reqwest_client);

    match client.ping().await {
        Ok(resource) => {
            println!("Server is up and running!");
            println!("Server Time: {}", resource.time);
            println!("Quote: {}", resource.quote.clone().unwrap_or_default());
        }
        Err(e) => println!("error: {}", e),
    }
}
