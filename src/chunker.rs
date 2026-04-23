use crate::models::{Chunk, ParserState};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct ChunkContext {
    pub current_state: ParserState,
    pub current_header: String,
    pub buffer: Vec<String>,
    pub finished_chunk: Vec<Chunk>,
}

impl Default for ChunkContext {
    fn default() -> Self {
        ChunkContext::new()
    }
}
impl ChunkContext {
    pub fn new() -> Self {
        Self {
            current_state: ParserState::NormalText,
            current_header: String::new(),
            buffer: Vec::new(),
            finished_chunk: Vec::new(),
        }
    }
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
                        ParserState::NormalText => {
                            if line.starts_with("+---") {
                                self.current_state = ParserState::InsideTable;
                                self.buffer.push(line.to_string());
                            } else if line.starts_with("- ") || line.starts_with("* ") {
                                self.current_state = ParserState::InsideList;
                                self.buffer.push(line.to_string());
                            } else {
                                self.buffer.push(line.to_string());
                                let total_size: usize =
                                    self.buffer.iter().map(|w| w.chars().count()).sum();
                                if total_size > 1000 {
                                    let giant_string = self.buffer.join("\n");
                                    let chunk_with_context = format!(
                                        "Section: {}\n{}",
                                        self.current_header, giant_string
                                    );
                                    let chunk = Chunk::new(file_name.clone(), chunk_with_context);
                                    self.finished_chunk.push(chunk);
                                    self.buffer.drain(0..self.buffer.len().saturating_sub(2));
                                }
                            }
                        }
                        ParserState::InsideTable => {
                            if line.is_empty() {
                                let giant_string = self.buffer.join("\n");
                                let chunk_with_context =
                                    format!("Section: {}\n{}", self.current_header, giant_string);
                                let chunk = Chunk::new(file_name.clone(), chunk_with_context);
                                self.finished_chunk.push(chunk);
                                self.buffer.clear();
                                self.current_state = ParserState::NormalText;
                            } else {
                                self.buffer.push(line.to_string());
                            }
                        }
                        ParserState::InsideList => {
                            if line.is_empty() {
                                let giant_string = self.buffer.join("\n");
                                let chunk_with_context =
                                    format!("Section: {}\n{}", self.current_header, giant_string);
                                let chunk = Chunk::new(file_name.clone(), chunk_with_context);
                                self.finished_chunk.push(chunk);
                                self.buffer.clear();
                                self.current_state = ParserState::NormalText;
                            } else {
                                self.buffer.push(line.to_string());
                            }
                        }
                    }
                }
                if !self.buffer.is_empty() {
                    let giant_string = self.buffer.join("\n");
                    let chunk_with_context =
                        format!("Section: {}\n{}", self.current_header, giant_string);
                    let chunk = Chunk::new(file_name.clone(), chunk_with_context);
                    self.finished_chunk.push(chunk);
                    self.buffer.clear();
                }
            }
        }
    }
}
