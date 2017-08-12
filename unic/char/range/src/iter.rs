use std::{char, iter};
use CharRange;

/// Range of Surrogate Code Points.
///
/// Reference: <http://unicode.org/glossary/#surrogate_code_point>
const SURROGATE_RANGE: ::std::ops::Range<u32> = 0xD800..0xE000;

/// An iterator over `char`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CharIter {
    /// The codepoint of the smallest uniterated codepoint.
    ///
    /// If greater than or equal to `high`, iteration is finished.
    ///
    /// # Safety
    ///
    /// Must be a valid, non-surrogate codepoint while iteration is unfinished.
    low: u32,

    /// The codepoint one greater than the largest uniterated codepoint.
    ///
    /// If less than or equal to `low`, iteration is finished.
    ///
    /// # Safety
    ///
    /// Must be one greater than a valid, non surrogate codepoint while iteration is unfinished.
    high: u32,
}

impl<'a> From<&'a CharRange> for CharIter {
    fn from(range: &CharRange) -> CharIter {
        CharIter {
            low: range.first() as u32,
            high: range.last() as u32 + 1,
        }
    }
}

impl CharIter {
    #[inline]
    fn is_finished(&self) -> bool {
        self.low >= self.high
    }
}

impl Iterator for CharIter {
    type Item = char;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<char> {
        if self.is_finished() {
            return None;
        }

        let char = unsafe { char::from_u32_unchecked(self.low) };
        self.low += 1;

        // ensure `low` is never a surrogate code point
        if self.low == SURROGATE_RANGE.start {
            self.low = SURROGATE_RANGE.end;
        }

        Some(char)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl iter::DoubleEndedIterator for CharIter {
    #[allow(unsafe_code)]
    fn next_back(&mut self) -> Option<char> {
        if self.is_finished() {
            return None;
        }

        self.high -= 1;
        let char = unsafe { char::from_u32_unchecked(self.high) };

        // ensure `high` is never one greater than a surrogate code point
        if self.high == SURROGATE_RANGE.end {
            self.high = SURROGATE_RANGE.start;
        }

        Some(char)
    }
}

impl iter::ExactSizeIterator for CharIter {
    fn len(&self) -> usize {
        if self.is_finished() {
            return 0;
        }
        let naive_len = self.high as usize - self.low as usize;
        if self.low <= SURROGATE_RANGE.start && SURROGATE_RANGE.end <= self.high {
            naive_len - SURROGATE_RANGE.len()
        } else {
            naive_len
        }
    }
}

#[cfg(feature = "fused")]
impl iter::FusedIterator for CharIter {}

#[allow(unsafe_code)]
#[cfg(feature = "trusted-len")]
unsafe impl iter::TrustedLen for CharIter {}
