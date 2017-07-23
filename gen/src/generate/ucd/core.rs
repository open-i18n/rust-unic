use std::io;
use std::path::Path;

pub fn generate<P: AsRef<Path>>(path: P) -> io::Result<()> {
    super::read_unicode_version()?.emit(path)?;
    println!("> unicode_version");
    Ok(())
}
