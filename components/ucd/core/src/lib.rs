// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code, missing_docs)]

//! # UNIC — UCD — Core
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Core create indicating the version of Unicode Character Database.


/// Type of `UNICODE_VERSION` value:
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct UnicodeVersion(
    pub u32, // Major version
    pub u32, // Minor version
    pub u32 // Micro (or Update) version
);


impl UnicodeVersion {
    /// Major version
    pub fn major(&self) -> u32 {
        self.0
    }

    /// Minor version
    pub fn minor(&self) -> u32 {
        self.1
    }

    /// Micro (or Update) version
    pub fn micro(&self) -> u32 {
        self.2
    }
}


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("unicode_version.rsv");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_versions() {
        assert!(UNICODE_VERSION.major() > 0);

        // Current release schedule of Unicode is to have one Major version update each year, with
        // no Minor updates. We hard-code this internal policy while it stans.
        assert!(UNICODE_VERSION.minor() == 0);
    }
}
