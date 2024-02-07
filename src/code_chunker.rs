use std::fs::File;
use std::io::{self, BufRead};
use walkdir::WalkDir;
use crate::ripgrep_lines::process_file;
use std::fmt;

#[derive(Debug)]
pub struct CodeChunk {
    pub filename: String,
    pub start_line: usize,
    pub end_line: usize,
    pub chunks: Vec<String>,
}

impl CodeChunk {
    pub fn new(filename: &str, start_line: usize, end_line: usize, chunks: Vec<String>) -> CodeChunk {
        CodeChunk {
            filename: filename.to_string(),
            start_line,
            end_line,
            chunks,
        }
    }
}

pub fn process_folder(folder_path: &str) {
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let relative_path = entry.path().strip_prefix(folder_path).unwrap();
            let filename = relative_path.to_string_lossy().to_string();
            let file_path = entry.path().to_str().unwrap();

            let filtered_lines = process_file(file_path, &filename);
            file_to_chunks(file_path, &filename, filtered_lines);
        }
    }
}

fn file_to_chunks(file_path: &str, filename: &str, lines: Vec<usize>) {
    // Print the filtered lines
    //println!("Ripgrep filtered Lines: {:?}", lines);

    // Process the lines and cut the file into chunks
    let mut start_line = 0;

    for end_line in lines {
        // Ensure end_line does not exceed the total number of lines in the file
        let total_lines = count_lines(file_path);
        let adjusted_end_line = if end_line > 0 { end_line - 1 } else { 0 };
        let end_line = adjusted_end_line.min(total_lines);

        let chunk = create_code_chunk(file_path, filename, start_line, end_line);
        if !chunk.chunks.is_empty() {
            // Use fmt::Display to format the CodeChunk and print the result
            let formatted_chunk = format!("{}", chunk);
            println!("{}", formatted_chunk);
        }
        start_line = end_line;
    }
}

fn count_lines(file_path: &str) -> usize {
    let file = std::fs::File::open(file_path).expect("Unable to open file");
    let lines = io::BufReader::new(file).lines().count();
    lines
}

fn create_code_chunk(file_path: &str, _relative_path: &str, start_line: usize, end_line: usize) -> CodeChunk {
    let file = File::open(file_path).expect("Unable to open file");
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|line| line.unwrap()).collect();

    // Ensure end_line does not exceed the total number of lines in the file
    let end_line = end_line.min(lines.len());

    let chunk_lines: Vec<String> = lines[start_line..end_line].to_vec();

    CodeChunk::new(&file_path, start_line, end_line, chunk_lines)
}

impl fmt::Display for CodeChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "File: {}\nStart Line: {}\nEnd Line: {}\nChunks:",
            self.filename, self.start_line, self.end_line
        )?;

        for chunk in &self.chunks {
            writeln!(f, "{}", chunk)?;
        }

        Ok(())
    }
}
