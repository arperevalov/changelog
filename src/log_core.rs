use std::collections::HashMap;
use std::fs::{File, DirBuilder};
use std::io::prelude::*;
use dialoguer::Select;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
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
    pub date: String,
}

pub fn new_directory(path: &String) -> Result<(), String> {
    match std::fs::read_dir(&path) {
        Ok(..) => {
            Ok(())
        },
        Err(..) => {
            match DirBuilder::new().create(&path) {
                Ok(..) => Ok(()),
                Err(..) => Err(String::from("Could not create directory"))
            }
        }
    }
}

pub fn _remove_directory(path: &String) -> Result<(), String> {
    match std::fs::remove_dir(&path) {
        Ok(..) => {
            Ok(())
        },
        Err(..) => {
            Err(String::from("Deletion error"))
        }
    }
}

pub fn new_json(path: &String) -> Result<File, String> {
    match File::create(path) {
        Ok(file) => Ok(file),
        Err(..) => Err(String::from("Could not create file"))
    }
}

pub fn write_initial_data(mut file: File, name: String) -> Result<(), String> {
    let data = format!("{{
        \"app_name\": \"{}\",
        \"app_current_version\": \"0.00\",
        \"app_current_logs\": [],
        \"app_previous\": {{}}
    }}", name);

    match file.write_all(data.as_bytes()) {
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
    let base_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);
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

pub fn set_select(values: &Vec<String>) -> Result<usize, String> {
    let select = Select::with_theme(&ColorfulTheme::default())
        .items(&values)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();

    match select {
        Some(index) => {
            Ok(index)
        },
        None => {
            Err(String::from("User did not select anything"))
        }
    }

}

#[test]
fn test_new_directory() {
    let path = String::from("./.testing_dir");
    let result = new_directory(&path);

    assert_eq!(result, Ok(()));

    let remove = _remove_directory(&path);
    
    assert_eq!(remove, Ok(()));
}

#[test]
fn test_existing_directory() {
    let path = String::from("./.testing_dir");
    
    let result = new_directory(&path);
    assert_eq!(result, Ok(()));

    let result = new_directory(&path);
    assert_eq!(result, Ok(()));
}