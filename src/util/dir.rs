use std::path::PathBuf;

pub fn get_working_dir() -> PathBuf {
    let path = std::env::current_dir().unwrap();
    path
}
