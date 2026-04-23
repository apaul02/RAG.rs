use crate::models::Chunk;

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let a_sq: f32 = a.iter().map(|e| e * e).sum();
    let a_mag = a_sq.sqrt();
    let b_sq: f32 = b.iter().map(|e| e * e).sum();
    let b_mag = b_sq.sqrt();
    if a_mag == 0.0 || b_mag == 0.0 {
        return 0.0;
    }
    (dot_product) / (a_mag * b_mag)
}

pub fn search<'a>(query_embedding: &[f32], chunks: &'a [Chunk], top_k: usize) -> Vec<&'a Chunk> {
    let mut scored_chunks = vec![];
    for chunk in chunks {
        if let Some(emb) = &chunk.embeddings {
            let score = cosine_similarity(emb, query_embedding);
            scored_chunks.push((score, chunk));
        }
    }
    scored_chunks.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    scored_chunks
        .into_iter()
        .take(top_k)
        .map(|(_, chunk)| chunk)
        .collect()
}
