use dialoguer::{Select, theme::ColorfulTheme, console::Term};

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