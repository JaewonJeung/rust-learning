use crate::core::domain::{Status, Task};
use ulid::Ulid;

pub fn create(label: String, desc: String, priority: u8) -> Task {
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
    task
}
