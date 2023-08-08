use std::env;

mod log_core;
mod log_init;
mod log_get;
mod log_new;
mod log_update;
mod log_remove;
mod log_build;
mod log_release;


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
            log_init::run();
        },
        "get" => {
            log_get::run();
        },
        "new" => {
            if args.len() > 2 {
                let string = String::from(&args[2]);
                println!("{:?}", string);
                log_new::run(string);
            }
        },
        "update" => {
            if args.len() > 2 {
                log_update::run(Some(String::from(&args[2])));   
            } else {
                log_update::run(None);
            }
        },
        "remove" => {
            if args.len() > 2 {
                log_remove::run(Some(String::from(&args[2])));   
            } else {
                log_remove::run(None);
            }
        },
        "build" => {
            if args.len() > 2 {
                let version = String::from(&args[2]);
                
                if version == "--current" {
                    println!("{}", version);
                    log_build::run_current();    
                } else {
                    println!("{}", version);
                    log_build::run_with_version(version);
                }
            } else {
                log_build::run()
            }
        },
        "release" => {
            log_release::run()
        }
        &_ => {
        }
    }

    // TODO: --help command

    Ok(())
}