use crate::core::domain::{Task, task_pretty_print};

pub fn list(tasks: &Vec<Task>) {
    for task in tasks {
        task_pretty_print(task);
    }
}
