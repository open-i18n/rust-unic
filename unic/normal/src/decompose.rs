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

use std::collections::VecDeque;
use std::fmt::{self, Write};

use unic_ucd_normal::{decompose_canonical, decompose_compatible, CanonicalCombiningClass};

// Helper functions used for Unicode normalization
fn canonical_sort(comb: &mut VecDeque<(char, CanonicalCombiningClass)>) {
    let len = comb.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 1..len - i {
            let class_a = comb[j - 1].1;
            let class_b = comb[j].1;
            if class_a.is_reordered() && class_b.is_reordered() && class_a > class_b {
                comb.swap(j - 1, j);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[derive(Clone, Debug)]
enum DecompositionType {
    Canonical,
    Compatible,
}

/// External iterator for a string decomposition's characters.
#[derive(Clone, Debug)]
pub struct Decompositions<I> {
    kind: DecompositionType,
    iter: I,
    buffer: VecDeque<(char, CanonicalCombiningClass)>,
    sorted: bool,
}

#[inline]
pub fn new_canonical<I: Iterator<Item = char>>(iter: I) -> Decompositions<I> {
    Decompositions {
        iter,
        buffer: VecDeque::new(),
        sorted: false,
        kind: self::DecompositionType::Canonical,
    }
}

#[inline]
pub fn new_compatible<I: Iterator<Item = char>>(iter: I) -> Decompositions<I> {
    Decompositions {
        iter,
        buffer: VecDeque::new(),
        sorted: false,
        kind: self::DecompositionType::Compatible,
    }
}

impl<I: Iterator<Item = char>> Iterator for Decompositions<I> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        use self::DecompositionType::*;

        match self.buffer.front() {
            Some(&(c, CanonicalCombiningClass::NotReordered)) => {
                self.sorted = false;
                self.buffer.pop_front();
                return Some(c);
            }
            Some(&(c, _)) if self.sorted => {
                self.buffer.pop_front();
                return Some(c);
            }
            _ => self.sorted = false,
        }

        if !self.sorted {
            for ch in self.iter.by_ref() {
                let buffer = &mut self.buffer;
                let sorted = &mut self.sorted;
                {
                    let callback = |d| {
                        let ccc = CanonicalCombiningClass::of(d);
                        if ccc.is_not_reordered() && !*sorted {
                            canonical_sort(buffer);
                            *sorted = true;
                        }
                        buffer.push_back((d, ccc));
                    };
                    match self.kind {
                        Canonical => decompose_canonical(ch, callback),
                        Compatible => decompose_compatible(ch, callback),
                    }
                }
                if *sorted {
                    break;
                }
            }
        }

        if !self.sorted {
            canonical_sort(&mut self.buffer);
            self.sorted = true;
        }

        if self.buffer.is_empty() {
            None
        } else {
            match self.buffer.pop_front() {
                Some((c, CanonicalCombiningClass::NotReordered)) => {
                    self.sorted = false;
                    Some(c)
                }
                Some((c, _)) => Some(c),
                None => unreachable!(),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, _) = self.iter.size_hint();
        (lower, None)
    }
}

impl<I: Iterator<Item = char> + Clone> fmt::Display for Decompositions<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}
