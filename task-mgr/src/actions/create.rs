use crate::core::domain::{Status, Task};
use crate::fs::port::Saver;
use std::collections::HashMap;
use tracing::info;
use ulid::Ulid;

pub fn create<T: Saver>(
    tasks: &mut HashMap<ulid::Ulid, Task>,
    label: String,
    desc: String,
    priority: u8,
    saver: &T,
) {
    let task = Task {
        id: Ulid::new(),
        label,
        desc,
        priority,
        status: Status::Todo,
    };
    info!("Created task: {}", task.id);
    tasks.insert(task.id, task);
    if saver.save(tasks).is_err() {
        info!("Failed to save tasks.");
    }
}
