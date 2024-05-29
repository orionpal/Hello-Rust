use std::io;
use crate::file_utilities::txt_utility;

pub fn handle_txt(file_path: &String) -> io::Result<()>{
    fn process_line(line: &String) {
        println!("Processed: {}", line)
    }
    println!("Here we pass a function to be used on the lines in the file:");
    txt_utility::process_file(file_path, &process_line)?;
    println!("Here we just read the lines to a Vec<String>:");
    let lines: Vec<String> = txt_utility::read_file(file_path)?;
    println!("Length of our vector: {}", lines.len());
    println!("And here we write the contents to examples/result.txt");
    let content: String = lines.join("\n");
    let write_path: String = "src/examples/result.txt".to_string();
    txt_utility::write_file(&write_path, &content)?;
    Ok(())
}