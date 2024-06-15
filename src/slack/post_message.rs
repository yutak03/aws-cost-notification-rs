use reqwest::Client;
use std::env;
use std::error::Error;

pub async fn post_slack(payload: &serde_json::Value) -> Result<(), Box<dyn Error>> {
    let url =
        env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL environment variable not set");

    let client = Client::new();
    let response = client.post(url).json(payload).send().await?;

    if response.status().is_success() {
        println!("Webhook sent successfully!");
    } else {
        println!("Failed to send webhook");
    }

    Ok(())
}
