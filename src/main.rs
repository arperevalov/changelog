use std::fs::File;

use log_core::{Base, Log};
mod log_core;

fn main() {
    // TODO: check if arguments provided
    // TODO: --help command

    // init();
    
    // get_current_records();

    let string = String::from("something");
    new_record(string);
    
    // remove_record()
    
    // build_report()
}

fn init() {
    // TODO: ask if should work with git
    // TODO: ask project name

    let directory = String::from("./.changelog/");

    log_core::new_directory(directory).unwrap_or_else(|err| {
        println!("Problem creating directory: {}", err);
    }) ;

    let file = log_core::new_json().unwrap_or_else(|err| {
        println!("Problem creating file: {}", err);
        File::open("/dev/null").expect("Failed to open /dev/null")
    });

    log_core::write_initial_data(file).unwrap_or_else(|err| {
        println!("Problem writing file: {}", err);
    });

    // TODO: print instruction how to use after init
}

fn get_current_records () {
    let base = log_core::get_json();
    let logs: Vec<Log> = base.app_current_logs;
    
    for item in logs {
        println!("{}", item.text);
    }
}

fn new_record(string: String) {
    let mut base = log_core::get_json();
    let mut records: Vec<Log> = base.app_current_logs;

    let new_record = Log {
        group: 0,
        text: string
    };

    records.push(new_record);
    base.app_current_logs = records;

    log_core::rewrite_file(base).unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}

fn remove_record() {
    let mut base = log_core::get_json();
    let mut records: Vec<Log> = base.app_current_logs;

    let selection = 2;

    records.remove(selection);

    base.app_current_logs = records;

    log_core::rewrite_file(base).unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}

fn build_report() {
    let base = log_core::get_json();
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
