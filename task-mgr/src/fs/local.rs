use serde::{Serialize, de::DeserializeOwned};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use tempfile::NamedTempFile;
use thiserror::Error;
use tracing::{debug, error};

const FILE_SAVE_FAIL_MSG: &str = "Failed to save to file";

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("I/O error: {0}")]
    Io(io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("File not found")]
    FileNotFound,
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        if e.kind() == std::io::ErrorKind::NotFound {
            LoadError::FileNotFound
        } else {
            LoadError::Io(e)
        }
    }
}

// TODO raising err instead of failing quietly
pub fn save_to_file<T: Serialize>(file_path: &str, tasks: &T) {
    let Ok(temp_file) = save_to_temp_file(tasks) else {
        return;
    };
    match temp_file.persist(file_path) {
        Ok(_) => debug!("Tasks saved."),
        Err(e) => error!("{FILE_SAVE_FAIL_MSG}: {}", e),
    };
}

pub fn load_file<T: DeserializeOwned>(file_path: &str) -> Result<T, LoadError> {
    let file = File::open(file_path)?;
    Ok(serde_json::from_reader(file)?)
}

fn save_to_temp_file<T: Serialize>(obj: &T) -> Result<tempfile::NamedTempFile, ()> {
    let mut temp_file = NamedTempFile::new().map_err(|e| {
        error!("{FILE_SAVE_FAIL_MSG}: {}", e);
    })?;
    let task_json = serde_json::to_string_pretty(obj).map_err(|e| {
        error!("{FILE_SAVE_FAIL_MSG}: {}", e);
    })?;
    if let Err(e) = temp_file.write_all(task_json.as_bytes()) {
        error!("{FILE_SAVE_FAIL_MSG}: {}", e);
        return Err(());
    }
    temp_file.flush().unwrap();
    Ok(temp_file)
}
