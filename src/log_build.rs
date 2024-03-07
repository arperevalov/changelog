use std::fs;

use crate::{log_core::{self, Base}, APP_DIRECTORY, APP_DIRECTORY_REPORTS};


pub fn run() -> Result<(), String> {
    let base = match Base::get() {
        Ok(value) => value,
        Err(value) => {
            let string = value.to_string();
            return Err(string);
        }
    };
    let previous_versions = base.app_previous;

    if previous_versions.len() == 0 {
        let error = String::from("No versions to build");
        return Err(error);
    }

    let mut values = vec![];

    for item in &previous_versions {
        let text = String::from(item.0);
        values.push(text);
    }

    let selection = log_core::set_select(&values, log_core::SelectDefault::Empty);

    match selection {
        Ok(index) => {
            let version = String::from(&values[index]);
            return run_with_version(version);
        },
        Err(error) => println!("{}", error)
    }

    Ok(())
}

pub fn run_current() -> Result<(), String> {
    let directory: String = format!("{}{}", APP_DIRECTORY, APP_DIRECTORY_REPORTS);

    log_core::new_directory(&directory).unwrap_or_else(|err| {
        println!("Problem creating reports directory: {}", err);
    }) ;

    let base = match Base::get() {
        Ok(value) => value,
        Err(value) => {
            let string = value.to_string();
            return Err(string);
        }
    };
    let file_path = format!("{}{}.txt",&directory, &base.app_current_version);
    let mut logs_string = String::new();

    for item in &base.app_current_logs {
        logs_string = logs_string + "\n— " + &item.text;
    }
        
    let report = format!(
"{}, {}

Changes of this version: {}", 
    base.app_name, base.app_current_version, logs_string);

    let data = String::from(report);

    fs::write(file_path, data).expect("Unable to write file");
    Ok(())
}

pub fn run_with_version(version: String) -> Result<(), String> {
    let directory: String = format!("{}{}", APP_DIRECTORY, APP_DIRECTORY_REPORTS);

    log_core::new_directory(&directory).unwrap_or_else(|err| {
        println!("Problem creating reports directory: {}", err);
    }) ;

    let base = match Base::get() {
        Ok(value) => value,
        Err(value) => {
            let string = value.to_string();
            return Err(string);
        }
    };
    let file_path = format!("{}{}.txt",&directory, &version);
    let previous_records = base.app_previous;
    let record = previous_records.get(&version).expect("No releases with this version found");

    let date = &record.date;
    let mut logs_string = String::new();

    for log in &record.logs {
        logs_string = logs_string + "\n— " + &log.text;
    }

    let report = format!(
"{}, {}
Release date: {}

Changes of this version: {}", 
    base.app_name, version, date, logs_string);

    let data = String::from(report);

    fs::write(file_path, data).expect("Unable to write file");

    Ok(())
}