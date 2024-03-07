use crate::log_core::{Base, Log};


pub fn run(string: String) -> Result<(), String> {
    let mut base = match Base::get() {
        Ok(value) => value,
        Err(value) => {
            let string = value.to_string();
            return Err(string);
        }
    };
    let mut records: Vec<Log> = base.app_current_logs;

    let new_record = Log {
        group: 0,
        text: string
    };

    records.push(new_record);
    base.app_current_logs = records;

    base.write().unwrap_or_else(|err| {
        println!("Problem writing a file: {}", err);
    });
    Ok(())
}