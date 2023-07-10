use std::fs::{File, self};
use std::{env, vec};

use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use log_core::{Log, LogArchive, Base};
mod log_core;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let error = String::from("Provided no arguments");
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
        },
        "release" => {
            release()
        }
        &_ => {
        }
    }

    // TODO: --help command

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
    
    if logs.len() == 0 {
        return println!("There are no records yet! Try run \"changelog new <STRING>\" to make your first record.")
    }
    
    println!("Changes of this version:");
    for item in logs {
        println!("— {}", item.text);
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

    if records.len() == 0 {
        return println!("No records to remove");
    }

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
                None => println!("User did not select anything")
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

fn release() {
    let file_path = "./.changelog/data.json";
    let base = log_core::get_json();
    let records: Vec<Log> = base.app_current_logs;

    let mut commits = vec![];
    commits.push(String::from("some_string"));

    let mut version:f64 = base.app_current_version.parse().expect("cannot parse app current version");
    version += 0.01;

    let current_archive: LogArchive = LogArchive { logs: records, commits };

    let mut previous_records = base.app_previous;
    previous_records.insert(base.app_current_version, current_archive);

    let new_base = Base {
        app_current_logs: vec![],
        app_name: base.app_name,
        app_current_version: version.to_string(),
        app_previous: previous_records
    };

    log_core::rewrite_file(file_path, new_base).unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
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