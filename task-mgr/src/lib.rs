// lib.rs is the entry point for the task_mgr as a library.
// pub mod allows the parent module to access the declared module. In lib.rs case, that's any external crate. The children can also access the declared module.
pub mod actions;
// simple mod declaration allows the current and the children to access the declared module
mod core;
mod fs;

pub use core::domain::Status;
