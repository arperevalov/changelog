use std::collections::HashMap;
use std::fs::{File, DirBuilder};
use std::io::prelude::*;
use dialoguer::Select;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use serde::{Deserialize, Serialize};
use std::io::Error;
use std::fmt;

use crate::{APP_DIRECTORY, APP_DB_NAME};

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    pub app_name: String,
    pub app_current_version: String,
    pub app_current_logs: Vec<Log>,
    pub app_previous: HashMap<String, LogArchive>
}

impl Base {
    fn get_filepath () -> String {
        format!("{}{}", APP_DIRECTORY, APP_DB_NAME)
    }

    fn read_file(path: &String) -> Result<String, Error> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(data)
    }

    pub fn from(app_name: String) -> Base {
        Base {
            app_name,
            app_current_version: String::from("0.0.0"),
            app_current_logs: vec![],
            app_previous: HashMap::new(),
        }
    }

    pub fn get() -> Base {
        let base_path: String = Base::get_filepath();
        let result = Base::read_file(&base_path).expect("error");
        let data: Base = serde_json::from_str(&result).expect("Could not read JSON");
        data
    }

    pub fn write(self) -> Result<(), String> {
        let file = File::create(Base::get_filepath()).expect("something");

        match serde_json::to_writer_pretty(file, &self) {
            Ok(..) => Ok(()),
            Err(error) => {
                
        println!("{:#?}", error);
                Err(String::from("Could not write new data."))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub group: u64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogArchive {
    pub logs: Vec<Log>,
    pub date: String,
}

pub enum SelectDefault {
    Empty,
    Is(usize)
}

#[derive(Debug)]
pub enum Version {
    Major,
    Minor,
    Patch
}
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:?}", self)
        // or, alternatively:
        fmt::Debug::fmt(self, f)
    }
}

pub fn new_directory(path: &String) -> Result<(), String> {
    match std::fs::read_dir(&path) {
        Ok(..) => {
            Ok(())
        },
        Err(..) => {
            match DirBuilder::new().create(&path) {
                Ok(..) => Ok(()),
                Err(..) => Err(String::from("Could not create directory"))
            }
        }
    }
}

pub fn _remove_directory(path: &String) -> Result<(), String> {
    match std::fs::remove_dir(&path) {
        Ok(..) => {
            Ok(())
        },
        Err(..) => {
            Err(String::from("Deletion error"))
        }
    }
}

pub fn new_json(path: &String) -> Result<File, String> {
    match File::create(path) {
        Ok(file) => Ok(file),
        Err(..) => Err(String::from("Could not create file"))
    }
}


pub fn set_select(values: &Vec<String>, default: SelectDefault) -> Result<usize, String> {
    let default_value = match default {
        SelectDefault::Is(value) => {
            value
        },
        SelectDefault::Empty => {
            0
        }
    };

    let select = Select::with_theme(&ColorfulTheme::default())
        .items(&values)
        .default(default_value)
        .interact_on_opt(&Term::stderr()).unwrap();

    match select {
        Some(index) => {
            Ok(index)
        },
        None => {
            Err(String::from("User did not select anything"))
        }
    }
}

pub fn increment_version(value: &String, position: u8) -> String {
    let expression = regex::Regex::new(r"\d{0,}").unwrap();

    let mut versions = vec![];
    for item in expression.captures_iter(&value) {
        let version: u32 = item.get(0).unwrap().as_str().parse().expect("Version parsing went wrong");
        versions.push(version);
    };

    let updated_version = match position {
        0 => {
            versions[0] = versions[0] + 1;
            versions[1] = 0;
            versions[2] = 0;
            format!("{}.{}.{}", versions[0], versions[1], versions[2])
        }
        1 => {
            versions[0] = versions[0];
            versions[1] = versions[1] + 1;
            versions[2] = 0;
            format!("{}.{}.{}", versions[0], versions[1], versions[2])
        }
        2 => {
            versions[0] = versions[0];
            versions[1] = versions[1];
            versions[2] = versions[2] + 1;
            format!("{}.{}.{}", versions[0], versions[1], versions[2])
        },
        3_u8..=u8::MAX => {value.to_string()}
    };

    String::from(updated_version)
}