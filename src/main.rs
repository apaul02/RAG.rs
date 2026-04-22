pub mod chunker;
pub mod models;

use crate::chunker::ChunkContext;

fn main() {
    let mut chunker = ChunkContext::new();
    chunker.process_file("distributed.txt");
    for l in chunker.finished_chunk {
        println!("{}", l.text);
    }
}
