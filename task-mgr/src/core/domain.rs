use clap::ValueEnum;
use uuid::Uuid;

/// normally, you would have an impl of this to have a constructor and getters
/// instead of making fields public, but since this is a DTO, we can skip that for brevity
#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub label: String,
    pub desc: String,
    pub priority: u8,
    pub status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}
