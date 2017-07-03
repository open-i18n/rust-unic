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
    pub u16, // Major version
    pub u16, // Minor version
    pub u16 // Micro (or Update) version
);


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("tables/unicode_version.rsv");


impl UnicodeVersion {
    /// Major version
    pub fn major(&self) -> u16 {
        self.0
    }

    /// Minor version
    pub fn minor(&self) -> u16 {
        self.1
    }

    /// Micro (or Update) version
    pub fn micro(&self) -> u16 {
        self.2
    }
}

impl<T: Into<u16>> From<(T, T, T)> for UnicodeVersion {
    fn from(t: (T, T, T)) -> UnicodeVersion {
        UnicodeVersion(t.0.into(), t.1.into(), t.2.into())
    }
}

impl<T: From<u16>> Into<(T, T, T)> for UnicodeVersion {
    fn into(self) -> (T, T, T) {
        (self.0.into(), self.1.into(), self.2.into())
    }
}



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

    #[test]
    fn test_against_rust_core_type() {
        // Same type as std::char::UNICODE_VERSION
        let uni_ver: (u64, u64, u64) = (9, 0, 0);
        assert!(uni_ver <= UNICODE_VERSION.into() || uni_ver > UNICODE_VERSION.into());
    }
}
