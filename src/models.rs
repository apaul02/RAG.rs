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
