use std::fs;
use std::io::Write;

pub fn write_file_lines(path: &str, lines: &[String]) -> bool {
    fs::File::create(path)
        .and_then(|mut f| {
            for l in lines {
                writeln!(f, "{}", l)?;
            }
            Ok(())
        }).is_ok()
}
