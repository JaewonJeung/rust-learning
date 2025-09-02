use crate::core::domain::Task;
use crate::fs::local::save_to_file;
use std::collections::HashMap;
use tracing::{error, info};

pub fn delete(tasks: &mut HashMap<ulid::Ulid, Task>, file_path: &str, id: &str) {
    match ulid::Ulid::from_string(id) {
        Ok(ulid) => {
            if tasks.remove(&ulid).is_some() {
                save_to_file(file_path, tasks);
                info!("Deleted task with ID: {}", id);
            } else {
                info!("No task found with ID: {}", id);
            }
        }
        Err(_) => {
            error!("Invalid ID format. Not a ULID: {}", id);
        }
    }
}
