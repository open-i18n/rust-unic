pub mod core;

use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write};

use generate::PREAMBLE;

use regex::Regex;

lazy_static! {
    /// Version of the local UCD files
    static ref UNICODE_VERSION: (u16, u16, u16) = {
        let mut readme = File::open(Path::new("data/ucd/ReadMe.txt"))
            .expect("Failed to open UCD ReadMe. Did you run -u?");
        let mut buffer = String::new();
        readme.read_to_string(&mut buffer)
            .expect("Failed to read UCD ReadMe. Did you run -u?");
        // This is a valid regex and should therefor never panic.
        let pattern = Regex::new(r"for Version (\d+).(\d+).(\d+)").unwrap();
        let captures = pattern.captures(&buffer)
            .expect("Regex didn't match UCD ReadMe. Did it download correctly?");
        (
            // These unwrap should never panic due to the regex.
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
            captures.get(3).unwrap().as_str().parse().unwrap(),
        )
    };
}

impl UNICODE_VERSION {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("unicode_version.rsv"))?;
        write!(file, "{}", PREAMBLE)?;
        writeln!(
            file,
            "UnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
            self.0, self.1, self.2,
        )?;
        Ok(())
    }
}
