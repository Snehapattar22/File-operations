use std::fs;

pub fn create_symlink(target: &str, link: &str) -> bool {
    fs::symlink_metadata(link).is_err() &&
    std::os::windows::fs::symlink_file(target, link).is_ok()
}
