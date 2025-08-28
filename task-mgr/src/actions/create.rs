use crate::core::domain::{Status, Task};
use uuid::Uuid;

pub fn create(label: String, desc: String, priority: u8) {
    println!(
        "Creating task: \"{}\": {} with priority {}",
        label, desc, priority
    );
    let task = Task {
        id: Uuid::new_v4(),
        label,
        desc,
        priority,
        status: Status::Todo,
    };
    println!("Created task: {:?}", task);
}
