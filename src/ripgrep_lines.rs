use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct SortedLineNumbers {
    lines: Vec<usize>,
}

pub fn process_file(file_path: &str, _filename: &str) -> Vec<usize> {
    let first_pattern = "^}";
    let second_pattern = "^    }$";

    // Get lines matching the "^}" pattern
    let first_ripgrep_output_lines = get_fileline_numbers(file_path, first_pattern);

    // Print lines before filtering for the first pattern
    println!("Lines before filtering ({}): {:?}", first_pattern, first_ripgrep_output_lines);

    // Sort and filter the lines
    let mut first_pattern_lines = SortedLineNumbers::from_vec(first_ripgrep_output_lines);
    let first_pattern_filtered_lines = first_pattern_lines.filter_line_intervals();

    // Print lines after filtering for the first pattern
    println!("Lines after filtering ({}): {:?}\n", first_pattern, first_pattern_filtered_lines);

    // Get lines matching the "^    }$" pattern
    let second_ripgrep_output_lines = get_fileline_numbers(file_path, second_pattern);

    // Print lines before filtering for the second pattern
    println!("Lines before filtering ({}): {:?}\n", second_pattern, second_ripgrep_output_lines);

    // Sort and filter the lines
    let mut second_pattern_lines = SortedLineNumbers::from_vec(second_ripgrep_output_lines);
    let second_pattern_filtered_lines = second_pattern_lines.filter_line_intervals();

    // Print lines after filtering for the second pattern
    println!("Lines after filtering ({}): {:?}\n", second_pattern, second_pattern_filtered_lines);

    // Merge and filter the lines from both patterns
    first_pattern_lines.merge(second_pattern_filtered_lines);
    let mut final_filtered_lines = first_pattern_lines.filter_line_intervals();
    final_filtered_lines.fill_line_gaps();

    // Perform further operations if needed
    let final_result = final_filtered_lines.to_vec();

    // Print the final result
    println!("Final Result: {:?}\n", final_result);

    final_result
}


impl SortedLineNumbers {
    pub fn new() -> Self {
        SortedLineNumbers { lines: Vec::new() }
    }

    pub fn from_vec(lines: Vec<usize>) -> Self {
        let mut sorted_lines = SortedLineNumbers::new();
        sorted_lines.lines.extend(lines);
        sorted_lines.lines.sort(); // Ensure the lines are sorted
        sorted_lines
    }

    pub fn merge(&mut self, lines: SortedLineNumbers) {
        let mut merged_lines = Vec::new();
        let mut iter_self = self.lines.iter().peekable();
        let mut iter_lines = lines.lines.iter();
        let mut current_lines = iter_lines.next();

        while let Some(&self_val) = iter_self.next() {
            if let Some(&next_self) = iter_self.peek() {
                merged_lines.push(self_val);
                merged_lines.push(*next_self);
                let diff = next_self - self_val;
                if diff > 50 {
                    // Gap greater than 50, look for corresponding values in current_lines
                    while let Some(&lines_val) = current_lines {
                        match lines_val.cmp(&self_val) {
                            Ordering::Less => {
                                // Lines value is less than self value, move to the next lines value
                                current_lines = iter_lines.next();
                            }
                            Ordering::Greater => {
                                // Found a match, insert the lines value between self_val and next_self
                                if lines_val > *next_self {
                                    // If lines_val is greater than next_self, stop the loop
                                    break;
                                }
                                merged_lines.push(lines_val);
                                current_lines = iter_lines.next();
                            }
                            Ordering::Equal => {
                                // Lines value is equal to self value, move to the next lines value
                                current_lines = iter_lines.next();
                            }
                        }
                    }
                }
            }
        }

        // Update self.lines with the merged result
        self.lines = merged_lines;
    }

    // Getter method to access the sorted lines
    pub fn get_lines(&self) -> &Vec<usize> {
        &self.lines
    }

    // Convert SortedLineNumbers to Vec<usize>
    pub fn to_vec(&self) -> Vec<usize> {
        self.lines.clone()
    }

    pub fn fill_line_gaps(&mut self) {
        let mut is_gap_greater_than_50 = true;

        while is_gap_greater_than_50 {
            let mut merged_lines = Vec::new();
            let mut iter_self = self.lines.iter().peekable();
            is_gap_greater_than_50 = false;

            while let Some(&self_val) = iter_self.next() {
                merged_lines.push(self_val);

                if let Some(&next_self) = iter_self.peek() {
                    let diff = next_self - self_val;
                    if diff > 50 {
                        // Gap greater than 50, insert a new element between self_val and next_self
                        let new_element = (self_val + next_self) / 2;
                        merged_lines.push(new_element);
                        is_gap_greater_than_50 = true;
                    }
                }
            }

            // Update self.lines with the merged result
            self.lines = merged_lines;
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
        Ok(lines) => {
            println!("Ripgrep lines: {:?}\n", lines);

            // Insert 0 at the start of the vector
            let mut lines = lines;  // Declare as mutable here
            lines.insert(0, 0);

            lines
        }
        Err(err) => {
            eprintln!("Error running Ripgrep: {}", err);
            Vec::new()
        }
    }
}

impl SortedLineNumbers {
    pub fn filter_line_intervals(&mut self) -> SortedLineNumbers {
        // Sort the lines before filtering
        self.lines.sort();

        let mut is_changed = true;

        while is_changed {
            is_changed = false;

            let mut i = 0;
            while i < self.lines.len() {
                let current_line = self.lines[i];

                if i + 2 < self.lines.len() {
                    let next_line = self.lines[i + 2];
                    if next_line - current_line <= 30 {
                        // Skip the second line if the interval is less than 50
                        self.lines.remove(i + 1);
                        is_changed = true;
                    } else if next_line - current_line <= 50 {
                        // Skip the second line if the interval is less than 50
                        self.lines.remove(i + 1);
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

        // Create a new SortedLineNumbers with the filtered lines
        SortedLineNumbers::from_vec(self.lines.clone())
    }
}
