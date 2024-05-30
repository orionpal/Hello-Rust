use std::io::BufRead;
use std::fs;
use std::io;
use std::collections::HashMap;

pub fn read_then_process(file_path: &str) {
    // 1. Try reading file
    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            // 2. Output the contents into a Vec<String> object
            let lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
            // 3. Run our function
            for line in lines {
                let _ = count_letters(&line);
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
}

pub fn read_while_processing(file_path: &str) {
    // 1. Try opening the file
    match fs::File::open(file_path) {
        Ok(file) => {
            // 2. Apply the function to each line of the file
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        let _ = count_letters(&line);
                    }
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
}
pub fn count_letters(s: &String) -> HashMap<char, usize> {
    let mut letter_counts = HashMap::new();

    for c in s.chars().filter(|c| c.is_alphabetic()) {
        let counter = letter_counts.entry(c).or_insert(0);
        *counter += 1;
    }

    letter_counts
}