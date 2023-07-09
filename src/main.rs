use std::fs::{File, self};
use std::env;

use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use log_core::{Log};
mod log_core;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let error = String::from(format!("Provided {} instead of 1", args.len() - 1 ));
        return Err(error)
    }

    match args[1].as_str() {
        "init" => {
            init();
        },
        "get" => {
            get_current_records();
        },
        "new" => {
            if args.len() > 2 {
                let string = String::from(&args[2]);
                println!("{:?}", string);
                new_record(string);
            }
        },
        "remove" => {
            if args.len() > 2 {
                remove_record(Some(String::from(&args[2])));   
            } else {
                remove_record(None);
            }
        },
        "build" => {
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

fn remove_record(index: Option<String>) {
    let file_path = "./.changelog/data.json";
    let mut base = log_core::get_json();
    let mut records: Vec<Log> = base.app_current_logs;

    match index {
        Some(index) => {
            let id:usize = index.parse().expect("Give number");
            records.remove(id);
        },
        None => {
            let mut values = vec![];


            for record in &records {
                println!("{}", &record.text);
                let text = String::from(&record.text);
                values.push(text);
            }

            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&values)
                .default(0)
                .interact_on_opt(&Term::stderr()).unwrap();

            match selection {
                Some(index) => {records.remove(index);},
                None => println!("User did not select anything"),
                _ => println!("User did not select anything")
            }
        }
    }

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
    let file_path = format!("./.changelog/reports/{}.txt", &base.app_current_version);
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
