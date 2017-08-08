mod tests;

use std::{fs, io};
use std::path::Path;

/// Generate all tables for the Idna component
pub fn generate() -> io::Result<()> {
    let path = Path::new("unic/normal/tests/tables");
    fs::remove_dir_all(path)?;
    fs::create_dir_all(path)?;
    tests::generate(path)?;

    Ok(())
}
