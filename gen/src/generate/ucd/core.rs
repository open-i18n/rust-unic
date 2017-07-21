use std::io;
use std::path::Path;

pub fn generate<P: AsRef<Path>>(path: P) -> io::Result<()> {
    super::UNICODE_VERSION.emit(path)?;
    Ok(())
}
