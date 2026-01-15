use std::fs;

pub fn check_file_permissions(path: &str, mode: &str) -> Option<bool> {
    let meta = fs::metadata(path).ok()?;
    let perm = meta.permissions();

    if mode.contains('r') && perm.readonly() {
        return Some(false);
    }

    Some(true)
}
