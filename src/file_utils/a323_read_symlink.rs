use std::fs;

pub fn read_symlink(path: &str) -> Option<String> {
    fs::read_link(path).ok()
        .map(|p| p.to_string_lossy().to_string())
}
