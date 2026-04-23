use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub enum ParserState {
    NormalText,
    InsideTable,
    InsideList,
}

pub struct Chunk {
    pub chunk_id: String,
    pub file_name: String,
    pub text: String,
    pub embeddings: Option<Vec<f32>>,
}

#[derive(Serialize)]
pub struct Part {
    pub text: String,
}

#[derive(Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize)]
pub struct GeminiRequest {
    pub model: String,
    pub content: Content,
}

#[derive(Serialize)]
pub struct BatchEmbededRequest {
    pub requests: Vec<GeminiRequest>,
}

#[derive(Deserialize)]
pub struct BatchEmbededResponse {
    pub embeddings: Vec<Embeddings>,
}

#[derive(Deserialize)]
pub struct Embeddings {
    pub values: Vec<f32>,
}

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub embedding: Embeddings,
}

impl Chunk {
    pub fn new(file_name: String, text: String) -> Self {
        Self {
            chunk_id: Uuid::new_v4().to_string(),
            file_name,
            text,
            embeddings: None,
        }
    }
}
