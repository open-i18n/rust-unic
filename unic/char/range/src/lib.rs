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
//! for character in chars!('a'=..='z') {
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

const BEFORE_SURROGATE: u32 = 0xD7FF;
const AFTER_SURROGATE: u32 = 0xE000;

mod range;
mod step;

pub use range::CharRange;
