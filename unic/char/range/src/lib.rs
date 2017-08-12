//! # Unic - Char - Range
//!
//! A simple way to control iteration over a range of characters.
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate unic_char_range;
//! # use unic_char_range::*;
//!
//! # fn main() {
//! for character in chars!('a'..='z') {
//!     // character is each character in the lowercase english alphabet in order
//! }
//!
//! for character in CharRange::all() {
//!     // character is every valid char from lowest codepoint to highest
//! }
//! # }
//! ```
//!
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion)]
#![deny(missing_docs, unsafe_code, unused, future_incompatible)]
#![cfg_attr(feature = "fused", feature(fused))]
#![cfg_attr(feature = "trusted-len", feature(trusted_len))]

mod range;
mod iter;
mod step;
mod macros;

use std::ops::Range;

pub use range::CharRange;
pub use iter::CharIter;

use step::{step_backward, step_forward};

/// Range of Surrogate Code Points.
///
/// Reference: <http://unicode.org/glossary/#surrogate_code_point>
const SURROGATE_RANGE: Range<u32> = 0xD800..0xE000;
