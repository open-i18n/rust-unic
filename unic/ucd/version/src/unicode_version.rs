// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "unstable", feature(unicode))]

use core::fmt;

#[cfg(feature = "unstable")]
use core::char;

/// Represents a *Unicode Version*, as used for Unicode datasets and specifications.
///
/// TODO: *Unicode Version* is guaranteed to have three integer fields between 0 and 255. We are
/// going to switch over to `u8` after Unicode 11.0.0 release.
///
/// Refs:
/// - <https://www.unicode.org/versions/>
/// - <https://www.unicode.org/L2/L2017/17222.htm#152-C3>
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Default)]
pub struct UnicodeVersion {
    /// Major version.
    pub major: u16,

    /// Minor version.
    pub minor: u16,

    /// Micro (or Update) version.
    pub micro: u16,
}

impl fmt::Display for UnicodeVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.micro)
    }
}

#[cfg(feature = "unstable")]
/// Convert from Rust's internal Unicode Version.
impl From<char::UnicodeVersion> for UnicodeVersion {
    fn from(value: char::UnicodeVersion) -> UnicodeVersion {
        UnicodeVersion {
            major: value.major.into(),
            minor: value.minor.into(),
            micro: value.micro.into(),
        }
    }
}

/// The [Version of The Unicode Standard](https://www.unicode.org/versions/) of the Unicode
/// Character Database in use.
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");

#[cfg(test)]
mod tests {
    #[cfg(feature = "unstable")]
    use core::char;

    use super::UNICODE_VERSION;

    #[test]
    fn validate_version_values() {
        assert!(UNICODE_VERSION.major > 0);

        // Current release schedule of Unicode is to have one Major version update each year, with
        // no Minor updates. We hard-code this internal policy while it stands.
        assert!(UNICODE_VERSION.minor == 0);
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn test_against_rust_internal_unicore_version() {
        use super::UnicodeVersion;
        let core_uni_ver: UnicodeVersion = char::UnicodeVersion.into();
        assert!(core_uni_ver.major >= 10);
        assert!(UNICODE_VERSION.major >= core_uni_ver.major);
    }
}
