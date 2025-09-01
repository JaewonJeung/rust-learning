use crate::core::domain::{Status, Task};
use crate::fs::local::save_to_file;
use std::collections::HashMap;
use ulid::Ulid;

pub fn create(
    tasks: &mut HashMap<ulid::Ulid, Task>,
    file_path: &str,
    label: String,
    desc: String,
    priority: u8,
) {
    let task = Task {
        id: Ulid::new(),
        label,
        desc,
        priority,
        status: Status::Todo,
    };
    println!(
        "Created task: {}",
        serde_json::to_string_pretty(&task).unwrap()
    );
    tasks.insert(task.id, task);
    save_to_file(file_path, tasks);
}
