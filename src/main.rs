pub mod chunker;
pub mod embedder;
pub mod generator;
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

    let question = "what is CAP";
    let query_res = crate::embedder::fetch_embedding(vec![question.to_string()], &api_key)
        .await
        .expect("Failed to embed query");
    let query_vector = &query_res[0];

    let top_results = crate::search::search(query_vector, &chunker.finished_chunk, 2);
    let mut all_results = String::new();
    for result in top_results.iter() {
        all_results.push_str(result.text.as_str());
    }
    let prompt = format!(
        "You are a helpful assistant. Answer the user's question using ONLY the context provided below \n\n {}\n\n QUESTION:\n{}",
        all_results, question
    );

    let final_answer = crate::generator::generate_answer(&prompt, &api_key)
        .await
        .expect("Failed to generate response");
    println!("\n====THE AI ANSWER====\n");
    println!("{}", final_answer);
}
