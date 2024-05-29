use std::io;
use crate::file_utilities::json_utility;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    example_key: String,
    example_number: u32,
    example_nest: Nest,
    example_array: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Nest {
    nest1: String,
    nest2: String
}

pub fn handle_json(file_path: &String) -> io::Result<()> {
    // Known struct serialization
    println!("Here we try reading from the json into our known struct:");
    let example: Example = json_utility::read_json::<Example>(file_path)?;
    println!("{:?}", example);

    // Unknown form serialization
    println!("Here we try reading from the json into a generic Value:");
    let something: Value = json_utility::read_some_json(file_path)?;
    match &something {
        Value::Object(map) => {
            println!("The JSON is an object with the following keys and values:");
            for (key, value) in map {
                println!("Key: {}", key);
                println!("Value: {}", value);
            }
        }
        Value::Array(arr) => {
            println!("The JSON is an array with the following values:");
            for value in arr {
                println!("Value: {}", value);
            }
        }
        Value::String(s) => {
            println!("The JSON is a string: {}", s);
        }
        Value::Number(num) => {
            println!("The JSON is a number: {}", num);
        }
        Value::Bool(b) => {
            println!("The JSON is a boolean: {}", b);
        }
        Value::Null => {
            println!("The JSON is null");
        }
    }

    // Writing 
    println!("Here we try writing the contents to result.json");
    let write_path: String = "result.json".to_string();
    json_utility::write_json::<Example>(&write_path, &example)?;

    Ok(())
}