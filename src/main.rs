use serde_json::{from_reader, to_writer_pretty, Value};
use std::io::{BufReader, stdin};

use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let data = from_reader(reader)?;

    // Return the `User`.
    Ok(data)
}

fn add_entry_to_json(_key: String, value: String) -> std::result::Result<(), serde_json::Error> {
    let mut data = read_json_file("test.json").unwrap();
    data[_key] = serde_json::Value::from(value);
    let writer = &File::create("test.json").expect("error");
    to_writer_pretty(writer, &data)
}

fn user_input(target: String) -> String {
    println!("Enter the {} :", target);
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().to_string()
        }
        Err(_) => "".to_string(),
    }
}

fn main() {
    let input_key = user_input(String::from("key"));
    let input_value = user_input(String::from("value"));
    add_entry_to_json(input_key, input_value).expect("Error")
}
