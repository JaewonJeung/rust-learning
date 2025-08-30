use serde::{Serialize, de::DeserializeOwned};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use tempfile::NamedTempFile;

const FILE_SAVE_FAIL_MSG: &str = "Failed to save to file";

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        LoadError::Io(e)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(e: serde_json::Error) -> Self {
        LoadError::Json(e)
    }
}

// TODO raising err instead of failing quietly
pub fn save_to_file<T: Serialize>(file_path: &str, tasks: &T) {
    let temp_file = match save_to_temp_file(tasks) {
        Ok(file) => file,
        Err(()) => return,
    };
    match temp_file.persist(file_path) {
        Ok(_) => println!("Tasks saved."),
        Err(e) => eprintln!("{FILE_SAVE_FAIL_MSG}: {}", e),
    }
}

// how the heck does this work
pub fn load_file<T: DeserializeOwned>(file_path: &str) -> Result<T, LoadError> {
    let file = File::open(file_path).map_err(LoadError::from)?;
    serde_json::from_reader(file).map_err(LoadError::from)
}

// TODO use anyhow?
fn save_to_temp_file<T: Serialize>(obj: &T) -> Result<tempfile::NamedTempFile, ()> {
    let mut temp_file = match NamedTempFile::new() {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{FILE_SAVE_FAIL_MSG}: {}", e);
            return Err(());
        }
    };
    let task_json = match serde_json::to_string_pretty(obj) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("{FILE_SAVE_FAIL_MSG}: {}", e);
            return Err(());
        }
    };
    if let Err(e) = temp_file.write_all(task_json.as_bytes()) {
        eprintln!("{FILE_SAVE_FAIL_MSG}: {}", e);
        return Err(());
    }
    temp_file.flush().unwrap();
    Ok(temp_file)
}
