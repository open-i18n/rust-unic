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

use unic_ucd_normal::{compose, CanonicalCombiningClass};

use crate::decompose::Decompositions;

#[derive(Clone, Debug)]
enum RecompositionState {
    Composing,
    Purging,
    Finished,
}

/// External iterator for a string recomposition's characters.
#[derive(Clone, Debug)]
pub struct Recompositions<I> {
    iter: Decompositions<I>,
    state: RecompositionState,
    buffer: VecDeque<char>,
    composee: Option<char>,
    last_ccc: Option<CanonicalCombiningClass>,
}

#[inline]
pub fn new_canonical<I: Iterator<Item = char>>(iter: I) -> Recompositions<I> {
    Recompositions {
        iter: super::decompose::new_canonical(iter),
        state: self::RecompositionState::Composing,
        buffer: VecDeque::new(),
        composee: None,
        last_ccc: None,
    }
}

#[inline]
pub fn new_compatible<I: Iterator<Item = char>>(iter: I) -> Recompositions<I> {
    Recompositions {
        iter: super::decompose::new_compatible(iter),
        state: self::RecompositionState::Composing,
        buffer: VecDeque::new(),
        composee: None,
        last_ccc: None,
    }
}

impl<I: Iterator<Item = char>> Iterator for Recompositions<I> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        use self::RecompositionState::*;

        loop {
            match self.state {
                Composing => {
                    for ch in self.iter.by_ref() {
                        let ch_ccc = CanonicalCombiningClass::of(ch);
                        if self.composee.is_none() {
                            if ch_ccc.is_reordered() {
                                return Some(ch);
                            }
                            self.composee = Some(ch);
                            continue;
                        }
                        let k = self.composee.unwrap();

                        match self.last_ccc {
                            None => match compose(k, ch) {
                                Some(r) => {
                                    self.composee = Some(r);
                                    continue;
                                }
                                None => {
                                    if ch_ccc.is_not_reordered() {
                                        self.composee = Some(ch);
                                        return Some(k);
                                    }
                                    self.buffer.push_back(ch);
                                    self.last_ccc = Some(ch_ccc);
                                }
                            },
                            Some(last_ccc) => {
                                if last_ccc >= ch_ccc {
                                    // `ch` is blocked from `composee`
                                    if ch_ccc.is_not_reordered() {
                                        self.composee = Some(ch);
                                        self.last_ccc = None;
                                        self.state = Purging;
                                        return Some(k);
                                    }
                                    self.buffer.push_back(ch);
                                    self.last_ccc = Some(ch_ccc);
                                    continue;
                                }
                                match compose(k, ch) {
                                    Some(r) => {
                                        self.composee = Some(r);
                                        continue;
                                    }
                                    None => {
                                        self.buffer.push_back(ch);
                                        self.last_ccc = Some(ch_ccc);
                                    }
                                }
                            }
                        }
                    }
                    self.state = Finished;
                    if self.composee.is_some() {
                        return self.composee.take();
                    }
                }
                Purging => match self.buffer.pop_front() {
                    None => self.state = Composing,
                    s => return s,
                },
                Finished => match self.buffer.pop_front() {
                    None => return self.composee.take(),
                    s => return s,
                },
            }
        }
    }
}

impl<I: Iterator<Item = char> + Clone> fmt::Display for Recompositions<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}
