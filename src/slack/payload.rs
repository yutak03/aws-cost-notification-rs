use serde_json::json;

pub struct SlackPayload {
    pub payload: serde_json::Value,
}

impl SlackPayload {
    pub fn create_payload(message: String) -> Self {
        Self { payload: json!({"text": message}) }
    }
}