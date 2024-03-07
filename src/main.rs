use std::{env, process};

mod log_core;
mod log_init;
mod log_get;
mod log_new;
mod log_update;
mod log_remove;
mod log_build;
mod log_release;
mod log_help;


const APP_DIRECTORY: &str = "./.changelog/";
const APP_DIRECTORY_REPORTS: &str = "reports/";
const APP_DB_NAME: &str = "data.json";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let error = String::from("Provided no arguments");
        end_with_error(error);
    }

    match args[1].as_str() {
        "init" => {
            log_init::run().unwrap_or_else(|error| end_with_error(error));
        },
        "get" => {
            log_get::run().unwrap_or_else(|error| end_with_error(error));
        },
        "new" => {
            if args.len() > 2 {
                let string = String::from(&args[2]);
                println!("{:?}", string);
                log_new::run(string).unwrap_or_else(|error| end_with_error(error));
            }
        },
        "update" => {
            if args.len() > 2 {
                log_update::run(Some(String::from(&args[2]))).unwrap_or_else(|error| end_with_error(error));   
            } else {
                log_update::run(None).unwrap_or_else(|error| end_with_error(error));
            }
        },
        "remove" => {
            if args.len() > 2 {
                log_remove::run(Some(String::from(&args[2]))).unwrap_or_else(|error| end_with_error(error));   
            } else {
                log_remove::run(None).unwrap_or_else(|error| end_with_error(error));
            }
        },
        "build" => {
            if args.len() > 2 {
                let version = String::from(&args[2]);
                
                if version == "--current" {
                    println!("{}", version);
                    log_build::run_current().unwrap_or_else(|error| end_with_error(error));    
                } else {
                    println!("{}", version);
                    log_build::run_with_version(version).unwrap_or_else(|error| end_with_error(error));
                }
            } else {
                log_build::run().unwrap_or_else(|error| end_with_error(error))
            }
        },
        "release" => {
            log_release::run().unwrap_or_else(|error| end_with_error(error))
        },
        "help" => {
            log_help::run()
        },
        &_ => {
        }
    }
}

fn end_with_error(message: String) {
    println!("{message}");
    process::exit(1);
}