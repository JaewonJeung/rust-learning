use crate::core::domain::Task;
use crate::fs::local::save_to_file;
use std::collections::HashMap;

pub fn delete(tasks: &mut HashMap<ulid::Ulid, Task>, file_path: &str, id: &str) {
    if let Ok(ulid) = ulid::Ulid::from_string(id) {
        tasks.remove(&ulid);
        save_to_file(file_path, tasks);
    } else {
        eprintln!("Invalid ID format. Not a ULID: {}", id);
    }
}
