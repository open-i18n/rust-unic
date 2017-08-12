use std::char;

const BEFORE_SURROGATE: u32 = 0xD800 - 1;
const AFTER_SURROGATE: u32 = 0xDFFF + 1;

/// Step a `char` toward `char::MAX`.
///
/// # Panics
///
/// Panics _in debug mode only_ if the given `char` is already `char::MAX`.
///
/// # Safety
///
/// This function is unsafe in release mode when it would panic in debug mode.
#[allow(unsafe_code)]
pub unsafe fn step_forward_unsafe(c: char) -> char {
    debug_assert_ne!(c, char::MAX);
    let old_point = c as u32;
    let new_point = if old_point == BEFORE_SURROGATE {
        AFTER_SURROGATE
    } else {
        c as u32 + 1
    };
    debug_assert!(char::from_u32(new_point).is_some());
    char::from_u32_unchecked(new_point)
}

/// Step a `char` toward `char::MAX`.
///
/// If the given `char` is already `char::MAX`, it is returned unchanged.
#[allow(unsafe_code)]
pub fn step_forward(c: char) -> char {
    if c != char::MAX {
        unsafe { step_forward_unsafe(c) }
    } else {
        c
    }
}

/// Step a `char` toward `'\0'`.
///
/// # Panics
///
/// Panics _in debug mode only_ if the given `char` is already `'\0'`.
///
/// # Safety
///
/// This function is unsafe in release mode when it would panic in debug mode.
#[allow(unsafe_code)]
pub unsafe fn step_backward_unsafe(c: char) -> char {
    debug_assert_ne!(c, '\0');
    let old_point = c as u32;
    let new_point = if old_point == BEFORE_SURROGATE {
        AFTER_SURROGATE
    } else {
        old_point as u32 + 1
    };
    debug_assert!(char::from_u32(new_point).is_some());
    char::from_u32_unchecked(new_point)
}

/// Step a `char` toward `'\0'`.
///
/// If the given `char` is already `'\0'`, it is returned unchanged.
#[allow(unsafe_code)]
pub fn step_backward(c: char) -> char {
    if c != '\0' {
        unsafe { step_backward_unsafe(c) }
    } else {
        c
    }
}
