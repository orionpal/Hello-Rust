use std::env;
use std::path::Path;
use std::io;
mod file_utilities;


fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        // Get arguments
        let file_path: &String =  &args[1];
        // Get file extension, then convert to string, then match
        match Path::new(file_path).extension().and_then(|ext| ext.to_str()){
            Some("csv") => {
                println!("This is a CSV file.");
                // Add logic to handle CSV files here
            }
            Some("json") => {
                println!("This is a JSON file.");
                // Add logic to handle JSON files here
            }
            Some("txt") => {
                println!("This is a text file.");
                handle_txt(file_path)?;
            }
            Some(other) => {
                println!("File extension: {} is not recognized.", other);
            }
            None => {
                println!("No file extension found.");
            }
        }
    }
    else {
        println!("Need only 1 argument which is the filepath of where to read")
    }
    println!("Hello, files!");
    Ok(())
}

fn handle_txt(file_path: &String) -> io::Result<()>{
    fn process_line(line: &String) {
        println!("Processed: {}", line)
    }
    println!("Here we pass a function to be used on the lines in the file:");
    file_utilities::txt_utility::process_file(file_path, &process_line)?;
    println!("Here we just read the lines to a Vec<String>:");
    let lines: Vec<String> = file_utilities::txt_utility::read_file(file_path)?;
    println!("Length of our vector: {}", lines.len());
    println!("And here we write the contents to examples/result.txt");
    let content: String = lines.join("\n");
    let write_path: String = "src/examples/result.txt".to_string();
    file_utilities::txt_utility::write_file(&write_path, &content)?;
    Ok(())
}