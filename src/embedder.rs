use crate::models::{BatchEmbededRequest, BatchEmbededResponse, Content, GeminiRequest, Part};
pub async fn fetch_embedding(
    texts: Vec<String>,
    api_key: &str,
) -> Result<Vec<Vec<f32>>, reqwest::Error> {
    let mut batch_request = BatchEmbededRequest { requests: vec![] };
    for text in texts {
        let part = Part {
            text: text.to_string(),
        };
        let content = Content { parts: vec![part] };
        let request = GeminiRequest {
            content,
            model: "models/gemini-embedding-2".to_string(),
        };
        batch_request.requests.push(request);
    }
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-2:batchEmbedContents";
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .json(&batch_request)
        .send()
        .await?;
    let parsed: BatchEmbededResponse = response.json().await?;
    Ok(parsed.embeddings.into_iter().map(|e| e.values).collect())
}
