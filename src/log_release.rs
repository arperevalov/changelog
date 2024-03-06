use crate::{log_core::{self, Log, LogArchive, Base, increment_version, Version}, log_build};
use chrono::Utc;

pub fn run() {
    let mut base = Base::get();
    let records: Vec<Log> = base.app_current_logs;

    log_build::run_current();

    println!("Please, select which version you want to update");
    let values = vec![
        Version::Major.to_string(),
        Version::Minor.to_string(),
        Version::Patch.to_string(),
    ];
    let selection = log_core::set_select(&values, log_core::SelectDefault::Is(2));

    let position = match selection {
        Ok(value) => {
            value as u8
        },
        Err(error) => {
            println!("{error}");
            3
        }
    };

    let version:String = increment_version(&base.app_current_version, position);

    let date = Utc::now().to_string();

    let current_archive: LogArchive = LogArchive { logs: records, date: date.to_string() };

    let mut previous_records = base.app_previous;
    previous_records.insert(base.app_current_version, current_archive);

    base.app_current_logs = vec![];
    base.app_current_version = version;
    base.app_previous = previous_records;

    base.write().unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}