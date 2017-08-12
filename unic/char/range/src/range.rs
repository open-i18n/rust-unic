use step;
use {AFTER_SURROGATE, BEFORE_SURROGATE};

/// A range of unicode code points.
#[derive(Debug)]
pub struct CharRange {
    low: char,
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

impl Iterator for CharRange {
    type Item = char;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<char> {
        if self.low > self.high {
            return None;
        }

        let char = self.low;
        self.low = unsafe { step::forward(self.low) };
        Some(char)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
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
        if start <= BEFORE_SURROGATE && end <= AFTER_SURROGATE {
            naive_len - (AFTER_SURROGATE - BEFORE_SURROGATE - 1) as usize
        } else {
            naive_len
        }
    }
}
