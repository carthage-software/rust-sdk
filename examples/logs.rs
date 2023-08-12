use carthage_rust_sdk::types::LogManagementGetLogCollectionLevelsItem;
use carthage_rust_sdk::Client;
use reqwest;

#[tokio::main]
async fn main() {
    let reqwest_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let client = Client::new_with_client("https://localhost", reqwest_client);

    let levels: Vec<LogManagementGetLogCollectionLevelsItem> =
        vec![100.try_into().unwrap(), 200.try_into().unwrap()];
    let log_collection_result = client
        .log_management_get_log_collection(
            None,
            None,
            Some(2000),    // Items Per Page
            Some(&levels), // Levels
            None,
            None,
            None,
            None,
        )
        .await;

    match log_collection_result {
        Ok(log_collection) => {
            for log in &log_collection.items {
                let level = log.level.name.to_string();
                let template = log.template.to_string();
                println!(
                    "{} -> {}...",
                    level,
                    &template[0..std::cmp::min(template.len(), 50)]
                );
            }
        }
        Err(e) => println!("error: {}", e),
    }
}
