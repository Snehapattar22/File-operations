use std::fs;
use std::time::SystemTime;

pub fn get_file_last_modified(path: &str) -> Option<SystemTime> {
    fs::metadata(path).ok()?.modified().ok()
}
