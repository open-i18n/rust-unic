// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Taxonomy and contracts for character property types: property domain.


use std::fmt::{Debug, Display};
use std::hash::Hash;


/// A Character Property defined for some characters.
///
/// Examples: *Decomposition_Type*, *Numeric_Type*
pub trait PartialCharProperty: Copy + Debug + Display + Eq + Hash {
    /// Find the character property value, or None.
    fn of(ch: char) -> Option<Self>;
}


/// A Character Property defined on all characters.
///
/// Examples: *Age*, *Name*, *General_Category*, *Bidi_Class*
// Because of rustc bug, we cannot rely on inheritance for the moment.
// See: <https://github.com/rust-lang/rust/issues/43777>
//pub trait CompleteCharProperty: PartialCharProperty + Default {
pub trait CompleteCharProperty
    : PartialCharProperty + Copy + Debug + Display + Eq + Hash + Default {
    /// Find the character property value.
    fn of(ch: char) -> Self;
}

impl<T: CompleteCharProperty> PartialCharProperty for T {
    fn of(ch: char) -> Option<Self> {
        Some(<Self as CompleteCharProperty>::of(ch))
    }
}
