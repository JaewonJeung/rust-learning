use crate::fs::port::{LoadError, SaveError, Saver};
use serde::{Serialize, de::DeserializeOwned};
use std::cell::RefCell;

// Mainly used for mocking in tests as a test double

#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct MemorySaver {
    blob: RefCell<String>,
}

impl Saver for MemorySaver {
    fn save<T: Serialize>(&self, tasks: &T) -> Result<(), SaveError> {
        self.blob.replace(serde_json::to_string_pretty(tasks)?);
        Ok(())
    }

    fn load_file<T: DeserializeOwned>(&self) -> Result<T, LoadError> {
        Ok(serde_json::from_str(&self.blob.borrow())?)
    }
}

#[allow(dead_code)]
impl MemorySaver {
    pub fn new() -> Self {
        Default::default()
    }
}
