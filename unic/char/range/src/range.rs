use std::char;
use std::ops::Range;
use step;

const SURROGATE_RANGE: Range<u32> = 0xD800..0xE000;

/// A range of unicode code points.
///
/// The members of this struct are public for const initialization by `chars!(..=)` only.
/// They should be considered unstable private API that may change at any time.
/// If you decide to use them anyway, make sure to note the safety notes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CharRange {
    /// The lowest uniterated character (inclusive).
    ///
    /// Iteration is finished if this is higher than `high`.
    ///
    /// # Safety
    ///
    /// This is not guaranteed to always be a valid character. Check before using!
    /// Note that `high` _is_ guaranteed to be a valid character,
    /// so this will always be a valid character when iteration is not yet finished.
    #[doc(hidden)]
    pub low: char,

    /// The highest uniterated character (inclusive).
    ///
    /// Iteration is finished if this is lower than `low`.
    #[doc(hidden)]
    pub high: char,
}

/// Constructors
impl CharRange {
    /// Construct a closed range of characters.
    pub fn closed(start: char, stop: char) -> CharRange {
        CharRange {
            low: start,
            high: stop,
        }
    }

    /// Construct a half open (right) range of characters.
    pub fn open_right(start: char, stop: char) -> CharRange {
        let mut range = CharRange::closed(start, stop);
        range.step_backward();
        range
    }

    /// Construct a half open (left) range of characters.
    pub fn open_left(start: char, stop: char) -> CharRange {
        let mut range = CharRange::closed(start, stop);
        range.step_forward();
        range
    }

    /// Construct a fully open range of characters.
    pub fn open(start: char, stop: char) -> CharRange {
        let mut range = CharRange::closed(start, stop);
        range.step_forward();
        range.step_backward();
        range
    }

    /// Construct a range over all characters.
    pub fn all() -> CharRange {
        CharRange::closed('\0', char::MAX)
    }
}

impl CharRange {
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

        let ch = self.low;
        self.step_forward();
        Some(ch)
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

        let ch = self.high;
        self.step_backward();
        Some(ch)
    }
}

impl ExactSizeIterator for CharRange {
    fn len(&self) -> usize {
        if self.low > self.high {
            return 0;
        }
        let start = self.low as u32;
        let end = self.high as u32;
        let naive_len = self.high as usize - self.low as usize + 1;
        if start <= SURROGATE_RANGE.start && SURROGATE_RANGE.end <= end {
            naive_len - SURROGATE_RANGE.len() as usize
        } else {
            naive_len
        }
    }
}
