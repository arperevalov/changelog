use crate::{APP_DIRECTORY, APP_DB_NAME, log_core::{self, Log, LogArchive, Base}, log_build};
use chrono::{Utc};

pub fn run() {
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);
    let base = log_core::get_base();
    let records: Vec<Log> = base.app_current_logs;

    log_build::run_current();

    let mut version:f64 = base.app_current_version.parse().expect("Сannot parse app current version");
    version += 0.01;

    let date = Utc::now().to_string();

    let current_archive: LogArchive = LogArchive { logs: records, date: date.to_string() };

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