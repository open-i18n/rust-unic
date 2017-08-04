// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    let mut file = File::open(Path::new("data/idna/ReadMe.txt"))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(
        buffer
            .parse()
            .expect("Failed to parse Idna ReadMe (for version)"),
    )
}
