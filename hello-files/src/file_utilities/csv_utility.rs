use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use csv::ReaderBuilder;
use csv::Writer;
use std::fs;
use std::io;
use std::collections::HashMap;



pub fn read_csv<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, io::Error> {
    // Open the CSV file
    let file = fs::File::open(file_path)?;
    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);
    
    // Iterate over the records and collect them into a vector
    let mut records: Vec<T> = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn read_some_csv(file_path: &str) -> Result<Vec<HashMap<String, Value>>, io::Error> {
    let file = fs::File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    // Get the headers
    let headers = rdr.headers()?.clone();
    
    // Iterate over the records
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
    // Create a CSV writer
    let file = fs::File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    // Write the records to the CSV file
    for record in records {
        wtr.serialize(record)?;
    }
    // Flush writer buffer into file (careful there's a trailing \n)
    wtr.flush()?;

    Ok(())
}