use std::fs::{File, self};

use crate::{APP_DIRECTORY, APP_DB_NAME, log_core};


pub fn run() {
    // TODO: ask if should work with git
    // TODO: ask project name

    let directory = String::from(APP_DIRECTORY);
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);

    log_core::new_directory(&directory).unwrap_or_else(|err| {
        println!("Problem creating directory: {}", err);
    }) ;

    let file = log_core::new_json(&file_path).unwrap_or_else(|err| {
        println!("Problem creating file: {}", err);
        File::open("/dev/null").expect("Failed to open /dev/null")
    });

    log_core::write_initial_data(file).unwrap_or_else(|err| {
        println!("Problem writing file: {}", err);
    });

    let file_path = String::from(APP_DIRECTORY) + &String::from(".gitignore");
    let data = "/reports";

    fs::write(file_path, data).expect("Unable to write .gitignore");
    // TODO: print instruction how to use after init
}