// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # UNIC — UCD — Age
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for `Age` property from Unicode Character Database (UCD)

extern crate unic_ucd_core;


mod tables;
mod traits;

pub use tables::UNICODE_VERSION;
pub use tables::Age;
pub use traits::CharAge;


#[cfg(test)]
mod tests {
    use super::Age;
    use unic_ucd_core::UnicodeVersion;

    #[test]
    fn test_age() {
        use Age::*;

        // ASCII
        assert_eq!(Age::of('\u{0000}'), V1_1);
        assert_eq!(Age::of('\u{0021}'), V1_1);
        assert_eq!(Age::of('\u{0041}'), V1_1);
        assert_eq!(Age::of('\u{007f}'), V1_1);

        assert_eq!(Age::of('\u{0100}'), V1_1);
        assert_eq!(Age::of('\u{01f5}'), V1_1);
        assert_eq!(Age::of('\u{037e}'), V1_1); // start == end
        assert_eq!(Age::of('\u{200c}'), V1_1);

        assert_eq!(Age::of('\u{01f6}'), V3_0);
        assert_eq!(Age::of('\u{01f7}'), V3_0);
        assert_eq!(Age::of('\u{01f9}'), V3_0);

        assert_eq!(Age::of('\u{0860}'), V10_0);
        assert_eq!(Age::of('\u{0866}'), V10_0);
        assert_eq!(Age::of('\u{086a}'), V10_0);

        assert_eq!(Age::of('\u{fffe}'), V1_1);
        assert_eq!(Age::of('\u{ffff}'), V1_1);

        assert_eq!(Age::of('\u{10000}'), V4_0);
        assert_eq!(Age::of('\u{10001}'), V4_0);

        assert_eq!(Age::of('\u{e0100}'), V4_0);
        assert_eq!(Age::of('\u{e0101}'), V4_0);
        assert_eq!(Age::of('\u{e0170}'), V4_0);
        assert_eq!(Age::of('\u{e01ee}'), V4_0);
        assert_eq!(Age::of('\u{e01ef}'), V4_0);

        assert_eq!(Age::of('\u{10000}'), V4_0);

        assert_eq!(Age::of('\u{20000}'), V3_1);

        assert_eq!(Age::of('\u{30000}'), Unassigned);
        assert_eq!(Age::of('\u{40000}'), Unassigned);
        assert_eq!(Age::of('\u{50000}'), Unassigned);
        assert_eq!(Age::of('\u{60000}'), Unassigned);
        assert_eq!(Age::of('\u{70000}'), Unassigned);
        assert_eq!(Age::of('\u{80000}'), Unassigned);
        assert_eq!(Age::of('\u{90000}'), Unassigned);
        assert_eq!(Age::of('\u{a0000}'), Unassigned);
        assert_eq!(Age::of('\u{b0000}'), Unassigned);
        assert_eq!(Age::of('\u{c0000}'), Unassigned);
        assert_eq!(Age::of('\u{d0000}'), Unassigned);
        assert_eq!(Age::of('\u{e0000}'), Unassigned);
        assert_eq!(Age::of('\u{efffd}'), Unassigned);

        assert_eq!(Age::of('\u{efffe}'), V2_0);
        assert_eq!(Age::of('\u{effff}'), V2_0);

        // Priavte-Use Area
        assert_eq!(Age::of('\u{f0000}'), V2_0);
        assert_eq!(Age::of('\u{f0001}'), V2_0);
        assert_eq!(Age::of('\u{ffffe}'), V2_0);
        assert_eq!(Age::of('\u{fffff}'), V2_0);
        assert_eq!(Age::of('\u{100000}'), V2_0);
        assert_eq!(Age::of('\u{100001}'), V2_0);
        assert_eq!(Age::of('\u{10fffe}'), V2_0);
        assert_eq!(Age::of('\u{10ffff}'), V2_0);
    }

    #[test]
    fn test_age_to_unicode_version() {
        assert_eq!(
            Age::V1_1.to_unicode_version(),
            Some(UnicodeVersion(1, 1, 0))
        );
        assert_eq!(Age::Unassigned.to_unicode_version(), None);
    }

    #[test]
    fn test_age_from_unicode_version() {
        assert_eq!(
            Age::from_unicode_version(UnicodeVersion(1, 1, 0)),
            Some(Age::V1_1)
        );
        assert_eq!(Age::from_unicode_version(UnicodeVersion(1, 0, 0)), None);
    }

    #[test]
    fn test_age_cmp() {
        use Age::*;

        assert!(V1_1 == V1_1);
        assert!(V1_1 < V2_0);
        assert!(V1_1 < V3_0);
        assert!(V1_1 < V10_0);
        assert!(V1_1 < Unassigned);

        assert!(V2_0 > V1_1);
        assert!(V2_0 == V2_0);
        assert!(V2_0 < V3_0);
        assert!(V2_0 < V10_0);
        assert!(V2_0 < Unassigned);

        assert!(V3_0 > V1_1);
        assert!(V3_0 > V2_0);
        assert!(V3_0 == V3_0);
        assert!(V3_0 < V10_0);
        assert!(V3_0 < Unassigned);

        assert!(V10_0 > V1_1);
        assert!(V10_0 > V2_0);
        assert!(V10_0 > V3_0);
        assert!(V10_0 == V10_0);
        assert!(V10_0 < Unassigned);

        assert!(Unassigned > V1_1);
        assert!(Unassigned > V2_0);
        assert!(Unassigned > V3_0);
        assert!(Unassigned > V10_0);
        assert!(Unassigned == Unassigned);
    }
}
