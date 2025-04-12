use reqwest::{self, header};
use serde_json::Value;
use futures_util::StreamExt; // Import StreamExt for the `next` method

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://api.deepseek.com/chat/completions";
    let api_key = "sk-6291976efe804526aa126002dd2b6999";

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(header::ACCEPT, "application/json".parse().unwrap());
    headers.insert(header::ACCEPT_ENCODING, "gzip".parse().unwrap());
    headers.insert(header::CONNECTION, "keep-alive".parse().unwrap());

    let http_client = reqwest::Client::new();
    let response = http_client
        .post(url)
        .headers(headers)
        .json(&serde_json::json!({
            "model": "deepseek-chat",
            "stream": true,
            "temperature": 0.7,
            "messages": [
                {
                    "role": "user",
                    "content": "你好，今天的天气怎么样？",
                }
            ]
        }))
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(response.error_for_status().unwrap_err());
    }

    // 使用 bytes_stream() 方法来处理响应体,需要reqwest future stream，需要 futures_util crate
    let mut stream = response.bytes_stream();
    // stream.next() 是一个异步迭代器，返回一个 Option<Bytes>，需要使用 futures_util::StreamExt trait
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let data = match std::str::from_utf8(&chunk) {
            Ok(data) => data,
            Err(_) => continue, // 跳过无效的 UTF-8 数据
        };

        for line in data.lines() {
            if line.is_empty() || line == "data: [DONE]" {
                if line == "data: [DONE]" {
                    // println!("Stream finished");
                    break;
                }
                continue;
            }

            if let Some(text) = line.strip_prefix("data: ") {
                match serde_json::from_str::<Value>(text) {
                    Ok(json) => process_json(&json).await,
                    Err(e) => eprintln!("Error parsing JSON: {}\n{}", text, e),
                }
            }
        }
    }

    Ok(())
}

async fn process_json(json: &Value) {
    if let Some(choices) = json["choices"].as_array() {
        for choice in choices {
            if let Some(delta) = choice["delta"].as_object() {
                if let Some(content) = delta["content"].as_str() {
                    // io::stdout().write_all(content.as_bytes()).await.unwrap();
                    print!("{}", content);
                }
            }
        }
    }
}
