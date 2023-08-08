
pub fn run() {
    let message = "Handle your application versions history.

Usage: changelog [COMMAND] [OPTION]

COMMANDS:
    init        Init changelog application files
    new         Set record to current application version
    get         Get all current version records
    update      Edit single record in current records
    remove      Remove single record in current records
    build       Build all records of specific version into single file
    release     Set current version to archive (history), clear all current records, autoincrement version of application
    ";
    println!("{}", message)
}