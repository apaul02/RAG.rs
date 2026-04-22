use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use uuid::Uuid;

pub enum ParserState {
    NormalText,
    InsideTable,
    InsideList,
}

pub struct Chunk {
    chunk_id: String,
    file_name: String,
    text: String,
}

impl Chunk {
    pub fn new(file_name: String, text: String) -> Self {
        Self {
            chunk_id: Uuid::new_v4().to_string(),
            file_name,
            text,
        }
    }
}

pub struct ChunkContext {
    current_state: ParserState,
    current_header: String,
    buffer: Vec<String>,
    finished_chunk: Vec<Chunk>,
}

impl ChunkContext {
    pub fn process_file<T: AsRef<Path>>(&mut self, file_name: T) {
        let file_name = file_name.as_ref().to_string_lossy().into_owned();
        let file = File::open(file_name.clone());
        match file {
            Err(e) => eprintln!("Error while Opening file: {}", e),
            Ok(file) => {
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line.expect("Could not read line");
                    let line = line.trim();
                    if line.starts_with("##") {
                        if !self.buffer.is_empty() {
                            let giant_string = self.buffer.join("\n");
                            let chunk_with_context =
                                format!("Section: {}\n{}", self.current_header, giant_string);
                            let chunk = Chunk::new(file_name.clone(), chunk_with_context);
                            self.finished_chunk.push(chunk);
                            self.buffer.clear();
                        }
                        self.current_header = line.trim_start_matches("#").trim().to_string();
                        continue;
                    }
                    match self.current_state {
                        ParserState::NormalText => {}
                        ParserState::InsideTable => {}
                        ParserState::InsideList => {}
                    }
                }
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
