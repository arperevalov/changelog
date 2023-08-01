use std::fs::{File, self};
use std::{env, vec};

use dialoguer::{console::Term, theme::ColorfulTheme, Select, Input};
use log_core::{Log, LogArchive, Base};
mod log_core;


const APP_DIRECTORY: &str = "./.changelog/";
const APP_DIRECTORY_REPORTS: &str = "reports/";
const APP_DB_NAME: &str = "data.json";

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
        "update" => {
            if args.len() > 2 {
                update_record(Some(String::from(&args[2])));   
            } else {
                update_record(None);
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
            if args.len() > 2 {
                let version = String::from(&args[2]);
                build_report_with_version(version);
            } else {
                build_report()
            }
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

fn get_current_records () {
    let base = log_core::get_base();
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
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);
    let mut base = log_core::get_base();
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

fn update_record(index: Option<String>) {
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);
    let mut base = log_core::get_base();
    let mut records: Vec<Log> = base.app_current_logs;

    if records.len() == 0 {
        return println!("No records to update");
    }

    let id:Result<usize, &str> = match index {
        Some(index) => {
            let id:usize = index.parse().expect("Give number");
            Ok(id)
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
                Some(index) => {Ok(index)},
                None => {Err("User did not select anything")}
            }
        }
    };

    let id = id.unwrap();

    if records.len() <= id {
        println!("There are no records with id of {}", id);
        return;
    }

    let input: String = Input::new()
        .with_prompt("Please provide new text for record\n")
        .with_initial_text(&records[id].text)
        .interact_text().unwrap();

    records[id].text = input.to_string();

    base.app_current_logs = records;

    log_core::rewrite_file(file_path, base).unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}

fn remove_record(index: Option<String>) {
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);
    let mut base = log_core::get_base();
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

fn release() {
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);
    let base = log_core::get_base();
    let records: Vec<Log> = base.app_current_logs;

    build_report();

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

fn build_report_with_version(version: String) {
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