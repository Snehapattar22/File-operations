use std::env;

pub fn get_current_directory() -> Option<String> {
    env::current_dir().ok()
        .map(|p| p.to_string_lossy().to_string())
}
