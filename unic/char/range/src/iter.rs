use std::char;
use std::ops::Range;
use {CharRange, step};

const SURROGATE_RANGE: Range<u32> = 0xD800..0xE000;

/// An iterator over a range of unicode code points.
#[derive(Clone, Debug)]
pub struct CharIter {
    /// The lowest uniterated character (inclusive).
    ///
    /// Iteration is finished if this is higher than `high`.
    ///
    /// # Safety
    ///
    /// This is not guaranteed to always be a valid character. Check before using!
    /// Note that `high` _is_ guaranteed to be a valid character,
    /// so this will always be a valid character when iteration is not yet finished.
    low: char,

    /// The highest uniterated character (inclusive).
    ///
    /// Iteration is finished if this is lower than `low`.
    high: char,
}

impl From<CharRange> for CharIter {
    fn from(range: CharRange) -> CharIter {
        CharIter { low: range.low, high: range.high }
    }
}

impl From<CharIter> for CharRange {
    fn from(iter: CharIter) -> CharRange {
        CharRange { low: iter.low, high: iter.high }
    }
}

impl CharIter {
    #[inline]
    #[allow(unsafe_code)]
    // It is always safe to step `self.low` forward because
    // `self.low` will only be used when less than `self.high`.
    fn step_forward(&mut self) {
        self.low = unsafe { step::forward(self.low) }
    }

    #[inline]
    #[allow(unsafe_code)]
    // When stepping `self.high` backward would cause underflow,
    // step `self.low` forward instead. It will have the same effect --
    // consuming the last element from the iterator and ending iteration.
    fn step_backward(&mut self) {
        if self.high == '\0' {
            self.low = char::MAX;
        } else {
            self.high = unsafe { step::backward(self.high) }
        }
    }

    #[inline]
    /// ExactSizeIterator::is_empty() for stable
    fn is_finished(&self) -> bool {
        self.low > self.high
    }
}

impl Iterator for CharIter {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        if self.is_finished() {
            return None;
        }

        let ch = self.low;
        self.step_forward();
        Some(ch)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn last(self) -> Option<char> {
        if self.is_finished() {
            None
        } else {
            Some(self.high)
        }
    }

    fn max(self) -> Option<char> {
        self.last()
    }

    fn min(mut self) -> Option<char> {
        self.next()
    }
}

impl DoubleEndedIterator for CharIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_finished() {
            None
        } else {
            let ch = self.high;
            self.step_backward();
            Some(ch)
        }
    }
}

impl ExactSizeIterator for CharIter {
    fn len(&self) -> usize {
        if self.is_finished() {
            return 0;
        }
        let naive_range = (self.low as u32)..(self.high as u32 + 1);
        if naive_range.start <= SURROGATE_RANGE.start && SURROGATE_RANGE.end <= naive_range.end {
            naive_range.len() - SURROGATE_RANGE.len()
        } else {
            naive_range.len()
        }
    }

    #[cfg(feature = "exact-size-is-empty")]
    fn is_empty(&self) -> bool {
        self.is_finished()
    }
}

#[cfg(any(feature = "fused", feature = "trusted-len"))]
use std::iter;

#[cfg(feature = "fused")]
impl iter::FusedIterator for CharIter {}

#[allow(unsafe_code)]
#[cfg(feature = "trusted-len")]
unsafe impl iter::TrustedLen for CharIter {}
