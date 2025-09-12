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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::memory::MemorySaver;

    #[test]
    fn test_create_task() {
        let mut tasks = HashMap::new();
        let saver = MemorySaver::new();
        let test_label = "Test Task";
        let test_desc = "This is a test task.";
        let test_priority = 1;

        create(
            &mut tasks,
            test_label.to_string(),
            test_desc.to_string(),
            test_priority,
            &saver,
        );

        assert!(!tasks.is_empty());
        for (_id, task) in tasks.iter() {
            assert_eq!(task.label, test_label);
            assert_eq!(task.desc, test_desc);
            assert_eq!(task.priority, test_priority);
            assert_eq!(task.status, Status::Todo);
        }
    }
}
