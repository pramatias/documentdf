use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChunk {
    pub id: usize,
    pub filename: String,
    pub start_line: usize,
    pub end_line: usize,
    pub documented: bool,
    pub chunks: Vec<String>,
}

impl CodeChunk {
    pub fn new(id: usize, filename: &str, start_line: usize, end_line: usize, chunks: Vec<String>, documented: bool) -> CodeChunk {
        CodeChunk {
            id,
            filename: filename.to_string(),
            start_line,
            end_line,
            chunks,
            documented,
        }
    }
}

impl fmt::Display for CodeChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {}\nFile: {}\nDocumented: {}\nStart Line: {}\nEnd Line: {}\nChunks:",
            self.id, self.filename, self.documented, self.start_line, self.end_line
        )?;

        for chunk in &self.chunks {
            writeln!(f, "{}", chunk)?;
        }

        Ok(())
    }
}
