fn main() {
    let data=r#"data: {"id":"ecb276d6-c991-43c5-94b8-d8a1937e5389","object":"chat.completion.chunk","created":1744432060,"model":"deepseek-chat","system_fingerprint":"fp_3d5141a69a_prod0225","choices":[{"index":0,"delta":{"content":" today"},"logprobs":null,"finish_reason":null}]}"#;
    let text = data.strip_prefix("data: ").unwrap_or("");
            let json = serde_json::from_str::<serde_json::Value>(text).unwrap();
            if let Some(choices) = json["choices"].as_array() {
                for choice in choices {
                    if let Some(delta) = choice["delta"].as_object() {
                        if let Some(content) = delta["content"].as_str() {
                            println!("Content: {}", content);
                        } else {
                            println!("No content found in delta");
                        }
                    } else {
                        println!("No delta found in choice");
                    }
                }
            } else {
                println!("No choices found in JSON: {}", text);
            }
}