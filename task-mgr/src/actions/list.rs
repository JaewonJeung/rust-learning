use crate::core::domain::{Task, task_pretty_print};
use std::collections::HashMap;

pub fn list(tasks: &HashMap<ulid::Ulid, Task>) {
    for task in tasks.values() {
        task_pretty_print(task);
    }
}
