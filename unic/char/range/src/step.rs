use std::char;

const BEFORE_SURROGATE: char = '\u{D7FF}';
const AFTER_SURROGATE: char = '\u{E000}';

/// Step a `char` toward `char::MAX`.
///
/// If the given `char` is already `char::MAX`, it is returned unchanged.
#[allow(unsafe_code)]
pub fn forward(c: char) -> char {
    if c == char::MAX {
        return c;
    }

    if c == BEFORE_SURROGATE {
        AFTER_SURROGATE
    } else {
        let codepoint = c as u32 + 1;
        debug_assert!(char::from_u32(codepoint).is_some());
        unsafe { char::from_u32_unchecked(codepoint) }
    }
}

/// Step a `char` toward `'\0'`.
///
/// If the given `char` is already `'\0'`, it is returned unchanged.
#[allow(unsafe_code)]
pub fn backward(c: char) -> char {
    if c == '\0' {
        return c;
    }

    if c == AFTER_SURROGATE {
        BEFORE_SURROGATE
    } else {
        let codepoint = c as u32 - 1;
        debug_assert!(char::from_u32(codepoint).is_some());
        unsafe { char::from_u32_unchecked(codepoint) }
    }
}
