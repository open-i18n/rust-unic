// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use unic_char_basics::unicode_notation;
use unic_char_range::CharRange;
use unic_ucd_category::GeneralCategory;

fn main() {
    println!("# Characters with General_Category(ch) == Lowercase_Letter && toUppercase(ch) == ch");

    let mut total_count = 0;
    let mut cond_count = 0;
    for ch in CharRange::assigned_normal_planes() {
        if GeneralCategory::of(ch) == GeneralCategory::LowercaseLetter {
            total_count += 1;
            // TODO(GH-153): Use unic-case for case manipulation.
            if ch.to_string() == ch.to_uppercase().collect::<String>() {
                if cond_count > 0 {
                    print!(", ");
                }
                cond_count += 1;
                print!("{}", unicode_notation(ch));
            }
        }
    }
    println!();
    println!(
        "# Count: {} out of {} Lowercase_Letter characters",
        cond_count, total_count
    );
}
