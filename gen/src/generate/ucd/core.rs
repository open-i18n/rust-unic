// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::path::Path;

use super::UnicodeVersion;

/// Generate tables for the ucd-core crate
pub fn generate<P: AsRef<Path>>(path: P, version: &UnicodeVersion) -> io::Result<()> {
    println!("> unic::ucd::core::tables::unicode_version");
    version.emit(path)?;
    Ok(())
}
