use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

// Function to read JSON from a file
fn read_json(file_path: &str) -> Result<Person> {
    let data = fs::read_to_string(file_path)?;
    let person: Person = serde_json::from_str(&data)?;
    Ok(person)
}

// Function to write JSON to a file
fn write_json(file_path: &str, person: &Person) -> Result<()> {
    let json_data = serde_json::to_string_pretty(person)?;
    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}