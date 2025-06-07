use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_key: String,
    pub base_url: String,
}

impl Config {
    pub fn new(app_key: &str,base_url: &str) -> Self {
        Config {
            app_key: app_key.to_string(),
            base_url: base_url.to_string(),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}