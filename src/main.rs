use std::fs::{File, DirBuilder};
use std::io::prelude::*;
use serde_json::Value as Json;
use std::io::Error;

fn main() {
    init();
    // let value = String::from("somebody");

    // read_value(value);
    // add_note();
}

fn init() {
    let directory = String::from("./.changelog/");

    new_directory(directory).unwrap_or_else(|err| {
        println!("Problem creating directory: {}", err);
    }) ;

    let file = new_json().unwrap_or_else(|err| {
        println!("Problem creating file: {}", err);
        File::open("/dev/null").expect("Failed to open /dev/null")
    });

    write_initial_data(file).unwrap_or_else(|err| {
        println!("Problem writing file: {}", err);
    });
}

fn new_directory(directory: String) -> Result<(), String> {
    match DirBuilder::new().create(directory) {
        Ok(..) => Ok(()),
        Err(..) => Err(String::from("Could not create directory"))
    }
}

fn new_json() -> Result<File, String> {
    match File::create("./.changelog/data.json") {
        Ok(file) => Ok(file),
        Err(..) => Err(String::from("Could not create directory"))
    }
}

fn write_initial_data(mut file: File) -> Result<(), String> {
    let data = b"{
    \"somebody\": \"once told me\"
}";

    match file.write_all(data) {
        Ok(..) => Ok(()),
        Err(..) => Err(String::from("Could not create directory"))
    }
}

fn read_file() -> Result<String, Error> {
    let mut file = File::open("./.changelog/data.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn get_json() -> Json {
    let result = read_file().expect("error");
    serde_json::from_str(&result).expect("could not read json")
}

fn read_value(value: String) {
    let json: Json = get_json();
    println!("{}", json[value]);
}

fn add_note() {
    let mut json = get_json();
    json["somesa"] = "sosos".into();
    println!("{:?}",json);
}