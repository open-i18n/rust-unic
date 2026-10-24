// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Extended_Pictographic` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Extended_Pictographic`](http://www.unicode.org/reports/tr51/#Emoji_Properties_and_Data_Files).
    ///
    /// The value is `true` for characters that are pictographic, or otherwise similar in kind to
    /// characters with the Emoji property, `false` otherwise.
    pub struct ExtendedPictographic(bool) {
        abbr => "Extended_Pictographic";
        long => "Extended_Pictographic";
        human => "Extended Pictographic";

        data_table_path => "../tables/extended_pictographic.rsv";
    }

    /// The value is `true` for characters that are pictographic, or otherwise similar in kind to
    /// characters with the Emoji property, `false` otherwise.
    pub fn is_extended_pictographic(char) -> bool;
}
