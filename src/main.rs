pub mod chunker;
pub mod embedder;
pub mod models;

use crate::chunker::ChunkContext;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY").expect("No api key found");
    let mut chunker = ChunkContext::new();
    chunker.process_file("distributed.txt");
    let mut text = vec![];
    for l in &mut chunker.finished_chunk {
        text.push(l.text.clone());
    }
    let res = crate::embedder::fetch_embedding(text, &api_key)
        .await
        .expect("Failed to do embeddings");

    for (chunk, vector) in chunker.finished_chunk.iter_mut().zip(res) {
        chunk.embeddings = Some(vector);
        println!("{:?}", chunk.embeddings);
    }
}
