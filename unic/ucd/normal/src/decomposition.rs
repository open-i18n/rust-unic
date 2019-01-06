// Copyright 2012-2015 The Rust Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::ops::FnMut;

use crate::composition::{canonical_decomposition, compatibility_decomposition};

use unic_ucd_hangul::{decompose_syllable, is_syllable};

/// Compute canonical Unicode decomposition for character.
///
/// See [Unicode Standard Annex #15](https://www.unicode.org/reports/tr15/) for more information.
pub fn decompose_canonical<F>(ch: char, mut callback: F)
where
    F: FnMut(char),
{
    d(ch, &mut callback, false);
}

/// Compute canonical or compatible Unicode decomposition for character.
///
/// See [Unicode Standard Annex #15](https://www.unicode.org/reports/tr15/) for more information.
pub fn decompose_compatible<F>(ch: char, mut callback: F)
where
    F: FnMut(char),
{
    d(ch, &mut callback, true);
}

// FIXME: This is a workaround, we should use `F` instead of `&mut F`
fn d<F>(ch: char, callback: &mut F, k: bool)
where
    F: FnMut(char),
{
    // 7-bit ASCII never decomposes
    if ch <= '\x7f' {
        (*callback)(ch);
        return;
    }

    // Perform decomposition for Hangul
    if is_syllable(ch) {
        decompose_syllable(ch, callback);
        return;
    }

    // First check the canonical decompositions
    if let Some(canon) = canonical_decomposition(ch) {
        for x in canon {
            d(*x, callback, k);
        }
        return;
    }

    // Bottom out if we're not doing compat.
    if !k {
        (*callback)(ch);
        return;
    }

    // Then check the compatibility decompositions
    if let Some(compat) = compatibility_decomposition(ch) {
        for x in compat {
            d(*x, callback, k);
        }
        return;
    }

    // Finally bottom out.
    (*callback)(ch);
}
