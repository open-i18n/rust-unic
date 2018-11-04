// Copyright 2016 The rust-url developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use unic_normal::StrNormalForm;
use unic_ucd_bidi::{bidi_class, BidiClass};
use unic_ucd_normal::is_combining_mark;

use mapping::Mapping;
use punycode;

/// Prefix used in Punycode encoding.
pub static PUNYCODE_PREFIX: &'static str = "xn--";

fn map_char(codepoint: char, flags: Flags, output: &mut String, errors: &mut Vec<Error>) {
    match Mapping::of(codepoint) {
        Mapping::Valid => output.push(codepoint),
        Mapping::Ignored => {}
        Mapping::Mapped(slice) => output.push_str(slice),
        Mapping::Deviation(slice) => {
            if flags.transitional_processing {
                output.push_str(slice)
            } else {
                output.push(codepoint)
            }
        }
        Mapping::Disallowed => {
            errors.push(Error::DissallowedCharacter);
            output.push(codepoint);
        }
        Mapping::DisallowedStd3Valid => {
            if flags.use_std3_ascii_rules {
                errors.push(Error::DissallowedByStd3AsciiRules);
            }
            output.push(codepoint)
        }
        Mapping::DisallowedStd3Mapped(slice) => {
            if flags.use_std3_ascii_rules {
                errors.push(Error::DissallowedMappedInStd3);
            }
            output.push_str(slice)
        }
    }
}

// http://tools.ietf.org/html/rfc5893#section-2
fn passes_bidi(label: &str, is_bidi_domain: bool) -> bool {
    use self::bidi_class::abbr_names::*;

    // Rule 0: Bidi Rules apply to Bidi Domain Names: a name with at least one RTL label.  A label
    // is RTL if it contains at least one character of bidi class R, AL or AN.
    if !is_bidi_domain {
        return true;
    }

    let mut chars = label.chars();
    let first_char_class = match chars.next() {
        Some(c) => BidiClass::of(c),
        None => return true, // empty string
    };

    match first_char_class {
        // LTR label
        L => {
            // Rule 5
            while let Some(c) = chars.next() {
                if !matches!(BidiClass::of(c), L | EN | ES | CS | ET | ON | BN | NSM) {
                    return false;
                }
            }

            // Rule 6
            // must end in L or EN followed by 0 or more NSM
            let mut rev_chars = label.chars().rev();
            let mut last_non_nsm = rev_chars.next();
            loop {
                match last_non_nsm {
                    Some(c) if BidiClass::of(c) == NSM => {
                        last_non_nsm = rev_chars.next();
                        continue;
                    }
                    _ => {
                        break;
                    }
                }
            }
            match last_non_nsm {
                Some(c) if BidiClass::of(c) == L || BidiClass::of(c) == EN => {}
                Some(_) => {
                    return false;
                }
                _ => {}
            }
        }

        // RTL label
        R | AL => {
            let mut found_en = false;
            let mut found_an = false;

            // Rule 2
            for c in chars {
                let char_class = BidiClass::of(c);

                if char_class == EN {
                    found_en = true;
                }
                if char_class == AN {
                    found_an = true;
                }

                if !matches!(char_class, R | AL | AN | EN | ES | CS | ET | ON | BN | NSM) {
                    return false;
                }
            }

            // Rule 3
            let mut rev_chars = label.chars().rev();
            let mut last = rev_chars.next();
            loop {
                // must end in L or EN followed by 0 or more NSM
                match last {
                    Some(c) if BidiClass::of(c) == NSM => {
                        last = rev_chars.next();
                        continue;
                    }
                    _ => {
                        break;
                    }
                }
            }
            match last {
                Some(c) if matches!(BidiClass::of(c), R | AL | EN | AN) => {}
                _ => {
                    return false;
                }
            }

            // Rule 4
            if found_an && found_en {
                return false;
            }
        }

        // Rule 1: Should start with L or R/AL
        _ => {
            return false;
        }
    }

    true
}

// https://www.unicode.org/reports/tr46/#Validity_Criteria
#[cfg_attr(feature = "cargo-clippy", allow(if_same_then_else))]
fn validate(label: &str, is_bidi_domain: bool, flags: Flags, errors: &mut Vec<Error>) {
    let first_char = label.chars().next();

    if first_char == None {
        // Empty string, pass
    }
    // V1: Must be in NFC form.
    else if label.nfc().ne(label.chars()) {
        errors.push(Error::ValidityCriteria);
    }
    // V2: No U+002D HYPHEN-MINUS in both third and fourth positions.
    //
    // NOTE: Spec says that the label must not contain a HYPHEN-MINUS character in both the
    // third and fourth positions. But nobody follows this criteria. See the spec issue below:
    // https://github.com/whatwg/url/issues/53
    //
    // TODO: Add *CheckHyphens* flag.

    // V3: neither begin nor end with a U+002D HYPHEN-MINUS
    else if label.starts_with('-') || label.ends_with('-') {
        errors.push(Error::ValidityCriteria);
    }
    // V4: not contain a U+002E FULL STOP
    //
    // Here, label can't contain '.' since the input is from .split('.')

    // V5: not begin with a GC=Mark
    else if is_combining_mark(first_char.unwrap()) {
        errors.push(Error::ValidityCriteria);
    }
    // V6: Check against Mapping Table
    else if label.chars().any(|c| match Mapping::of(c) {
        Mapping::Valid => false,
        Mapping::Deviation(_) => flags.transitional_processing,
        Mapping::DisallowedStd3Valid => flags.use_std3_ascii_rules,
        _ => true,
    }) {
        errors.push(Error::ValidityCriteria);
    }
    // V7: ContextJ rules
    //
    // TODO: Implement rules and add *CheckJoiners* flag.

    // V8: Bidi rules
    //
    // TODO: Add *CheckBidi* flag
    else if !passes_bidi(label, is_bidi_domain) {
        errors.push(Error::ValidityCriteria);
    }
}

// https://www.unicode.org/reports/tr46/#Processing
fn processing(domain: &str, flags: Flags, errors: &mut Vec<Error>) -> String {
    use self::bidi_class::abbr_names::*;

    let mut mapped = String::new();
    for c in domain.chars() {
        map_char(c, flags, &mut mapped, errors)
    }
    let normalized: String = mapped.nfc().collect();

    // Find out if it's a Bidi Domain Name
    //
    // First, check for literal bidi chars
    let mut is_bidi_domain = domain
        .chars()
        .any(|c| matches!(BidiClass::of(c), R | AL | AN));
    if !is_bidi_domain {
        // Then check for punycode-encoded bidi chars
        for label in normalized.split('.') {
            if label.starts_with(PUNYCODE_PREFIX) {
                match punycode::decode_to_string(&label[PUNYCODE_PREFIX.len()..]) {
                    Some(decoded_label) => {
                        if decoded_label
                            .chars()
                            .any(|c| matches!(BidiClass::of(c), R | AL | AN))
                        {
                            is_bidi_domain = true;
                        }
                    }
                    None => {
                        is_bidi_domain = true;
                    }
                }
            }
        }
    }

    let mut validated = String::new();
    let mut first = true;
    for label in normalized.split('.') {
        if !first {
            validated.push('.');
        }
        first = false;
        if label.starts_with(PUNYCODE_PREFIX) {
            match punycode::decode_to_string(&label[PUNYCODE_PREFIX.len()..]) {
                Some(decoded_label) => {
                    let flags = Flags {
                        transitional_processing: false,
                        ..flags
                    };
                    validate(&decoded_label, is_bidi_domain, flags, errors);
                    validated.push_str(&decoded_label)
                }
                None => errors.push(Error::PunycodeError),
            }
        } else {
            validate(label, is_bidi_domain, flags, errors);
            validated.push_str(label)
        }
    }
    validated
}

/// Optional settings for processing and conversion algorithms.
#[derive(Copy, Clone, Debug)]
pub struct Flags {
    /// *UseSTD3ASCIIRules* flag.
    ///
    /// <https://www.unicode.org/reports/tr46/#UseSTD3ASCIIRules>
    pub use_std3_ascii_rules: bool,

    /// *Transitional_Processing* or *Nontransitional Processing*, for transitional handling of
    /// *Deviation* characters.
    ///
    /// <https://www.unicode.org/reports/tr46/#Conformance>
    pub transitional_processing: bool,

    /// *VerifyDnsLength* flag, to verify DNS length restrictions.
    ///
    /// <https://www.unicode.org/reports/tr46/#ToASCII>
    pub verify_dns_length: bool,
}

/// Error types recorded during UTS #46 processing.
#[cfg_attr(feature = "cargo-clippy", allow(enum_variant_names))]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Error {
    PunycodeError,
    ValidityCriteria,
    DissallowedByStd3AsciiRules,
    DissallowedMappedInStd3,
    DissallowedCharacter,
    TooLongForDns,
    TooShortForDns,
}

/// Errors recorded during UTS #46 processing.
///
/// This is opaque for now, only indicating the presence of at least one error.
/// More details may be exposed in the future.
#[derive(Debug, Eq, PartialEq)]
pub struct Errors(Vec<Error>);

/// <https://www.unicode.org/reports/tr46/#ToASCII>
pub fn to_ascii(domain: &str, flags: Flags) -> Result<String, Errors> {
    let mut errors = Vec::new();
    let mut result = String::new();
    let mut first = true;
    for label in processing(domain, flags, &mut errors).split('.') {
        if !first {
            result.push('.');
        }
        first = false;
        if label.is_ascii() {
            result.push_str(label);
        } else {
            match punycode::encode_str(label) {
                Some(x) => {
                    result.push_str(PUNYCODE_PREFIX);
                    result.push_str(&x);
                }
                None => errors.push(Error::PunycodeError),
            }
        }
    }

    if flags.verify_dns_length {
        let domain = if result.ends_with('.') {
            &result[..result.len() - 1]
        } else {
            &*result
        };
        if domain.is_empty() || domain.split('.').any(|label| label.is_empty()) {
            errors.push(Error::TooShortForDns)
        }
        if domain.len() > 253 || domain.split('.').any(|label| label.len() > 63) {
            errors.push(Error::TooLongForDns)
        }
    }
    if errors.is_empty() {
        Ok(result)
    } else {
        Err(Errors(errors))
    }
}

/// <https://www.unicode.org/reports/tr46/#ToUnicode>
///
/// Only `use_std3_ascii_rules` is used in `flags`.
pub fn to_unicode(domain: &str, mut flags: Flags) -> (String, Result<(), Errors>) {
    flags.transitional_processing = false;
    let mut errors = Vec::new();
    let domain = processing(domain, flags, &mut errors);
    let errors = if errors.is_empty() {
        Ok(())
    } else {
        Err(Errors(errors))
    };
    (domain, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// https://github.com/servo/rust-url/issues/373
    #[test]
    fn test_punycode_prefix_with_length_check() {
        fn _to_ascii(domain: &str) -> Result<String, Errors> {
            to_ascii(
                domain,
                Flags {
                    transitional_processing: false,
                    use_std3_ascii_rules: true,
                    verify_dns_length: true,
                },
            )
        }

        assert!(_to_ascii("xn--").is_err());
        assert!(_to_ascii("xn---").is_err());
        assert!(_to_ascii("xn-----").is_err());
        assert!(_to_ascii("xn--.").is_err());
        assert!(_to_ascii("xn--...").is_err());
        assert!(_to_ascii(".xn--").is_err());
        assert!(_to_ascii("...xn--").is_err());
        assert!(_to_ascii("xn--.xn--").is_err());
        assert!(_to_ascii("xn--.example.org").is_err());
    }

    /// https://github.com/servo/rust-url/issues/373
    #[test]
    fn test_punycode_prefix_without_length_check() {
        fn _to_ascii(domain: &str) -> Result<String, Errors> {
            to_ascii(
                domain,
                Flags {
                    transitional_processing: false,
                    use_std3_ascii_rules: true,
                    verify_dns_length: false,
                },
            )
        }

        assert_eq!(_to_ascii("xn--"), Ok("".to_owned()));
        assert!(_to_ascii("xn---").is_err());
        assert!(_to_ascii("xn-----").is_err());
        assert_eq!(_to_ascii("xn--."), Ok(".".to_owned()));
        assert_eq!(_to_ascii("xn--..."), Ok("...".to_owned()));
        assert_eq!(_to_ascii(".xn--"), Ok(".".to_owned()));
        assert_eq!(_to_ascii("...xn--"), Ok("...".to_owned()));
        assert_eq!(_to_ascii("xn--.xn--"), Ok(".".to_owned()));
        assert_eq!(_to_ascii("xn--.example.org"), Ok(".example.org".to_owned()));
    }

    #[test]
    fn test_v5() {
        fn _to_ascii(domain: &str) -> Result<String, Errors> {
            to_ascii(
                domain,
                Flags {
                    transitional_processing: false,
                    use_std3_ascii_rules: true,
                    verify_dns_length: true,
                },
            )
        }

        // IdnaTest:784 蔏｡𑰺
        assert!(is_combining_mark('\u{11C3A}'));
        assert!(_to_ascii("\u{11C3A}").is_err());
        assert!(_to_ascii("\u{850f}.\u{11C3A}").is_err());
        assert!(_to_ascii("\u{850f}\u{ff61}\u{11C3A}").is_err());
    }

    #[test]
    fn test_v8_bidi_rules() {
        fn _to_ascii(domain: &str) -> Result<String, Errors> {
            to_ascii(
                domain,
                Flags {
                    transitional_processing: false,
                    use_std3_ascii_rules: true,
                    verify_dns_length: true,
                },
            )
        }

        assert_eq!(_to_ascii("abc"), Ok("abc".to_owned()));
        assert_eq!(_to_ascii("123"), Ok("123".to_owned()));
        assert_eq!(_to_ascii("אבּג"), Ok("xn--kdb3bdf".to_owned()));
        assert_eq!(_to_ascii("ابج"), Ok("xn--mgbcm".to_owned()));
        assert_eq!(_to_ascii("abc.ابج"), Ok("abc.xn--mgbcm".to_owned()));
        assert_eq!(
            _to_ascii("אבּג.ابج"),
            Ok("xn--kdb3bdf.xn--mgbcm".to_owned())
        );

        // Bidi domain names cannot start with digits
        assert!(_to_ascii("0a.\u{05D0}").is_err());
        assert!(_to_ascii("0à.\u{05D0}").is_err());

        // Bidi chars may be punycode-encoded
        assert!(_to_ascii("xn--0ca24w").is_err());
    }
}
