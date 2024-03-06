use crate::log_core::{Base, Log};

pub fn run () {
    let base = Base::get();
    let logs: Vec<Log> = base.app_current_logs;
    
    if logs.len() == 0 {
        return println!("There are no records yet! Try run \"changelog new <STRING>\" to make your first record.")
    }
    
    println!("Changes of this version:");
    for item in logs {
        println!("— {}", item.text);
    }
}