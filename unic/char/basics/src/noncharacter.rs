// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode Noncharacters
//!
//! *Unicode Noncharacters* are the Unicode code-poitns permanently reserved in the Unicode Standard
//! for internal use in applications. These code-points should not be used in text storage or
//! exchange.
//!
//! Since Unicode 3.1.0, the list of 66 Unicode Noncharacters is *stabilized* by the Unicode
//! Standard and will never change.
//!
//! References:
//! - https://www.unicode.org/faq/private_use.html#noncharacters
//! - https://www.unicode.org/policies/stability_policy.html#Property_Value

/// Check if the Unicode code-point provided is a *Unicode Noncharacter*.
///
/// - https://www.unicode.org/faq/private_use.html#noncharacters
pub fn is_noncharacter(codepoint: char) -> bool {
    match codepoint {
        '\u{fdd0}'..='\u{fdef}'
        | '\u{fffe}'..='\u{ffff}'
        | '\u{1_fffe}'..='\u{1_ffff}'
        | '\u{2_fffe}'..='\u{2_ffff}'
        | '\u{3_fffe}'..='\u{3_ffff}'
        | '\u{4_fffe}'..='\u{4_ffff}'
        | '\u{5_fffe}'..='\u{5_ffff}'
        | '\u{6_fffe}'..='\u{6_ffff}'
        | '\u{7_fffe}'..='\u{7_ffff}'
        | '\u{8_fffe}'..='\u{8_ffff}'
        | '\u{9_fffe}'..='\u{9_ffff}'
        | '\u{A_fffe}'..='\u{A_ffff}'
        | '\u{B_fffe}'..='\u{B_ffff}'
        | '\u{C_fffe}'..='\u{C_ffff}'
        | '\u{D_fffe}'..='\u{D_ffff}'
        | '\u{E_fffe}'..='\u{E_ffff}'
        | '\u{F_fffe}'..='\u{F_ffff}'
        | '\u{10_fffe}'..='\u{10_ffff}' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_noncharacter;

    #[test]
    fn test_sample_codepoints() {
        // Plane 0 (BMP)
        assert_eq!(is_noncharacter('\u{0}'), false);
        assert_eq!(is_noncharacter('\u{20}'), false);
        assert_eq!(is_noncharacter('\u{41}'), false);
        assert_eq!(is_noncharacter('\u{80}'), false);
        assert_eq!(is_noncharacter('\u{200c}'), false);
        assert_eq!(is_noncharacter('\u{d7ff}'), false);

        assert_eq!(is_noncharacter('\u{e000}'), false);
        assert_eq!(is_noncharacter('\u{e001}'), false);
        assert_eq!(is_noncharacter('\u{f8fe}'), false);
        assert_eq!(is_noncharacter('\u{f8ff}'), false);
        assert_eq!(is_noncharacter('\u{f900}'), false);

        assert_eq!(is_noncharacter('\u{fdcf}'), false);
        assert_eq!(is_noncharacter('\u{fdd0}'), true);
        assert_eq!(is_noncharacter('\u{fdd1}'), true);
        assert_eq!(is_noncharacter('\u{fdee}'), true);
        assert_eq!(is_noncharacter('\u{fdef}'), true);
        assert_eq!(is_noncharacter('\u{fdf0}'), false);

        assert_eq!(is_noncharacter('\u{fff0}'), false);
        assert_eq!(is_noncharacter('\u{fffc}'), false);
        assert_eq!(is_noncharacter('\u{fffd}'), false);
        assert_eq!(is_noncharacter('\u{fffe}'), true);
        assert_eq!(is_noncharacter('\u{ffff}'), true);

        // Plane 1 (SMP)
        assert_eq!(is_noncharacter('\u{1_0000}'), false);
        assert_eq!(is_noncharacter('\u{1_0001}'), false);
        assert_eq!(is_noncharacter('\u{1_fffd}'), false);
        assert_eq!(is_noncharacter('\u{1_fffe}'), true);
        assert_eq!(is_noncharacter('\u{1_ffff}'), true);

        // Plane 14 (SSP)
        assert_eq!(is_noncharacter('\u{e_0000}'), false);
        assert_eq!(is_noncharacter('\u{e_0001}'), false);
        assert_eq!(is_noncharacter('\u{e_fffd}'), false);
        assert_eq!(is_noncharacter('\u{e_fffe}'), true);
        assert_eq!(is_noncharacter('\u{e_ffff}'), true);

        // Plane 15 (PUA-A)
        assert_eq!(is_noncharacter('\u{f_0000}'), false);
        assert_eq!(is_noncharacter('\u{f_0001}'), false);
        assert_eq!(is_noncharacter('\u{f_fffd}'), false);
        assert_eq!(is_noncharacter('\u{f_fffe}'), true);
        assert_eq!(is_noncharacter('\u{f_ffff}'), true);

        // Plane 16 (PUA-B)
        assert_eq!(is_noncharacter('\u{10_0000}'), false);
        assert_eq!(is_noncharacter('\u{10_0001}'), false);
        assert_eq!(is_noncharacter('\u{10_fffd}'), false);
        assert_eq!(is_noncharacter('\u{10_fffe}'), true);
        assert_eq!(is_noncharacter('\u{10_ffff}'), true);
    }
}
