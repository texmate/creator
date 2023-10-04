// json_writer.rs

extern crate serde;
extern crate serde_json;

use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Component {
    threshold_inputs: String,
    count_inputs: String,
    filename_inputs: String,
}

pub fn write_to_json(
    threshold_inputs: Vec<String>,
    count_inputs: Vec<String>,
    filename_inputs: Vec<String>,
    filename: String,
) -> std::io::Result<()> {
    // Create an empty vector to hold the Component objects
    let mut components: Vec<Component> = Vec::new();

    // Iterate through the input vectors and populate the components vector
    for i in 0..threshold_inputs.len() {
        let component = Component {
            threshold_inputs: threshold_inputs[i].clone(),
            count_inputs: count_inputs[i].clone(),
            filename_inputs: filename_inputs[i].clone(),
        };
        components.push(component);
    }

    // Serialize the components vector to a JSON string
    let json_string = serde_json::to_string(&components).unwrap();

    // Create a JSON object with the key "components"
    let json_data = format!("{{\"components\": {}}}", json_string);

    // Write the JSON data to a file
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
