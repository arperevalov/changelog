use crate::{log_build, log_core::{self, Base, Log, LogArchive, Version}};
use chrono::Utc;

pub fn run() {
    let mut base = Base::get();

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

    let records: Vec<Log> = base.app_current_logs.clone();
    let date = Utc::now().to_string();

    let current_archive: LogArchive = LogArchive { logs: records, date: date.to_string() };

    let mut previous_records = base.app_previous.clone();
    previous_records.insert(base.app_current_version.clone(), current_archive);

    base.increment_version(position).expect("Cannot increment app version");
    base.app_current_logs = vec![];
    base.app_previous = previous_records;

    base.write().unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}