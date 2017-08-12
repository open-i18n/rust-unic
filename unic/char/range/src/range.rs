use std::char;
use {step, CharIter};

// This is a closed range to match the pattern used in text processing rather than in Rust's Range:
// regex `['a'-'z']` and unicode ranges `U+0000 - U+10FFFF` are inclusive on both ends.
// Additionally, this allows us to easily store ranges which border invalid characters.
/// A range of unicode code points.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CharRange {
    /// The lower bound of the range (inclusive).
    first: char,
    /// The upper bound of the range (inclusive).
    last: char,
}

impl CharRange {
    /// The first character in this range (inclusive).
    pub fn first(&self) -> char {
        self.first
    }
    /// The last character in this range (inclusive).
    pub fn last(&self) -> char {
        self.last
    }
}

/// Convenience constructors
impl CharRange {
    /// Construct a `CharRange` from a closed range.
    pub fn closed_range(start: char, end: char) -> CharRange {
        CharRange {
            first: start,
            last: end,
        }
    }

    // Note that the saturating behavior of `step_forward`/`step_backward` is not an issue here
    // because if it happens, the range is by necessity empty before the step.

    /// Construct a `CharRange` from a half open (right) range.
    pub fn half_open_right_range(start: char, end: char) -> CharRange {
        CharRange {
            first: start,
            last: step::backward(end),
        }
    }

    /// Construct a `CharRange` from a half open (left) range.
    pub fn half_open_left_range(start: char, end: char) -> CharRange {
        CharRange {
            first: step::forward(start),
            last: end,
        }
    }

    /// Construct a `CharRange` from an open range.
    pub fn open_range(start: char, end: char) -> CharRange {
        CharRange {
            first: step::forward(start),
            last: step::backward(end),
        }
    }

    /// Construct a `CharRange` encompassing all of Unicode.
    #[cfg_attr(feature = "associated-consts",
               deprecated(note = "Use the associated-const version instead."))]
    #[inline]
    pub fn all() -> CharRange {
        CharRange::closed_range('\0', char::MAX)
    }
}

impl Default for CharRange {
    /// `CharRange` defaults to
    #[cfg_attr(feature = "associated-consts", doc = "`CharRange::ALL`")]
    #[cfg_attr(not(feature = "associated-consts"), doc = "`CharRange::all()`")]
    /// . [Read More](https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default)
    #[allow(deprecated)]
    fn default() -> CharRange {
        CharRange::all()
    }
}

/// Common ranges
#[cfg(feature = "associated-consts")]
impl CharRange {
    /// All Unicode characters.
    ///
    /// `U+0000` through `U+10FFFF`
    pub const ALL: CharRange = CharRange {
        first: '\0',
        last: char::MAX,
    };

    /// Basic Latin characters. Roughly equals the printable ascii range.
    ///
    /// `U+0020` through `U+007F`
    pub const BASIC_LATIN: CharRange = CharRange {
        first: '\u{0020}',
        last: '\u{007F}',
    };

    /// The basic multilingual plane. <http://unicode.org/roadmaps/bmp/>
    ///
    /// This contains most "normal" characters, and is the range of UTF-16 codepoints.
    ///
    /// `U+0000` through `U+FFFF`
    pub const BASIC_MULTILINGUAL_PLANE: CharRange = CharRange {
        first: '\u{0000}',
        last: '\u{FFFF}',
    };

    /// The two private use planes. <http://www.unicode.org/faq/private_use.html>
    ///
    /// Note that the last two characters on each plane _are not_ private use characters.
    ///
    /// `U+F0000` through `U+10FFFF`
    pub const PRIVATE_USE_PLANES: CharRange = CharRange {
        first: '\u{F0000}',
        last: '\u{10FFFF}',
    };
}

/// Common collection-like fn
impl CharRange {
    /// Create an iterator over this range of characters.
    #[inline]
    pub fn iter(&self) -> CharIter {
        CharIter::from(self)
    }

    /// Does this range include a character?
    #[inline]
    pub fn contains(&self, char: char) -> bool {
        self.first <= char && char <= self.last
    }

    /// The number of characters in this range.
    #[inline]
    pub fn len(&self) -> usize {
        self.iter().len()
    }
}

impl IntoIterator for CharRange {
    type Item = char;
    type IntoIter = CharIter;

    #[inline]
    fn into_iter(self) -> CharIter {
        self.iter()
    }
}
