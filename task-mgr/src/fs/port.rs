use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io;
use thiserror::Error;

pub trait Saver {
    fn save<T: Serialize>(&self, tasks: &T) -> Result<(), SaveError>;
    fn load_file<T: DeserializeOwned>(&self) -> Result<T, LoadError>;
}

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

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<tempfile::PersistError> for SaveError {
    fn from(e: tempfile::PersistError) -> Self {
        SaveError::Io(e.error)
    }
}
