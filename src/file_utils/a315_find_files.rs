use std::fs;

pub fn find_files(dir: &str, pattern: &str) -> Vec<String> {
    fs::read_dir(dir).map(|e|
        e.filter_map(|x| {
            let name = x.ok()?.file_name().to_string_lossy().to_string();
            if name.ends_with(pattern.trim_start_matches('*')) {
                Some(name)
            } else { None }
        }).collect()
    ).unwrap_or_default()
}
