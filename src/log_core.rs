use std::fs::{File, DirBuilder};
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    pub app_name: String,
    pub app_current_version: String,
    pub app_current_logs: Vec<Log>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub group: u64,
    pub text: String,
}

pub fn new_directory(directory: String) -> Result<(), String> {
    match std::fs::read_dir(&directory) {
        Ok(..) => {
            Ok(())
        },
        Err(..) => {
            match DirBuilder::new().create(&directory) {
                Ok(..) => Ok(()),
                Err(..) => Err(String::from("Could not create directory. It may already exists!"))
            }
        }
    }
}


pub fn new_json() -> Result<File, String> {
    match File::create("./.changelog/data.json") {
        Ok(file) => Ok(file),
        Err(..) => Err(String::from("Could not create file"))
    }
}

pub fn write_initial_data(mut file: File) -> Result<(), String> {
    let data = b"{
    \"app_name\": \"app_name\",
    \"app_current_version\": \"0.01\",
    \"app_current_logs\": [
        {
            \"group\": 0,
            \"text\": \"Rule of cool\"
        }
    ],
    \"app_previous\": {
        \"0.00\": {
            \"logs\": [],
            \"commits\": []
        }
    }
}";

    match file.write_all(data) {
        Ok(..) => Ok(()),
        Err(..) => Err(String::from("Could not write initial data."))
    }
}

pub fn read_file() -> Result<String, Error> {
    let mut file = File::open("./.changelog/data.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

pub fn get_json() -> Base {
    let result = read_file().expect("error");
    let data: Base = serde_json::from_str(&result).expect("Could not read JSON");
    data
}

pub fn rewrite_file(path: &str, data: Base) -> Result<(), String> {
    let file = File::create(path).expect("something");

    match serde_json::to_writer_pretty(file, &data) {
        Ok(..) => Ok(()),
        Err(error) => {
            
    println!("{:#?}", error);
            Err(String::from("Could not write new data."))
        }
    }
}
