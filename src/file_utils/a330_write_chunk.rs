use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};

pub fn write_file_chunk(path: &str, offset: u64, data: &[u8]) -> bool {
    OpenOptions::new().write(true).open(path)
        .and_then(|mut f| {
            f.seek(SeekFrom::Start(offset))?;
            f.write_all(data)
        }).is_ok()
}
