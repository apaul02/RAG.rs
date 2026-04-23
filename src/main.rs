pub mod chunker;
pub mod embedder;
pub mod models;
pub mod search;

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
    }
    let query = "What happens if a network partition occurs?";
    let query_res = crate::embedder::fetch_embedding(vec![query.to_string()], &api_key)
        .await
        .expect("Failed to embed query");
    let query_vector = &query_res[0];

    let top_results = crate::search::search(query_vector, &chunker.finished_chunk, 2);
    println!("\n====SEARCH RESULTS====A");
    for (i, chunk) in top_results.iter().enumerate() {
        println!("\nResult {}: \n{}", i + 1, chunk.text);
    }
}
