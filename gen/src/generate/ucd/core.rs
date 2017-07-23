use std::io;
use std::path::Path;

use super::{UnicodeData, UnicodeVersion};

pub fn generate<P: AsRef<Path>>(
    path: P,
    version: &UnicodeVersion,
    _: &UnicodeData,
) -> io::Result<()> {
    version.emit(path)?;
    println!("> unic::ucd::core::tables::unicode_version");
    Ok(())
}
