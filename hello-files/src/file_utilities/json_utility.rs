use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io;
use std::io::Write;

// Function to read JSON from a file where we know the form as some struct of type T
//The DeserializeOwned trait is used because it ensures the return object owns its data, not the file
pub fn read_json<T: DeserializeOwned>(file_path: &str) -> Result<T, io::Error> {
    let data = fs::read_to_string(file_path)?;
    let obj: T = serde_json::from_str(&data)?;
    Ok(obj)
}

pub fn read_some_json(file_path: &str) -> Result<Value, io::Error> {
    let data = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&data)?;
    Ok(json)
}
pub fn write_json<T: Serialize>(file_path: &str, data: &T) -> Result<(), io::Error> {
    // Serialize the data to a JSON string
    let json_string = serde_json::to_string_pretty(data)?;
    
    let mut file = fs::File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}