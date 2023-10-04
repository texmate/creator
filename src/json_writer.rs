// json_writer.rs

extern crate serde;
extern crate serde_json;

use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Component {
    threshold: f64,
    count: i32,
    filename: String,
}

#[derive(Serialize)]
struct JsonFile {
    name: String,
    components: Vec<Component>,
}

pub fn write_to_json(
    threshold_inputs: Vec<String>,
    count_inputs: Vec<String>,
    filename_inputs: Vec<String>,
    name: String,
    filename: String,
) -> std::io::Result<()> {
    // Create an empty vector to hold the Component objects
    let mut components: Vec<Component> = Vec::new();

    // Iterate through the input vectors and populate the components vector
    for i in 0..threshold_inputs.len() {
        let component = Component {
            threshold: threshold_inputs[i].clone().parse::<f64>().unwrap_or(0.0),
            count: count_inputs[i].clone().parse::<i32>().unwrap_or(0),
            filename: filename_inputs[i].clone(),
        };
        components.push(component);
    }

    let wrapper = JsonFile { name: name, components: components };

    //let my_int = my_string;

    // Create a JSON object with the key "components"
    let json_data = serde_json::to_string(&wrapper).unwrap();

    // Write the JSON data to a file
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
