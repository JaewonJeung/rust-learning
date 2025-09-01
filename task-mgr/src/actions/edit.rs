use crate::core::domain::{Status, Task, task_pretty_print};
use crate::fs::local::save_to_file;
use std::collections::HashMap;

pub fn edit(
    tasks: &mut HashMap<ulid::Ulid, Task>,
    tasks_file: &str,
    target_id: &str,
    label: String,
    desc: String,
    priority: u8,
    status: Status,
) {
    if let Ok(target_ulid) = ulid::Ulid::from_string(target_id) {
        if let Some(task) = tasks.get_mut(&target_ulid) {
            task.label = label;
            task.desc = desc;
            task.priority = priority;
            task.status = status;
            save_to_file(tasks_file, tasks);
            println!("Edited task");
        } else {
            eprintln!("No task found with ID: {}", target_id);
        }

        if let Some(task) = tasks.get(&target_ulid) {
            task_pretty_print(task);
        }
    } else {
        eprintln!("Invalid ID format. Not a ULID: {}", target_id);
    }
}
