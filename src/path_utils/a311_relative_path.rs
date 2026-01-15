use std::path::{Path, PathBuf};

pub fn getelative_path(from: &str, to: &str) -> Option<String> {
    let from = Path::new(from).canonicalize().ok()?;
    let to = Path::new(to).canonicalize().ok()?;

    let mut result = PathBuf::new();
    let from_parts: Vec<_> = from.components().collect();
    let to_parts: Vec<_> = to.components().collect();

    let common = from_parts.iter().zip(&to_parts)
        .take_while(|(a,b)| a == b)
        .count();

    for _ in common..from_parts.len() {
        result.push("..");
    }
    for part in &to_parts[common..] {
        result.push(part.as_os_str());
    }

    Some(result.to_string_lossy().to_string())
}
