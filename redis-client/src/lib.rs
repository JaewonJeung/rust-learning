#[macro_export]
macro_rules! dlog {
    ($($t:tt)*) => {{
        eprintln!("[redis-client][{}:{}] {}", file!(), line!(), format!($($t)*));
    }};
}

pub mod connection;
pub mod frame;
pub mod parse;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;
