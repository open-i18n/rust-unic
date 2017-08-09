// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Taxonomy and Contracts for Character Property types.
//!
//! See also the list of types of character properties defined in the UCD:
//! <http://unicode.org/reports/tr44/#About_Property_Table>, in [Unicode® Standard Annex #44 —
//! Unicode Character Database](http://unicode.org/reports/tr44/#About_Property_Table)


use std::fmt::{Debug, Display};
use std::hash::Hash;


// == Completeness and Accessors ==

/// Marker for character property types.
pub trait CharProperty<T> {}

// Make char property type T also a parameter to this trait, otherwise there will be "conflicting
// implementation" error when marking traits below as `CharProperty`.
impl<T> CharProperty<T> for T {}


/// A Character Property defined on all characters.
///
/// Examples: *Age*, *Name*, *General_Category*, *Bidi_Class*
pub trait CompleteCharProperty
where
    Self: Copy + Debug + Default + Display + Eq + Hash,
{
    /// Find the character property value.
    fn of(ch: char) -> Self;
}


/// A Character Property defined for some characters.
///
/// Examples: *Decomposition_Type*, *Numeric_Type*
pub trait PartialCharProperty
where
    Self: Copy + Debug + Display + Eq + Hash,
{
    /// Find the character property value, or None.
    fn of(ch: char) -> Option<Self>;
}


// == Enumerated/Catalog Property Type ==

/// A Character Property with enumerated values.
///
/// This is similar to types *Enumeration* and *Catalog*, as defined in UAX#44.
///
/// Usage Note: If the property is of type *Catalog*, it's recommended to (in some way) mark the
/// type as *non-exhaustive*, so that adding new variants to the `enum` type won't result in API
/// breakage.
pub trait EnumeratedCharProperty<T: CharProperty<T>>: Sized
where
    Self: CharProperty<T>,
{
    /// Exhaustive list of all property values.
    fn all_values() -> &'static [Self];
}


// == Numeric Property Type ==

/// Marker for numeric types accepted by `NumericCharProperty`.
pub trait NumericCharPropertyValue {}

impl NumericCharPropertyValue for u8 {}


/// A Character Property with numeric values.
///
/// Examples: *Numeric_Value*, *Canonical_Combining_Class*
pub trait NumericCharProperty<T: CharProperty<T>, Value: NumericCharPropertyValue>
where
    Self: CharProperty<T>,
{
    /// Get numeric value for character property value
    fn number(&self) -> Value;
}
