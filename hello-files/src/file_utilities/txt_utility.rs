use std::io::BufRead;
use std::fs;
use std::io;
use std::io::Write;

pub fn read_file(file_path: &String) -> Result<Vec<String>, io::Error> {
    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            let lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
            return Ok(lines);
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            Err(e)
        }
    }
}

pub fn process_file(file_path: &String, process_line: &dyn Fn(&String)) -> Result<(), io::Error> {
    match fs::File::open(file_path) {
        Ok(file) => {
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

pub fn write_file(file_path: &String, content: &String) -> Result<(), io::Error> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}