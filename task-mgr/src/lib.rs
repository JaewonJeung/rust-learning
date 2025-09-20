// lib.rs is the entry point for the task_mgr as a library.
mod actions;
mod core;
mod fs;
mod test_support;

use core::domain::Task;
use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, error, info};

pub use core::domain::Status;
pub use fs::local::LocalSaver;
pub use fs::port::{LoadError, SaveError, Saver};

#[derive(Debug, Error)]
pub enum TaskManagerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Default)]
pub struct TaskManager<T: Saver> {
    tasks: HashMap<ulid::Ulid, Task>,
    saver: T,
}

impl<T: Saver> TaskManager<T> {
    pub fn new(saver: T) -> Result<Self, TaskManagerError> {
        debug!("Initializing task manager...");
        let task_mgr = match saver.load_file() {
            Ok(tasks) => Ok(Self { tasks, saver }),
            Err(LoadError::FileNotFound) => {
                info!("No existing tasks file found, starting fresh.");
                Ok(TaskManager {
                    tasks: HashMap::new(),
                    saver,
                })
            }
            Err(LoadError::Json(e)) => {
                error!("Failed to parse the tasks data file. Corrupted file.");
                Err(TaskManagerError::Json(e))
            }
            Err(LoadError::Io(e)) => {
                error!("Failed to load tasks file.");
                Err(TaskManagerError::Io(e))
            }
        };
        debug!("Initialized task manager...");
        task_mgr
    }

    pub fn create_task(&mut self, label: String, desc: String, priority: u8) {
        actions::create::create(&mut self.tasks, label, desc, priority, &self.saver);
    }

    pub fn delete_task(&mut self, id: &str) {
        actions::delete::delete(&mut self.tasks, id, &self.saver);
    }

    pub fn list_tasks(&self) {
        actions::list::list(&self.tasks);
    }

    pub fn edit_task(
        &mut self,
        target_id: &str,
        label: String,
        desc: String,
        priority: u8,
        status: Status,
    ) {
        actions::edit::edit(
            &mut self.tasks,
            target_id,
            label,
            desc,
            priority,
            status,
            &self.saver,
        );
    }
}
