mod shared;

mod age;
mod bidi;
mod category;
mod core;
mod normal;

use std::io;

pub use self::shared::unicode_data::{UnicodeData, UnicodeDataEntry};
pub use self::shared::version::UnicodeVersion;

pub fn generate_all<I, S>(crates: I) -> io::Result<()>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    println!(">>> Loading UCD Version");
    let ucd_version = shared::version::read_unicode_version()?;
    println!(">>> Loading UCD UnicodeData");
    let unicode_data = shared::unicode_data::read_unicode_data()?;

    for krate in crates {
        let path = super::tables_path(krate.as_ref());
        match krate.as_ref() {
            "unic-ucd-age" => {
                age::generate(
                    path,
                    &ucd_version,
                    &unicode_data,
                )?;
            }
            "unic-ucd-bidi" => {
                bidi::generate(
                    path,
                    &ucd_version,
                    &unicode_data,
                )?;
            }
            "unic-ucd-category" => {
                category::generate(
                    path,
                    &ucd_version,
                    &unicode_data,
                )?;
            }
            "unic-ucd-core" => {
                core::generate(
                    path,
                    &ucd_version,
                    &unicode_data,
                )?;
            }
            "unic-ucd-normal" => {
                normal::generate(
                    path,
                    &ucd_version,
                    &unicode_data,
                )?;
            }
            _ => (),
        }
    }

    Ok(())
}
