use std::fs;
use std::path::Path;

pub fn find_files_recursive(dir: &Path, pattern: &str, out: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                find_files_recursive(&p, pattern, out);
            } else if let Some(n) = p.file_name() {
                if n.to_string_lossy().ends_with(pattern.trim_start_matches('*')) {
                    out.push(p.to_string_lossy().to_string());
                }
            }
        }
    }
}
