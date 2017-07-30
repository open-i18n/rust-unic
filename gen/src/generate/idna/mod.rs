mod shared;

mod mapping;

use std::{fs, io};
use std::path::Path;

pub use self::shared::version::UnicodeVersion;

/// Generate all tables for the Idna component
pub fn generate() -> io::Result<()> {
    println!(">>> Loading idna Version");
    let idna_version = shared::version::read_unicode_version()?;

    let path = Path::new("unic/idna/mapping/src/tables");
    fs::remove_dir_all(path)?;
    fs::create_dir_all(path)?;
    mapping::generate(path, &idna_version)?;

    Ok(())
}
