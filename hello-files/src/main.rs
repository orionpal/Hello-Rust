use std::env;
use std::path::Path;
use std::io;
mod file_utilities;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // Get arguments
        let file_path: &str =  &args[1];
        // Get file extension, then convert to string, then match
        match Path::new(file_path).extension().and_then(|ext| ext.to_str()){
            Some("csv") => {
                println!("This is a CSV file.");
                file_utilities::testing::csv_test::handle_csv(file_path)?;
            }
            Some("json") => {
                println!("This is a JSON file.");
                file_utilities::testing::json_test::handle_json(file_path)?;
            }
            Some("txt") => {
                println!("This is a text file.");
                file_utilities::testing::txt_test::handle_txt(file_path)?;
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
