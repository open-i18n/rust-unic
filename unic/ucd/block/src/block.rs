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
        Block::new(data::BLOCKS.find_with_range(chr))
    }

    fn new(raw_block: Option<(CharRange, &'static str)>) -> Option<Block> {
        match raw_block {
            None => None,
            Some((range, name)) => Some(Block { range, name }),
        }
    }
}

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
        Block::new(self.iter.next()) 
    }
}

mod data {
    use unic_char_property::tables::CharDataTable;
    pub const BLOCKS: CharDataTable<&str> = include!("../tables/blocks.rsv");
}
