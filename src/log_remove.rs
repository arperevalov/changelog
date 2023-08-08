use crate::{APP_DIRECTORY, APP_DB_NAME, log_core::{self, Log}};

pub fn run(index: Option<String>) {
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

            let selection = log_core::set_select(&values);

            match selection {
                Ok(index) => {records.remove(index);},
                Err(error) => println!("{}", error)
            }
        }
    }

    base.app_current_logs = records;

    log_core::rewrite_file(file_path, base).unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}