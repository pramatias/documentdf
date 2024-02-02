use std::fs::File;
use std::io::{self, BufRead};
use walkdir::WalkDir;
use std::path::Path;

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
            let relative_path = entry.path().strip_prefix(folder_path).unwrap();
            let filename = relative_path.to_string_lossy().to_string();
            let file_path = entry.path().to_str().unwrap();

            // Find matching lines using modified run_ripgrep
            let lines_to_chunk = run_ripgrep(file_path, "^}");

            match lines_to_chunk {
                Ok(lines) => {
                    // Process the lines and cut the file into chunks
                    let mut start_line = 0;

                    for end_line in lines {
                        println!("{}", &end_line);
                        let chunk = get_file_chunk(file_path, &filename, start_line, end_line + 1);
                        if !chunk.chunk.is_empty() {
                            // Do something with the code chunk, like printing it
                            //println!("{:?}", chunk);
                        }
                        start_line = end_line + 1;
                    }
                }
                Err(err) => eprintln!("Error running Ripgrep: {}", err),
            }
        }
    }
}

fn run_ripgrep(file_path: &str, pattern: &str) -> Result<Vec<usize>, String> {
    // Run Ripgrep and capture its output
    let rg_output = std::process::Command::new("rg")
        .arg(file_path)
        .arg("-e")
        .arg(pattern)
        .arg("--line-number") // Include line numbers in the output
        .output();

    match rg_output {
        Ok(output) => {
            if output.status.success() {
                // Parse Ripgrep output and extract line numbers
                let stdout_str = String::from_utf8_lossy(&output.stdout);

                // Print the entire Ripgrep output for debugging
                println!("Ripgrep output: {:?}", stdout_str);

                let lines: Vec<usize> = stdout_str
                    .lines()
                    .filter_map(|line| line.split(':').next().and_then(|num| num.parse().ok()))
                    .map(|num: usize| num + 1) // Add 1 to get the next line
                    .collect();

                Ok(lines)
            } else {
                // Return an error with Ripgrep's stderr
                Err(format!("Ripgrep failed: {:?}", output.stderr))
            }
        }
        Err(err) => {
            // Return an error with the description of the process creation failure
            Err(format!("Error running Ripgrep: {}", err))
        }
    }
}


pub fn get_file_chunk(file_path: &str, filename: &str, start_line: usize, end_line: usize) -> CodeChunk {
    let file = File::open(file_path).expect("Unable to open file");
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|line| line.unwrap()).collect();
    let chunk_lines: Vec<String> = lines[start_line..end_line].to_vec();
    let chunk = chunk_lines.join("\n");

    CodeChunk::new(filename, start_line, end_line, &chunk)
}
