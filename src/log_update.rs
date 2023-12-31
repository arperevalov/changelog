use dialoguer::Input;

use crate::{APP_DIRECTORY, APP_DB_NAME, log_core::{self, Log}};

pub fn run(index: Option<String>) {
    let file_path: String = format!("{}{}", APP_DIRECTORY, APP_DB_NAME);
    let mut base = log_core::get_base();
    let mut records: Vec<Log> = base.app_current_logs;

    if records.len() == 0 {
        return println!("No records to update");
    }

    let id:Result<usize, String> = match index {
        Some(index) => {
            let id:usize = index.parse().expect("Give number");
            Ok(id)
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
                Ok(index) => {Ok(index)},
                Err(error) => {Err(error)}
            }
        }
    };

    let id = id.unwrap();

    if records.len() <= id {
        println!("There are no records with id of {}", id);
        return;
    }

    println!("Please provide new text for record\n");

    let input: String = Input::new()
        .with_initial_text(&records[id].text)
        .interact_text().unwrap();

    records[id].text = input.to_string();

    base.app_current_logs = records;

    log_core::rewrite_file(file_path, base).unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
}