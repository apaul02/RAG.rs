use crate::models::{Content, GenerateResponse, LLMRequest, Part};

pub async fn generate_answer(
    prompt: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let part = Part {
        text: prompt.to_string(),
    };

    let content = Content { parts: vec![part] };

    let llm_request = LLMRequest {
        contents: vec![content],
    };

    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent";
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .json(&llm_request)
        .send()
        .await?;

    let parsed: GenerateResponse = response.json().await?;
    Ok(parsed
        .candidates
        .first()
        .ok_or("No candidates")?
        .content
        .parts
        .first()
        .ok_or("No parts")?
        .text
        .clone())
}
