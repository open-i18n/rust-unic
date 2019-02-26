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
    /// A character that can start an identifier.
    pub struct IdStart(bool) {
        abbr => "IDS";
        long => "ID_Start";
        human => "ID Start";

        data_table_path => "../tables/id_start.rsv";
    }

    /// Is this a identifier starting character?
    pub fn is_id_start(char) -> bool;
}

char_property! {
    /// A character that can continue an identifier.
    pub struct IdContinue(bool) {
        abbr => "IDC";
        long => "ID_Continue";
        human => "ID Continue";

        data_table_path => "../tables/id_continue.rsv";
    }

    /// Is this a identifier continuing character?
    pub fn is_id_continue(char) -> bool;
}
