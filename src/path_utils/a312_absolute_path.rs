use std::env;
use std::path::Path;

pub fn get_absolute_path(path: &str) -> Option<String> {
    let p = Path::new(path);
    if p.is_absolute() {
        Some(p.to_string_lossy().to_string())
    } else {
        env::current_dir().ok()?.join(p).canonicalize().ok()
            .map(|v| v.to_string_lossy().to_string())
    }
}
