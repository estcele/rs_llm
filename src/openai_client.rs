
pub struct OpenAIClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}
/// https://api.deepseek.com/chat/completions
/// sk-6291976efe804526aa126002dd2b6999
/// deepseek-chat
impl OpenAIClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        OpenAIClient { 
            api_key, 
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }

    pub fn get_base_url(&self) -> &String {
        &self.base_url
    }

    pub async fn talk(&self, prompt: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/chat/completions", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", self.api_key).parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip".parse().unwrap());
        headers.insert("User-Agent", "DeepSeek".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());

        let mut response = self.client.post(&url)
            .headers(headers)
            .json(&serde_json::json!({
            "model": "deepseek-chat",
            "stream": true,
            "temperature": 0.7,
            "messages": [
                {
                "role": "user",
                "content": prompt
                }
            ]
            }))
            .send()
            .await?;
        if response.status() != reqwest::StatusCode::OK {
            return Err(response.error_for_status().unwrap_err());
        }
        while let Some(chunk) = response.chunk().await? {
            println!("{}", std::str::from_utf8(&chunk).unwrap());
            if let Ok(text) = std::str::from_utf8(&chunk) {
                let text = text.strip_prefix("data: ").unwrap_or(text);
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                    if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                        println!("{}", content);
                    }else {
                        println!("No content found in JSON: {}", text);
                    }
                }else {
                    println!("Failed to parse JSON: {}", text);
                }
            }else {
                println!("Failed to convert chunk to string");
            }
        }

        Ok("Stream completed".to_string())
    }
}