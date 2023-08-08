use std::fs;

use crate::{APP_DIRECTORY, APP_DIRECTORY_REPORTS, log_core};


pub fn run() {
    let base = log_core::get_base();
    let previous_versions = base.app_previous;

    if previous_versions.len() == 0 {
        return println!("No versions to build");
    }

    let mut values = vec![];

    for item in &previous_versions {
        let text = String::from(item.0);
        values.push(text);
    }

    let selection = log_core::set_select(&values);

    match selection {
        Ok(index) => {
            let version = String::from(&values[index]);
            run_with_version(version)
        },
        Err(error) => println!("{}", error)
    }
}

pub fn run_current() {
    let directory: String = format!("{}{}", APP_DIRECTORY, APP_DIRECTORY_REPORTS);

    log_core::new_directory(&directory).unwrap_or_else(|err| {
        println!("Problem creating reports directory: {}", err);
    }) ;

    let base = log_core::get_base();
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
}

pub fn run_with_version(version: String) {
    let directory: String = format!("{}{}", APP_DIRECTORY, APP_DIRECTORY_REPORTS);

    log_core::new_directory(&directory).unwrap_or_else(|err| {
        println!("Problem creating reports directory: {}", err);
    }) ;

    let base = log_core::get_base();
    let file_path = format!("{}{}.txt",&directory, &version);
    let previous_records = base.app_previous;
    let record = previous_records.get(&version).expect("No releases with this version found");

    let mut logs_string = String::new();

    for log in &record.logs {
        logs_string = logs_string + "\n— " + &log.text;
    }

    let mut commits_string = String::new();

    for commit in &record.commits {
        commits_string = commits_string + "\n— " + &commit;
    }

    let report = format!(
"{}, {}

Changes of this version: {}

With commits: {}", 
    base.app_name, version, logs_string, commits_string);

    let data = String::from(report);

    fs::write(file_path, data).expect("Unable to write file");
}