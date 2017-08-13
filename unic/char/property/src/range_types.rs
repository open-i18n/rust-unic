// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Character Property Range types.


use super::CharProperty;


// == Enumerated/Catalog Types ==

/// A Character Property with enumerated values.
///
/// This is similar to types *Enumeration* and *Catalog*, as defined in UAX#44.
///
/// Usage Note: If the property is of type *Catalog*, it's recommended to (in some way) mark the
/// type as *non-exhaustive*, so that adding new variants to the `enum` type won't result in API
/// breakage.
pub trait EnumeratedCharProperty: Sized + CharProperty {
    /// Exhaustive list of all property values.
    fn all_values() -> &'static [Self];

    /// The *abbreviated name* of the property value.
    fn abbr_name(&self) -> &'static str;

    /// The *long name* of the property value.
    fn long_name(&self) -> &'static str;

    /// The *human-readable name* of the property value.
    fn human_name(&self) -> &'static str;
}


// == Numeric Types ==

/// Marker for numeric types accepted by `NumericCharProperty`.
pub trait NumericCharPropertyValue {}

impl NumericCharPropertyValue for u8 {}


/// A Character Property with numeric values.
///
/// Examples: *Numeric_Value*, *Canonical_Combining_Class*
pub trait NumericCharProperty<Value: NumericCharPropertyValue>: CharProperty {
    /// The numeric value for the property value.
    fn number(&self) -> Value;
}
