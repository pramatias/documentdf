use std::fs::File;
use std::io::{self, BufRead};
use walkdir::WalkDir;

use crate::ripgrep_lines::process_file;

use core::code_chunk::{CodeChunk};

pub fn process_folder(folder_path: &str) -> Vec<CodeChunk> {
    let mut all_chunks = Vec::new(); // Vector to accumulate all chunks

    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let relative_path = entry.path().strip_prefix(folder_path).unwrap();
            let filename = relative_path.to_string_lossy().to_string();
            let file_path = entry.path().to_str().unwrap();

            let filtered_lines = process_file(file_path, &filename);
            let chunks = file_to_chunks(file_path, &filename, filtered_lines);

            // Extend the all_chunks vector with the chunks for this file
            all_chunks.extend(chunks);
        }
    }

    // Return the accumulated vector of chunks
    all_chunks
}

pub fn file_to_chunks(file_path: &str, filename: &str, lines: Vec<usize>) -> Vec<CodeChunk> {
    // Print the filtered lines
    //println!("Ripgrep filtered Lines: {:?}", lines);

    // Process the lines and cut the file into chunks
    let mut start_line = 0;
    let mut id = 0;  // Initialize id counter
    let mut chunks = Vec::new(); // Initialize vector to store chunks

    for end_line in lines {
        // Ensure end_line does not exceed the total number of lines in the file
        let total_lines = count_lines(file_path);
        let adjusted_end_line = if end_line > 0 { end_line - 1 } else { 0 };
        let end_line = adjusted_end_line.min(total_lines);

        let chunk = create_code_chunk(file_path, filename, id, start_line, end_line);

        if !chunk.chunks.is_empty() {
            // Push the chunk to the vector
            chunks.push(chunk);
        }
        start_line = end_line;

        // Increment id for the next chunk
        id += 1;
    }

    // Return the vector of chunks
    chunks
}

fn count_lines(file_path: &str) -> usize {
    let file = std::fs::File::open(file_path).expect("Unable to open file");
    let lines = io::BufReader::new(file).lines().count();
    lines
}

fn create_code_chunk(file_path: &str, _relative_path: &str, id: usize, start_line: usize, end_line: usize) -> CodeChunk {
    let file = File::open(file_path).expect("Unable to open file");
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|line| line.unwrap()).collect();

    // Ensure end_line does not exceed the total number of lines in the file
    let end_line = end_line.min(lines.len());

    let chunk_lines: Vec<String> = lines[start_line..end_line].to_vec();

    CodeChunk::new(id, file_path, start_line, end_line, chunk_lines, false)
}

