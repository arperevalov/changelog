use std::fs::{File, self};

use dialoguer::Input;

use crate::{log_core::{self, Base}, log_help, APP_DB_NAME, APP_DIRECTORY};


pub fn run() {
    let directory = String::from(APP_DIRECTORY);
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);

    log_core::new_directory(&directory).unwrap_or_else(|err| {
        println!("Problem creating directory: {}", err);
    }) ;

    let file = log_core::new_json(&file_path).unwrap_or_else(|err| {
        println!("Problem creating file: {}", err);
        File::open("/dev/null").expect("Failed to open /dev/null")
    });

    let mut name = String::new();

    println!("What's the name of your project?");

    let input = Input::new().interact_text();

    match input {
        Ok(value) => {
            name = value
        },
        Err(_error) => {
            println!("You haven't typed anything. Value will be empty.")
        }
    }

    let base = Base::from(name);
    
    base.write().unwrap_or_else(|err| {
        println!("Problem writing file: {}", err);
    });

    let file_path = String::from(APP_DIRECTORY) + &String::from(".gitignore");
    let data = "/reports";

    fs::write(file_path, data).expect("Unable to write .gitignore");

    log_help::run();
}