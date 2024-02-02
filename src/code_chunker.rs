use std::fs::File;
use std::io::{self, BufRead};
use walkdir::WalkDir;



#[derive(Debug)]
pub struct CodeChunk {
    pub filename: String,
    pub start_line: usize,
    pub end_line: usize,
    pub chunk: String,
}

impl CodeChunk {
    pub fn new(filename: &str, start_line: usize, end_line: usize, chunk: &str) -> CodeChunk {
        CodeChunk {
            filename: filename.to_string(),
            start_line,
            end_line,
            chunk: chunk.to_string(),
        }
    }
}

pub fn process_folder(folder_path: &str) {
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let filename = entry.file_name().to_string_lossy().to_string();
            let file_path = entry.path().to_str().unwrap();
            
            // Find matching lines using Ripgrep
            let rg_output = std::process::Command::new("rg")
                .arg("^}")
                .arg(file_path)
                .output();

            match rg_output {
                Ok(output) => {
                    if output.status.success() {
                        // Process Ripgrep output and cut the file into chunks
                        let stdout_str = String::from_utf8_lossy(&output.stdout);
                        let mut start_line = 0;

                        for (end_line, _) in stdout_str.lines().enumerate() {
                            let chunk = get_file_chunk(file_path, start_line, end_line + 1);
                            if !chunk.chunk.is_empty() {
                                // Do something with the code chunk, like printing it
                                println!("{:?}", chunk);
                            }
                            start_line = end_line + 1;
                        }
                    } else {
                        eprintln!("Ripgrep failed: {:?}", output.stderr);
                    }
                }
                Err(err) => eprintln!("Error running Ripgrep: {}", err),
            }
        }
    }
}


pub fn get_file_chunk(filename: &str, start_line: usize, end_line: usize) -> CodeChunk {
    let file = File::open(filename).expect("Unable to open file");
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|line| line.unwrap()).collect();
    let chunk_lines: Vec<String> = lines[start_line..end_line].to_vec();
    let chunk = chunk_lines.join("\n");

    CodeChunk::new(filename, start_line, end_line, &chunk)
}

