use std::fs::File;
use std::fs;
use std::io::{Write, Read};
use serde_json;

use crate::code_chunk::CodeChunk;
use crate::code_chunker::process_folder;

pub fn read_chunks_from_file(file_path: &str) -> Result<Vec<CodeChunk>, Box<dyn std::error::Error>> {
    // Open the file for reading
    let mut file = File::open(file_path)?;

    // Read the file contents into a string
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    // Deserialize the JSON string into a vector of CodeChunk
    let chunks: Vec<CodeChunk> = serde_json::from_str(&json_string)?;

    Ok(chunks)
}

pub fn create_json_source(folder_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let chunks = process_folder(folder_path);
    let file_path = "./doc/source.json";
    write_chunks_to_file(&chunks, file_path)?;

    // Read the chunks from the JSON file
    let read_chunks = read_chunks_from_file(file_path)?;
    // Print each read chunk
    for chunk in &read_chunks {
        println!("{}", chunk);
    }

    Ok(())
}

pub fn write_chunks_to_file(chunks: &[CodeChunk], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize the chunks to JSON format
    let json_string = serde_json::to_string(chunks)?;

    // Create the directory if it doesn't exist
    fs::create_dir_all("./doc")?;

    // Open the file for writing, creating it if it doesn't exist
    let mut file = File::create(file_path)?;

    // Write the JSON string to the file
    file.write_all(json_string.as_bytes())?;

    // Flush the buffer to ensure all data is written to the file
    file.flush()?;

    Ok(())
}
