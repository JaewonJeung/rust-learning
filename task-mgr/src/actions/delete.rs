use crate::core::domain::Task;
use crate::fs::port::Saver;
use std::collections::HashMap;
use tracing::{error, info};

pub fn delete<T: Saver>(tasks: &mut HashMap<ulid::Ulid, Task>, id: &str, saver: &T) {
    let Ok(ulid) = ulid::Ulid::from_string(id) else {
        error!("Invalid ID format. Not a ULID: {}", id);
        return;
    };

    if tasks.remove(&ulid).is_none() {
        info!("No task found with ID: {}", id);
        return;
    }

    if saver.save(tasks).is_err() {
        info!("Failed to save tasks after deletion");
    } else {
        info!("Deleted task with ID: {}", id);
    }
}
