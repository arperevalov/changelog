use std::fs::File;
mod log_core;

fn main() {
    // TODO: check if arguments provided
    // TODO: --help command

    // init();
    let value = String::from("app_name");

    log_core::read_value(value);
    log_core::new_record();
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
    // get_json
    // get_records
}

fn remove_record() {
    // get_json
    // get_records
    // display records with select
    // remove current record by index
}

fn build_report() {
    // get all current data
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
