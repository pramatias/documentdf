use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use std::collections::HashMap;

use super::code_chunk::CodeChunk;

#[derive(Deserialize)]
pub struct CodeChunksJson {
    chunks: Vec<CodeChunk>,
}

impl CodeChunksJson {
    pub fn from_file(filename: &str) -> Result<CodeChunksJson, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut code_chunks: Vec<CodeChunk> = serde_json::from_str(&contents)?;

        code_chunks.sort_by(|a, b| {
            let filename_cmp = a.filename.cmp(&b.filename);
            if filename_cmp == std::cmp::Ordering::Equal {
                a.id.cmp(&b.id)
            } else {
                filename_cmp
            }
        });

        let result = CodeChunksJson { chunks: code_chunks };

        Ok(result)
    }

    pub fn sort(&mut self) {
        self.chunks.sort_by(|a, b| {
            let filename_cmp = a.filename.cmp(&b.filename);
            if filename_cmp == std::cmp::Ordering::Equal {
                a.id.cmp(&b.id)
            } else {
                filename_cmp
            }
        });
    }

    pub fn summarize_with_chunk_count(code_chunks: &[CodeChunk]) -> HashMap<String, usize> {
        let unique_filenames: Vec<String> = code_chunks
            .iter()
            .map(|chunk| chunk.filename.clone())
            .collect::<Vec<String>>();

        let mut file_chunk_counts = HashMap::new();

        for filename in unique_filenames {
            let chunks_for_file: Vec<&CodeChunk> = code_chunks
                .iter()
                .filter(|chunk| chunk.filename == filename)
                .collect();

            file_chunk_counts.insert(filename, chunks_for_file.len());
        }

        file_chunk_counts
    }

    pub fn display_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let code_chunks = &self.chunks;

        let summary = Self::summarize_with_chunk_count(code_chunks);
        println!("Total chunks: {}", code_chunks.len());

        for (filename, chunk_count) in summary {
            let documented_chunks = code_chunks
                .iter()
                .filter(|chunk| chunk.is_documented())
                .filter(|chunk| chunk.filename == filename)
                .count();

            let undocumented_chunks = chunk_count - documented_chunks;

            println!(
                "File: {}, Total chunks: {}, Undocumented: {}, Documented: {}",
                filename, chunk_count, undocumented_chunks, documented_chunks
            );
        }

        Ok(())
    }
}
