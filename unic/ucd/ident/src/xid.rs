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
    /// A character that can start an identifier, stable under NFKC.
    pub struct XidStart(bool) {
        abbr => "XIDS";
        long => "XID_Start";
        human => "XID Start";

        data_table_path => "../tables/xid_start.rsv";
    }

    /// Is this a NFKC-safe identifier starting character?
    pub fn is_xid_start(char) -> bool;
}

char_property! {
    /// A character that can continue an identifier, stable under NFKC.
    pub struct XidContinue(bool) {
        abbr => "XIDC";
        long => "XID_Continue";
        human => "XID Continue";

        data_table_path => "../tables/xid_continue.rsv";
    }

    /// Is this a NFKC-safe identifier continuing character?
    pub fn is_xid_continue(char) -> bool;
}
