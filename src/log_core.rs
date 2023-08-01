use std::collections::HashMap;
use std::fs::{File, DirBuilder};
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Error;

use crate::{APP_DIRECTORY, APP_DB_NAME};

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    pub app_name: String,
    pub app_current_version: String,
    pub app_current_logs: Vec<Log>,
    pub app_previous: HashMap<String, LogArchive>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub group: u64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogArchive {
    pub logs: Vec<Log>,
    pub commits: Vec<String>,
}

pub fn new_directory(path: &String) -> Result<(), String> {
    match std::fs::read_dir(&path) {
        Ok(..) => {
            Ok(())
        },
        Err(..) => {
            match DirBuilder::new().create(&path) {
                Ok(..) => Ok(()),
                Err(..) => Err(String::from("Could not create directory. It may already exists!"))
            }
        }
    }
}


pub fn new_json(path: &String) -> Result<File, String> {
    match File::create(path) {
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

pub fn read_file(path: &String) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

pub fn get_base() -> Base {
    let base_path: String = String::from(APP_DIRECTORY) + &String::from(APP_DB_NAME);
    let result = read_file(&base_path).expect("error");
    let data: Base = serde_json::from_str(&result).expect("Could not read JSON");
    data
}

pub fn rewrite_file(path: String, data: Base) -> Result<(), String> {
    let file = File::create(path).expect("something");

    match serde_json::to_writer_pretty(file, &data) {
        Ok(..) => Ok(()),
        Err(error) => {
            
    println!("{:#?}", error);
            Err(String::from("Could not write new data."))
        }
    }
}
