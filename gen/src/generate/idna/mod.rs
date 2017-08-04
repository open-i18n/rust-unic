// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
