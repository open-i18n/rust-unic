use std::char;
use {AFTER_SURROGATE, BEFORE_SURROGATE};

#[inline(always)]
#[allow(unsafe_code)]
pub unsafe fn forward(c: char) -> char {
    let old_point = c as u32;
    let new_point = if old_point == BEFORE_SURROGATE {
        AFTER_SURROGATE
    } else {
        old_point + 1
    };
    char::from_u32_unchecked(new_point)
}
