// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: (u64, u64, u64) = include!("unicode_version.rsv");

/// Represents values of the Unicode character property
/// [Bidi_Class](http://www.unicode.org/reports/tr44/#Bidi_Class), also
/// known as the *bidirectional character type*.
///
/// * http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types
/// * http://www.unicode.org/reports/tr44/#Bidi_Class_Values
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
// Compare with `bidi_class_type.rsv`
pub enum BidiClass {
    AL,
    AN,
    B,
    BN,
    CS,
    EN,
    ES,
    ET,
    FSI,
    L,
    LRE,
    LRI,
    LRO,
    NSM,
    ON,
    PDF,
    PDI,
    R,
    RLE,
    RLI,
    RLO,
    S,
    WS,
}

use self::BidiClass::*;

pub const BIDI_CLASS_TABLE: &'static [(char, char, BidiClass)] = include!("bidi_class_values.rsv");
