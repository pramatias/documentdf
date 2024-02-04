pub fn process_file(file_path: &str, _filename: &str) -> Vec<usize> {
    let pattern = "^}";
    let lines = get_fileline_numbers(file_path, pattern);
    filter_line_intervals(lines)
}

fn filter_line_intervals(lines: Vec<usize>) -> Vec<usize> {
    let mut filtered_lines = lines.clone();
    let mut is_changed = true;

    while is_changed {
        is_changed = false;

        let mut i = 0;
        while i < filtered_lines.len() {
            let current_line = filtered_lines[i];

            if i + 2 < filtered_lines.len() {
                let next_line = filtered_lines[i + 2];
                if next_line - current_line <= 30 {
                    // Skip the second line if the interval is less than 50
                    filtered_lines.remove(i + 1);
                    is_changed = true;
                } else if next_line - current_line <= 50 {
                    // Skip the second line if the interval is less than 50
                    filtered_lines.remove(i + 1);
                    is_changed = true;
                } else {
                    i += 1;
                }
            } else {
                // No next line, just move to the next line
                i += 1;
            }
        }
    }

    // Print the filtered lines
    println!("Final Filtered Lines: {:?}", filtered_lines);

    filtered_lines
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

fn get_fileline_numbers(file_path: &str, pattern: &str) -> Vec<usize> {
    // Find matching lines using modified run_ripgrep
    match run_ripgrep(file_path, pattern) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("Error running Ripgrep: {}", err);
            Vec::new()
        }
    }
}

