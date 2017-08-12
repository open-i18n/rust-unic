use std::char;
use std::ops::Range;
use step;

const SURROGATE_RANGE: Range<u32> = 0xD800..0xE000;

/// A range of unicode code points.
#[derive(Debug)]
pub struct CharRange {
    /// The lowest uniterated character.
    ///
    /// Iteration is finished if this is higher than `high`.
    ///
    /// # Safety
    ///
    /// This is not guaranteed to always be a valid character. Check before using!
    /// Note that `high` _is_ guaranteed to be a valid character,
    /// so this will always be a valid character when iteration is not yet finished.
    low: char,

    /// The highest uniterated character.
    ///
    /// Iteration is finished if this is lower than `low`.
    high: char,
}

/// Constructors
impl CharRange {
    /// Construct a closed range of characters
    pub fn closed(start: char, stop: char) -> CharRange {
        CharRange {
            low: start,
            high: stop,
        }
    }
}

impl CharRange {
    #[inline]
    #[allow(unsafe_code)]
    // It is always safe to step `self.low` forward because
    // it will only be used when less than `self.high`.
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
            self.step_forward();
        } else {
            self.high = unsafe { step::backward(self.high) }
        }
    }
}

impl Iterator for CharRange {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        if self.low > self.high {
            return None;
        }

        let char = self.low;
        self.step_forward();
        Some(char)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for CharRange {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.low > self.high {
            return None;
        }

        let char = self.high;
        self.step_backward();
        Some(char)
    }
}

impl ExactSizeIterator for CharRange {
    fn len(&self) -> usize {
        if self.low > self.high {
            return 0;
        }
        let start = self.low as u32;
        let end = self.high as u32;
        let naive_len = (end - start + 1) as usize;
        if start <= SURROGATE_RANGE.start && SURROGATE_RANGE.end <= end {
            naive_len - SURROGATE_RANGE.len() as usize
        } else {
            naive_len
        }
    }
}