// lib.rs is the entry point for the task_mgr as a library.
mod actions;
mod core;
mod fs;

use core::domain::Task;
use fs::local::{LoadError, load_file};
use std::collections::HashMap;
use std::fmt;
use tracing::{debug, error, info};

pub use core::domain::Status;

// TODO use config file for this
const TASKS_FILE: &str = "tasks.json";

pub enum TaskManagerError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for TaskManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskManagerError::Io(e) => write!(f, "I/O error: {}", e),
            TaskManagerError::Json(e) => write!(f, "JSON error: {}", e),
        }
    }
}

#[derive(Default)]
pub struct TaskManager {
    tasks: HashMap<ulid::Ulid, Task>,
}

impl TaskManager {
    pub fn new() -> Result<Self, TaskManagerError> {
        debug!("Initializing task manager...");
        let task_mgr = match load_file(TASKS_FILE) {
            Ok(tasks) => Ok(Self { tasks }),
            Err(LoadError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {
                info!("No existing tasks file found, starting fresh.");
                Ok(Self::default())
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
        debug!("Initializing task manager...");
        task_mgr
    }

    pub fn create_task(&mut self, label: String, desc: String, priority: u8) {
        actions::create::create(&mut self.tasks, TASKS_FILE, label, desc, priority);
    }

    pub fn delete_task(&mut self, id: &str) {
        actions::delete::delete(&mut self.tasks, TASKS_FILE, id);
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
            TASKS_FILE,
            target_id,
            label,
            desc,
            priority,
            status,
        );
    }
}
