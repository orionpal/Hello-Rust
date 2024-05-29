use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io;
use std::io::Write;

/*
The DeserializeOwned trait is used because it ensures
that the deserialized data does not borrow from the input data string.

This is important because the JSON string read from the file is a
temporary variable inside the function.

If the deserialized type borrowed data from this string,
the references would be invalid once the function exits.
By requiring DeserializeOwned, we ensure that the deserialized type fully owns its data,
avoiding potential lifetime issues.
*/
// Function to read JSON from a file where we know the form as some struct of type T
pub fn read_json<T: DeserializeOwned>(file_path: &String) -> Result<T, io::Error> {
    let data = fs::read_to_string(file_path)?;
    let obj: T = serde_json::from_str(&data)?;
    Ok(obj)
}

pub fn read_some_json(file_path: &String) -> Result<Value, io::Error> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
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