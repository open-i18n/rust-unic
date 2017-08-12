//! # Unic - Char - Range
//!
//! A simple way to control iteration over a range of characters.
//!
//! TODO
#![forbid(bad_style, future_incompatible)]
#![forbid(missing_debug_implementations, unconditional_recursion)]
#![deny(missing_docs, unsafe_code, unused)]
#![cfg_attr(feature = "fused", feature(fused))]
#![cfg_attr(feature = "trusted-len", feature(trusted_len))]

mod range;
mod iter;
mod macros;

use std::ops::Range;

pub use range::CharRange;
pub use iter::CharIter;

/// Range of Surrogate Code Points.
///
/// Reference: <http://unicode.org/glossary/#surrogate_code_point>
const SURROGATE_RANGE: Range<u32> = 0xD800..0xE000;
