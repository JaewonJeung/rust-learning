use std::str::FromStr;

use serde_derive::{Deserialize, Serialize};
use ulid::Ulid;

/// normally, you would have an impl of this to have a constructor and getters
/// instead of making fields public, but since this is a DTO, we can skip that for brevity
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub id: Ulid,
    pub label: String,
    pub desc: String,
    pub priority: u8,
    pub status: Status,
}

pub fn task_pretty_print(task: &Task) {
    println!("-------------------------");
    println!("Label: {}", task.label);
    println!("Description: {}", task.desc);
    println!("Priority: {}", task.priority);
    match task.status {
        crate::Status::Todo => println!("Status: TODO"),
        crate::Status::InProgress => println!("Status: IN PROGRESS"),
        crate::Status::Done => println!("Status: DONE"),
    }
    println!("Task ID: {}", task.id);
    println!("-------------------------");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "todo" => Ok(Status::Todo),
            "ip" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("'{}' is not a valid status", s)),
        }
    }
}
