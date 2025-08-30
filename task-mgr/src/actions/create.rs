use crate::core::domain::{Status, Task};
use crate::fs::local::save_to_file;
use ulid::Ulid;

pub fn create(tasks: &mut Vec<Task>, file_path: &str, label: String, desc: String, priority: u8) {
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
    tasks.push(task);
    save_to_file(file_path, tasks);
}
