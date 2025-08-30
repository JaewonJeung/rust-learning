use crate::core::domain::{Status, Task, task_pretty_print};
use crate::fs::local::save_to_file;

pub fn edit(
    tasks: &mut Vec<Task>,
    tasks_file: &str,
    target_id: &str,
    label: String,
    desc: String,
    priority: u8,
    status: Status,
) {
    if let Ok(target_ulid) = ulid::Ulid::from_string(target_id) {
        let task_idx = tasks.iter_mut().position(|task| task.id == target_ulid);
        if let Some(task_idx) = task_idx {
            tasks[task_idx].label = label;
            tasks[task_idx].desc = desc;
            tasks[task_idx].priority = priority;
            tasks[task_idx].status = status;
            save_to_file(tasks_file, tasks);
            println!("Edited task");
            task_pretty_print(&tasks[task_idx]);
        } else {
            eprintln!("No task found with ID: {}", target_id);
        }
    } else {
        eprintln!("Invalid ID format. Not a ULID: {}", target_id);
    }
}
