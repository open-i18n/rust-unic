mod shared;

mod mapping;

use std::io;

pub use self::shared::version::UnicodeVersion;

pub fn generate_all<I, S>(crates: I) -> io::Result<()>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    println!(">>> Loading idna Version");
    let idna_version = shared::version::read_unicode_version()?;

    for krate in crates {
        let path = super::tables_path(krate.as_ref());
        #[cfg_attr(feature = "cargo-clippy", allow(single_match))]
        match krate.as_ref() {
            "unic-idna-mapping" => {
                mapping::generate(path, &idna_version)?;
            }
            _ => (),
        }
    }

    Ok(())
}
