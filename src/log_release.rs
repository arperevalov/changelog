use crate::{APP_DIRECTORY, APP_DB_NAME, log_core::{self, Log, LogArchive, Base}, log_build};

pub fn run() {
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);
    let base = log_core::get_base();
    let records: Vec<Log> = base.app_current_logs;

    log_build::run();

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