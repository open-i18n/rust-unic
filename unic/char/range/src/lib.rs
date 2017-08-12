//! # Unic - Char - Range
//!
//! A simple way to control iteration over a range of characters.
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate unic_char_range;
//! # use unic_char_range::*;
//! # fn main() {
//! for character in chars!('a'..='z') {
//!     // character is each character in the lowercase english alphabet in order
//! }
//!
//! for character in chars!(..) {
//!     // character is every valid char from lowest codepoint to highest
//! }
//! # }
//! ```
//!
//! # Features
//!
//! None of these features are included by default; they rely on unstable Rust feature gates.
//!
//! - `unstable`: enables all features
//! - `exact-size-is-empty`: provide a specific impl of [`ExactSizeIterator::is_empty`][is_empty]
//! - `fused`: impl the [`FusedIterator`] contract
//! - `trusted-len`: impl the [`TrustedLen`] contract
//!
//! [is_empty](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html#method.is_empty)
//! [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html)
//! [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html)
//!
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion)]
#![deny(missing_docs, unsafe_code, unused, future_incompatible)]
#![cfg_attr(feature = "exact-size-is-empty", feature(exact_size_is_empty))]
#![cfg_attr(feature = "fused", feature(fused))]
#![cfg_attr(feature = "trusted-len", feature(trusted_len))]

mod macros;
mod range;
mod iter;
mod step;

pub use range::CharRange;
pub use iter::CharIter;
