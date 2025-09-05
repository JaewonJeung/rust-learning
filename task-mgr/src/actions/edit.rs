use crate::core::domain::{Status, Task, task_pretty_print};
use crate::fs::port::Saver;
use std::collections::HashMap;
use tracing::{error, info};

pub fn edit<T: Saver>(
    tasks: &mut HashMap<ulid::Ulid, Task>,
    target_id: &str,
    label: String,
    desc: String,
    priority: u8,
    status: Status,
    saver: &T,
) {
    let Ok(target_ulid) = ulid::Ulid::from_string(target_id) else {
        error!("Invalid ID format. Not a ULID: {}", target_id);
        return;
    };

    let Some(task) = tasks.get_mut(&target_ulid) else {
        error!("No task found with ID: {}", target_id);
        return;
    };

    task.label = label;
    task.desc = desc;
    task.priority = priority;
    task.status = status;

    if saver.save(tasks).is_err() {
        info!("Failed to save tasks after editing");
        return;
    }
    info!("Edited task");

    if let Some(task) = tasks.get(&target_ulid) {
        task_pretty_print(task);
    }
}
