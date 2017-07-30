mod shared;

mod age;
mod bidi;
mod category;
mod core;
mod normal;

use std::{fs, io};
use std::path::Path;

pub use self::shared::unicode_data::{UnicodeData, UnicodeDataEntry};
pub use self::shared::version::UnicodeVersion;

/// Generate all tables for the UCD component
pub fn generate() -> io::Result<()> {
    println!(">>> Loading UCD Version");
    let ucd_version = shared::version::read_unicode_version()?;
    println!(">>> Loading UCD UnicodeData");
    let unicode_data = shared::unicode_data::read_unicode_data()?;

    let path = Path::new("unic/ucd/age/src/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    age::generate(path, &ucd_version, &unicode_data)?;

    let path = Path::new("unic/ucd/bidi/src/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    bidi::generate(path, &ucd_version, &unicode_data)?;

    let path = Path::new("unic/ucd/category/src/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    category::generate(path, &ucd_version, &unicode_data)?;

    let path = Path::new("unic/ucd/core/src/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    core::generate(path, &ucd_version, &unicode_data)?;

    let path = Path::new("unic/ucd/normal/src/tables");
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path)?;
    normal::generate(path, &ucd_version, &unicode_data)?;

    Ok(())
}
