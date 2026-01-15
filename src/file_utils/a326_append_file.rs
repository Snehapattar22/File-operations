use std::fs::OpenOptions;
use std::io::Write;

pub fn append_to_file(path: &str, content: &str) -> bool {
    OpenOptions::new().create(true).append(true).open(path)
        .and_then(|mut f| f.write_all(content.as_bytes())).is_ok()
}
