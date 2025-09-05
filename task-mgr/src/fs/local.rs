use crate::fs::port::{LoadError, SaveError, Saver};
use serde::{Serialize, de::DeserializeOwned};
use std::fs::File;
use std::io::prelude::*;
use tempfile::NamedTempFile;

pub struct LocalSaver {
    file_path: String,
}

impl Saver for LocalSaver {
    fn save<T: Serialize>(&self, tasks: &T) -> Result<(), SaveError> {
        self.save_to_temp_file(tasks)?.persist(&self.file_path)?;
        Ok(())
    }

    fn load_file<T: DeserializeOwned>(&self) -> Result<T, LoadError> {
        let file = File::open(&self.file_path)?;
        Ok(serde_json::from_reader(file)?)
    }
}

impl LocalSaver {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    fn save_to_temp_file<T: Serialize>(
        &self,
        obj: &T,
    ) -> Result<tempfile::NamedTempFile, SaveError> {
        let mut temp_file = NamedTempFile::new()?;
        let task_json = serde_json::to_string_pretty(obj)?;

        temp_file.write_all(task_json.as_bytes())?;
        temp_file.flush().unwrap();
        Ok(temp_file)
    }
}
