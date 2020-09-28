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

#![forbid(unsafe_code)]

use unic::bidi::BidiInfo;

#[rustfmt::skip]
fn main() {
    let text = concat![
        "א",
        "ב",
        "ג",
        "a",
        "b",
        "c",
    ];

    // Resolve embedding levels within the text.  Pass `None` to detect the
    // paragraph level automatically.
    let bidi_info = BidiInfo::new(text, None);

    // This paragraph has embedding level 1 because its first strong character is RTL.
    assert_eq!(bidi_info.paragraphs.len(), 1);
    let para = &bidi_info.paragraphs[0];
    assert_eq!(para.level.number(), 1);
    assert_eq!(para.level.is_rtl(), true);

    // Re-ordering is done after wrapping each paragraph into a sequence of
    // lines. For this example, I'll just use a single line that spans the
    // entire paragraph.
    let line = para.range.clone();

    let display = bidi_info.reorder_line(para, line);
    assert_eq!(
        display,
        concat![
            "a",
            "b",
            "c",
            "ג",
            "ב",
            "א",
        ]
    );
}
