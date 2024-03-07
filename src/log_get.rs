use crate::log_core::{Base, Log};

pub fn run () -> Result<(), String> {
    let base = match Base::get() {
        Ok(value) => value,
        Err(value) => {
            let string = value.to_string();
            return Err(string);
        }
    };
    let logs: Vec<Log> = base.app_current_logs;
    
    if logs.len() == 0 {
        let error = String::from("There are no records yet! Try run \"changelog new <STRING>\" to make your first record.");
        return Err(error);
    }
    
    println!("Changes of this version:");
    for item in logs {
        println!("— {}", item.text);
    }

    Ok(())
}