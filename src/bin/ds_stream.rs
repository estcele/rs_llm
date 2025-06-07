// main.rs
use rs_llm::api::openai_client::{DeepSeekClient, ChatChunk};
use rs_llm::config::openai_config::Config;
use futures::StreamExt;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::new("sk-6291976efe804526aa126002dd2b6999","https://api.deepseek.com/v1/chat/completions");
    let client = DeepSeekClient::new(config);
    
    let mut stream = client.stream_chat_completion("Hello, how are you?").await?;
    let mut stdout = io::stdout();
    
    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                if let Some(content) = chunk.choices[0].delta.content.as_ref() {
                    stdout.write_all(content.as_bytes()).await?;
                    stdout.flush().await?;
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}