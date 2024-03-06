use std::fs;

use dialoguer::Input;

use crate::{log_core::{self, Base}, log_help, APP_DIRECTORY};


pub fn run() {
    let directory = String::from(APP_DIRECTORY);

    log_core::new_directory(&directory).unwrap_or_else(|err| {
        println!("Problem creating directory: {}", err);
    }) ;

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