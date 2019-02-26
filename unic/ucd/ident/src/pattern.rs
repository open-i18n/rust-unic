// Copyright 2017-2019 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

char_property! {
    /// A character that should be treated as a syntax character in patterns.
    pub struct PatternSyntax(bool) {
        abbr => "Pat_Syn";
        long => "Pattern_Syntax";
        human => "Pattern Syntax";

        data_table_path => "../tables/pattern_syntax.rsv";
    }

    /// Is this a character that should be treated as syntax in patterns?
    pub fn is_pattern_syntax(char) -> bool;
}

char_property! {
    /// A character that should be treated as a whitespace in patterns.
    pub struct PatternWhitespace(bool) {
        abbr => "Pat_WS";
        long => "Pattern_White_Space";
        human => "Pattern Whitespace";

        data_table_path => "../tables/pattern_white_space.rsv";
    }

    /// Is this a character that should be treated as whitespace in patterns?
    pub fn is_pattern_whitespace(char) -> bool;
}
