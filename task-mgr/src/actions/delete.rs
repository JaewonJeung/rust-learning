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

// I guess ideally the delete function would return a Result type

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Status;
    use crate::fs::port::Saver;
    use crate::test_support::memory::MemorySaver;
    use rstest::{fixture, rstest};
    use std::collections::HashMap;
    use ulid::Ulid;

    #[fixture]
    fn tasks() -> HashMap<ulid::Ulid, Task> {
        HashMap::new()
    }

    #[fixture]
    fn saver() -> MemorySaver {
        MemorySaver::new()
    }

    #[fixture]
    fn task() -> Task {
        Task {
            id: Ulid::new(),
            label: "Test Task".to_string(),
            desc: "This is a test task.".to_string(),
            priority: 1,
            status: Status::Todo,
        }
    }

    #[rstest]
    fn test_delete_existing_task(
        mut tasks: HashMap<ulid::Ulid, Task>,
        saver: impl Saver,
        task: Task,
    ) {
        let id_str = task.id.to_string();
        tasks.insert(task.id, task);
        delete(&mut tasks, &id_str, &saver);
        assert!(tasks.is_empty());
    }

    #[rstest]
    fn test_delete_nonexistent_task(
        mut tasks: HashMap<ulid::Ulid, Task>,
        saver: impl Saver,
        task: Task,
    ) {
        let task_id = task.id;
        tasks.insert(task.id, task);
        let bogo_id = Ulid::new().to_string();
        delete(&mut tasks, &bogo_id, &saver);
        assert!(!tasks.is_empty());
        assert!(tasks.contains_key(&task_id));
    }
}
