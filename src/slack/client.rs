use reqwest::Client;
use std::env;
use std::error::Error;

pub struct SlackClient {
    webhook_url: String,
    client: Client,
}

impl SlackClient {
    pub fn new() -> Self {
        let webhook_url = env::var("SLACK_WEBHOOK_URL")
            .expect("SLACK_WEBHOOK_URL environment variable not set");
        let client = Client::new();
        Self { webhook_url, client }
    }

    pub async fn post_message(&self, payload: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let response = self.client
            .post(self.webhook_url.clone())
            .json(payload)
            .send()
            .await?;

        if response.status().is_success() {
            println!("Webhook sent successfully!");
        } else {
            println!("Failed to send webhook");
        }
        Ok(())
    }
}

impl Default for SlackClient {
    fn default() -> Self {
        Self::new()
    }
}
