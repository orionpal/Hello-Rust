use std::io::BufRead;
use std::fs;
use std::io;
use std::io::Write;

pub fn read_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    // 1. Try reading file
    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            // 2. Output the contents into a Vec<String> object
            let lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
            return Ok(lines);
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            Err(e)
        }
    }
}

pub fn process_file(file_path: &str, process_line: &dyn Fn(&String)) -> Result<(), io::Error> {
    // 1. Try opening the file
    match fs::File::open(file_path) {
        Ok(file) => {
            // 2. Apply the passed function to each line of the file
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => process_line(&line),
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                        return Err(e);
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            Err(e)
        }
    }
}

pub fn write_file(file_path: &str, content: &String) -> Result<(), io::Error> {
    // 1. Create
    let mut file = fs::File::create(file_path)?;
    // 2. Write
    file.write_all(content.as_bytes())?;
    Ok(())
}