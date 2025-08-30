// lib.rs is the entry point for the task_mgr as a library.
// pub mod allows the parent module to access the declared module. In lib.rs case, that's any external crate. The children can also access the declared module.
mod actions;
// simple mod declaration allows the current and the children to access the declared module
mod core;
mod fs;

pub use core::domain::Status;
use core::domain::Task;
use fs::local::{LoadError, load_file, save_to_file};
use std::fmt;

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
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Result<Self, TaskManagerError> {
        match load_file(TASKS_FILE) {
            Ok(tasks) => Ok(Self { tasks }),
            Err(LoadError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("No existing tasks file found, starting fresh.");
                Ok(Self::default())
            }
            Err(LoadError::Json(e)) => {
                eprintln!("Failed to parse the tasks data file. Corrupted file.");
                Err(TaskManagerError::Json(e))
            }
            Err(LoadError::Io(e)) => {
                eprintln!("Failed to load tasks file.");
                Err(TaskManagerError::Io(e))
            }
        }
    }

    pub fn create_task(&mut self, label: String, desc: String, priority: u8) {
        let task = actions::create::create(label, desc, priority);
        self.tasks.push(task);
        save_to_file(TASKS_FILE, &self.tasks);
    }
}
