// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Taxonomy and contracts for character property types.
//!
//! See also the list of types of character properties defined in the UCD:
//! <http://unicode.org/reports/tr44/#About_Property_Table>, in [Unicode® Standard Annex #44 —
//! Unicode Character Database](http://unicode.org/reports/tr44/#About_Property_Table)


mod domain;
mod range;
mod macros;


pub use self::domain::{CompleteCharProperty, PartialCharProperty};
pub use self::range::{EnumeratedCharProperty, NumericCharProperty, NumericCharPropertyValue};
