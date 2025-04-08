use rs_llm::openai_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let openai_client = openai_client::OpenAIClient::new(
        "sk-6291976efe804526aa126002dd2b6999".to_string(),
        "https://api.deepseek.com".to_string(),
    );
    let prompt = "What is the capital of France?";
    let response = openai_client.talk(prompt).await?;
    println!("Response: {}", response);
    // let api_key = openai_client.get_api_key();
    // let base_url = openai_client.get_base_url();
    Ok(())
}
