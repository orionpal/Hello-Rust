use std::io;
use crate::file_utilities::csv_utility;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    field1: String,
    field2: u32,
    field3: f64,
}

pub fn handle_csv(file_path: &str) -> io::Result<()> {
    // Known struct serialization
    println!("Here we try reading from the csv into our known struct:");
    let example: Vec<Record> = csv_utility::read_csv::<Record>(file_path)?;
    println!("{:?}", example);

    // Unknown form serialization
    println!("Here we try reading from the csv into a generic Value:");
    let something: Vec<Map<String, Value> = csv_utility::read_some_csv(file_path)?;
    let mut row_num: u32 = &1;
    for row in something {
        println!("Row number")
        for field in row{

        }

    }
    Ok(())
}

