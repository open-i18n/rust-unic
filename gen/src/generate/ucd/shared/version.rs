use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use generate::PREAMBLE;

use regex::Regex;

lazy_static! {
    /// Version of the local UCD files
    pub static ref UNICODE_VERSION: (u16, u16, u16) = {
        let mut readme = File::open(Path::new("data/ucd/ReadMe.txt"))
            .expect("Failed to open UCD ReadMe. Did you run -u?");
        let mut buffer = String::new();
        readme.read_to_string(&mut buffer)
            .expect("Failed to read UCD ReadMe. Did you run -u?");
        let pattern = Regex::new(r"for Version (\d+).(\d+).(\d+)").unwrap();
        let captures = pattern.captures(&buffer)
            .expect("Regex didn't match UCD ReadMe. Did it download correctly?");
        (
            captures[1].parse().unwrap(),
            captures[2].parse().unwrap(),
            captures[3].parse().unwrap(),
        )
    };
}

impl UNICODE_VERSION {
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
