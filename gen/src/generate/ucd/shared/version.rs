use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use generate::PREAMBLE;

use regex::Regex;

pub struct UnicodeVersion(u16, u16, u16);

impl UnicodeVersion {
    /// Emit `unicode_version.rsv` into a directory.
    pub fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("unicode_version.rsv"))?;
        writeln!(
            file,
            "{}\nUnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
            PREAMBLE,
            self.0,
            self.1,
            self.2,
        )?;
        Ok(())
    }
}

impl FromStr for UnicodeVersion {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"for Version (\d+).(\d+).(\d+)").unwrap();
        }
        REGEX
            .captures(str)
            .map(|m| {
                UnicodeVersion(
                    m[1].parse().unwrap(),
                    m[2].parse().unwrap(),
                    m[3].parse().unwrap(),
                )
            })
            .ok_or(())
    }
}

pub fn read_unicode_version() -> io::Result<UnicodeVersion> {
    let mut file = File::open(Path::new("data/ucd/ReadMe.txt"))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.parse().unwrap())
}
