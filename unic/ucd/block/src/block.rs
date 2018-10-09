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

use unic_char_property::tables::CharDataTableIter;
use unic_char_range::CharRange;

#[derive(Debug)]
pub struct Block {
    pub range: CharRange,
    pub name: &'static str,
}

impl Block {
    pub fn of(chr: char) -> Option<Block> {
        match data::BLOCKS.find_with_range(chr) {
            None => None,
            Some((range, name)) => Some(Block::new(range, name)),
        }
    }

    pub fn new(range: CharRange, name: &'static str) -> Block {
        Block {
            range: range,
            name: name,
        }
    }
}

/// Iterator for all assigned Unicode blocks, except:
/// - U+D800..U+DB7F, High Surrogates
/// - U+DB80..U+DBFF, High Private Use Surrogates
/// - U+DC00..U+DFFF, Low Surrogates
#[derive(Debug)]
pub struct BlockIter<'a> {
    iter: CharDataTableIter<'a, &'static str>,
}

impl<'a> BlockIter<'a> {
    pub fn new() -> BlockIter<'a> {
        BlockIter {
            iter: data::BLOCKS.iter(),
        }
    }
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Block> {
        match self.iter.next() {
            None => None,
            Some((range, name)) => Some(Block::new(range, name)),
        }
    }
}

mod data {
    use unic_char_property::tables::CharDataTable;
    pub const BLOCKS: CharDataTable<&str> = include!("../tables/blocks.rsv");
}
