use crate::{log_core::{self, Log}, APP_DIRECTORY, APP_DB_NAME};


pub fn run(string: String) {
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