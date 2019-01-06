// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode Private-Use Characters
//!
//! *Unicode Private-Use Characters* are the Unicode code-poitns whose interpretation is not
//! specified by a character encoding standard and whose use and interpretation may be determined by
//! private agreement among cooperating users.
//!
//! Since Unicode 2.0.0, the list of Unicode Private-Use characters is *stabilized* by the Unicode
//! Standard and will never change.
//!
//! References:
//! - https://www.unicode.org/faq/private_use.html#private_use
//! - https://www.unicode.org/policies/stability_policy.html#Property_Value

/// Check if the Unicode code-point provided is a *Unicode Private-Use Character*.
///
/// - https://www.unicode.org/faq/private_use.html#private_use
pub fn is_private_use(codepoint: char) -> bool {
    match codepoint {
        '\u{e000}'..='\u{f8ff}' // 6,400 Plane 0 (BMP) chars
        | '\u{f_0000}'..='\u{f_fffd}' // 65,534 Plane 15 chars
        | '\u{10_0000}'..='\u{10_fffd}' // 65,534 Plane 16 chars
        => {
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_private_use;

    #[test]
    fn test_sample_codepoints() {
        // Plane 0 (BMP)
        assert_eq!(is_private_use('\u{0}'), false);
        assert_eq!(is_private_use('\u{20}'), false);
        assert_eq!(is_private_use('\u{41}'), false);
        assert_eq!(is_private_use('\u{80}'), false);
        assert_eq!(is_private_use('\u{200c}'), false);
        assert_eq!(is_private_use('\u{d7ff}'), false);

        assert_eq!(is_private_use('\u{e000}'), true);
        assert_eq!(is_private_use('\u{e001}'), true);
        assert_eq!(is_private_use('\u{f8fe}'), true);
        assert_eq!(is_private_use('\u{f8ff}'), true);
        assert_eq!(is_private_use('\u{f900}'), false);

        assert_eq!(is_private_use('\u{fdcf}'), false);
        assert_eq!(is_private_use('\u{fdd0}'), false);
        assert_eq!(is_private_use('\u{fdd1}'), false);
        assert_eq!(is_private_use('\u{fdee}'), false);
        assert_eq!(is_private_use('\u{fdef}'), false);
        assert_eq!(is_private_use('\u{fdf0}'), false);

        assert_eq!(is_private_use('\u{fff0}'), false);
        assert_eq!(is_private_use('\u{fffc}'), false);
        assert_eq!(is_private_use('\u{fffd}'), false);
        assert_eq!(is_private_use('\u{fffe}'), false);
        assert_eq!(is_private_use('\u{ffff}'), false);

        // Plane 1 (SMP)
        assert_eq!(is_private_use('\u{1_0000}'), false);
        assert_eq!(is_private_use('\u{1_0001}'), false);
        assert_eq!(is_private_use('\u{1_fffd}'), false);
        assert_eq!(is_private_use('\u{1_fffe}'), false);
        assert_eq!(is_private_use('\u{1_ffff}'), false);

        // Plane 14 (SSP)
        assert_eq!(is_private_use('\u{e_0000}'), false);
        assert_eq!(is_private_use('\u{e_0001}'), false);
        assert_eq!(is_private_use('\u{e_fffd}'), false);
        assert_eq!(is_private_use('\u{e_fffe}'), false);
        assert_eq!(is_private_use('\u{e_ffff}'), false);

        // Plane 15 (PUA-A)
        assert_eq!(is_private_use('\u{f_0000}'), true);
        assert_eq!(is_private_use('\u{f_0001}'), true);
        assert_eq!(is_private_use('\u{f_fffd}'), true);
        assert_eq!(is_private_use('\u{f_fffe}'), false);
        assert_eq!(is_private_use('\u{f_ffff}'), false);

        // Plane 16 (PUA-B)
        assert_eq!(is_private_use('\u{10_0000}'), true);
        assert_eq!(is_private_use('\u{10_0001}'), true);
        assert_eq!(is_private_use('\u{10_fffd}'), true);
        assert_eq!(is_private_use('\u{10_fffe}'), false);
        assert_eq!(is_private_use('\u{10_ffff}'), false);
    }
}
