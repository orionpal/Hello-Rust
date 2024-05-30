use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use csv::ReaderBuilder;
use csv::Writer;
use std::fs;
use std::io;
use std::collections::HashMap;



pub fn read_csv<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, io::Error> {
    // 1. Open the CSV file
    let file = fs::File::open(file_path)?;
    // 2. Create a CSV reader
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);
    
    // 3. Iterate over the records and collect them into a vector
    let mut records: Vec<T> = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn read_some_csv(file_path: &str) -> Result<Vec<HashMap<String, Value>>, io::Error> {
    // 1. Open file
    let file = fs::File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    // 2. Get the headers
    let headers = rdr.headers()?.clone();
    
    // 3. Iterate over the records and populate a HashMap
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut map = HashMap::new(); // Using map as generic object type

        // Populate the map with the field names and values
        for (header, field) in headers.iter().zip(record.iter()) {
            map.insert(header.to_string(), Value::String(field.to_string()));
        }
        records.push(map);
    }
    Ok(records)
}

pub fn write_csv<T: Serialize>(file_path: &str, records: &Vec<T>) -> Result<(), io::Error>  {
    // 1. Create a file
    let file = fs::File::create(file_path)?;

    // 2. Write the records to the CSV file
    let mut wtr = Writer::from_writer(file);
    for record in records {
        wtr.serialize(record)?;
    }
    // 3. Flush writer buffer into file (careful there's a trailing \n)
    wtr.flush()?;

    Ok(())
}