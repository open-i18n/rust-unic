use std::io;
use std::path::Path;

use super::{UnicodeData, UnicodeVersion};

/// Generate tables for the ucd-core crate
pub fn generate<P: AsRef<Path>>(
    path: P,
    version: &UnicodeVersion,
    _: &UnicodeData,
) -> io::Result<()> {
    println!("> unic::ucd::core::tables::unicode_version");
    version.emit(path)?;
    Ok(())
}
