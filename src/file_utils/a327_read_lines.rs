use std::fs;

pub fn read_file_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap_or_default()
        .lines().map(String::from).collect()
}
