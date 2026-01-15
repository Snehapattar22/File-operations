use std::env;

pub fn change_directory(path: &str) -> bool {
    env::set_current_dir(path).is_ok()
}
