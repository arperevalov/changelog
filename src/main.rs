use std::fs::File;
use std::env;

use log_core::{Log};
mod log_core;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let error = String::from(format!("Provided {} instead of 1", args.len() - 1 ));
        return Err(error)
    }

    match args[1].as_str() {
        "i" => {
            init();
        },
        "g" => {
            get_current_records();
        },
        "n" => {
            if args.len() > 2 {
                let string = String::from(&args[2]);
                println!("{:?}", string);
                new_record(string);
            }
        },
        "r" => {
            let index = 0;
            remove_record(index)
        },
        "b" => {
            build_report()
        }
        &_ => {
        }
    }

    // TODO: --help command
   
    // build_report()

    Ok(())
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
    let file_path = "./.changelog/data.json";
    let mut base = log_core::get_json();
    let mut records: Vec<Log> = base.app_current_logs;

    let new_record = Log {
        group: 0,
        text: string
    };

    records.push(new_record);
    base.app_current_logs = records;

    log_core::rewrite_file(file_path, base).unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}

fn remove_record(index: usize) {
    let file_path = "./.changelog/data.json";
    let mut base = log_core::get_json();
    let mut records: Vec<Log> = base.app_current_logs;


    records.remove(index);

    base.app_current_logs = records;

    log_core::rewrite_file(file_path, base).unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}

fn build_report() {
    let directory = String::from("./.changelog/reports");

    log_core::new_directory(directory).unwrap_or_else(|err| {
        println!("Problem creating reports directory: {}", err);
    }) ;

    let base = log_core::get_json();
    let mut logs_string = String::new();

    for item in &base.app_current_logs {
        logs_string += &item.text;
    }
        
    let report = format!(
"{}, {}
logs: {}", 
    base.app_name, base.app_current_version, logs_string);

    println!("{}", report);
    // create or rewrite file
    // paste data to file
}

// fn build_report_with_commits() {
//     // get all versions data
//     // format data
//     // create or rewrite file
//     // paste data to file
// }

// fn build_report_with_version() {
//     // get all version data
//     // format data
//     // create or rewrite file
//     // paste data to file
// }
