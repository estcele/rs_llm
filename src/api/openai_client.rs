// api.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::error::Error;
use bytes::Bytes;
use std::str;
use futures::stream;

use crate::config::openai_config::{Config, ChatRequest, Message};

pub type ChatStream = Pin<Box<dyn Stream<Item = Result<ChatChunk, Box<dyn Error + Send + Sync>>> + Send>>;

#[derive(Debug, Deserialize)]
pub struct ChatChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
}

pub struct DeepSeekClient {
    client: Client,
    config: Config,
}

impl DeepSeekClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn stream_chat_completion(
        &self,
        prompt: &str,
    ) -> Result<ChatStream, Box<dyn Error + Send + Sync>> {
        let request = ChatRequest {
            model: "deepseek-chat".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            stream: true,
        };

        let response = self.client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.config.app_key))
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .json(&serde_json::json!(request))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Request failed with status: {}", response.status()).into());
        }

        let stream = response.bytes_stream()
            .flat_map(|chunk| {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(e) => return stream::iter(vec![Err(Box::new(e) as Box<dyn Error + Send + Sync>)]),
                };

                let chunk_str = match str::from_utf8(&chunk) {
                    Ok(s) => s,
                    Err(e) => return stream::iter(vec![Err(Box::new(e) as Box<dyn Error + Send + Sync>)]),
                };

                let mut results = Vec::new();
                for line in chunk_str.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if data == "[DONE]" {
                            continue;
                        }
                        match serde_json::from_str::<ChatChunk>(data) {
                            Ok(chat_chunk) => results.push(Ok(chat_chunk)),
                            Err(e) => results.push(Err(Box::new(e) as Box<dyn Error + Send + Sync>)),
                        }
                    }
                }

                stream::iter(results)
            });

        Ok(Box::pin(stream))
    }
}