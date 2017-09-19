// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code, missing_docs, unconditional_recursion)]


//! # UNIC — UCD — Core
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Core create indicating the version of Unicode Character Database.


use std::fmt;


/// Type of `UNICODE_VERSION` value:
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Default)]
pub struct UnicodeVersion {
    /// Major version.
    pub major: u16,

    /// Minor version.
    pub minor: u16,

    /// Micro (or Update) version.
    pub micro: u16,
}


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");


impl UnicodeVersion {
    /// Human-readable description of the Age property value.
    #[inline]
    pub fn display(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.micro)
    }
}


impl fmt::Display for UnicodeVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}


/// NOTE: (T, T, T) is the type of Rust's internal UNICODE_VERSION until Rust 1.20.
impl<T: Into<u16>> From<(T, T, T)> for UnicodeVersion {
    fn from(value: (T, T, T)) -> UnicodeVersion {
        UnicodeVersion {
            major: value.0.into(),
            minor: value.1.into(),
            micro: value.2.into(),
        }
    }
}


/// NOTE: (T, T, T) is the type of Rust's internal UNICODE_VERSION until Rust 1.20.
impl<T: From<u16>> Into<(T, T, T)> for UnicodeVersion {
    fn into(self) -> (T, T, T) {
        (self.major.into(), self.minor.into(), self.micro.into())
    }
}


// TODO: Add conversion to/from `std::char::UnicodeVersion` whenever it becomes accessible.


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_version_values() {
        assert!(UNICODE_VERSION.major > 0);

        // Current release schedule of Unicode is to have one Major version update each year, with
        // no Minor updates. We hard-code this internal policy while it stans.
        assert!(UNICODE_VERSION.minor == 0);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!(
                "Unicode {}",
                UnicodeVersion {
                    major: 1,
                    minor: 2,
                    micro: 0,
                }
            ),
            "Unicode 1.2.0"
        );
    }

    #[test]
    fn test_against_rust_core_type() {
        // Same type as std::char::UNICODE_VERSION
        let uni_ver: (u64, u64, u64) = (9, 0, 0);
        assert!(uni_ver <= UNICODE_VERSION.into() || uni_ver > UNICODE_VERSION.into());
    }
}
