use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn read_file_chunk(path: &str, offset: u64, len: usize) -> Option<Vec<u8>> {
    let mut f = File::open(path).ok()?;
    f.seek(SeekFrom::Start(offset)).ok()?;
    let mut buf = vec![0; len];
    f.read_exact(&mut buf).ok()?;
    Some(buf)
}
