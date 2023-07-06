use std::fs::{File, DirBuilder};
use std::io::prelude::*;
use serde_json::Value as Json;
use std::io::Error;

fn main() {
    // TODO: check if arguments provided
    // TODO: --help command

    init();
    // let value = String::from("somebody");

    // read_value(value);
    // new_record();
}

fn init() {
    // TODO: ask if should work with git
    // TODO: ask project name

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

    // TODO: print instruction how to use after init
}

fn get_current_records () {
    // get_json
    // get_records
}

fn new_record() {
    let mut json = get_json();
    json["somesa"] = "sosos".into();
    println!("{:?}",json);
}

fn remove_record() {
    // get_json
    // get_records
    // display records with select
    // remove current record by index
}

fn build_report() {
    // get all current data
    // format data
    // create or rewrite file
    // paste data to file
}

fn build_report_with_commits() {
    // get all versions data
    // format data
    // create or rewrite file
    // paste data to file
}

fn build_report_with_version() {
    // get all version data
    // format data
    // create or rewrite file
    // paste data to file
}

fn new_directory(directory: String) -> Result<(), String> {
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

fn new_json() -> Result<File, String> {
    match File::create("./.changelog/data.json") {
        Ok(file) => Ok(file),
        Err(..) => Err(String::from("Could not create file"))
    }
}

fn write_initial_data(mut file: File) -> Result<(), String> {
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

fn read_file() -> Result<String, Error> {
    let mut file = File::open("./.changelog/data.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn get_json() -> Json {
    let result = read_file().expect("error");
    serde_json::from_str(&result).expect("Could not read JSON")
}

fn read_value(value: String) {
    let json: Json = get_json();
    println!("{}", json[value]);
}
