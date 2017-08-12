use std::{char, iter};
use CharIter;

/// A closed range of unicode code points.
///
/// This matches the pattern used in text processing rather than in Rust's Range:
/// regex `['a'-'z']` and unicode ranges `U+0000 - U+10FFFF` are inclusive on both ends.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CharRange {
    /// The lower bound of the range (inclusive).
    pub start: char,
    /// The upper bound of the range (inclusive).
    pub end: char,
}

/// Convenience constructors
impl CharRange {
    /// Construct a new `CharRange` with inclusive start and end.
    #[inline]
    pub fn new(start: char, end: char) -> CharRange {
        CharRange { start, end }
    }

    /// Construct a `CharRange` encompassing all of Unicode.
    #[cfg_attr(feature = "associated-consts",
               deprecated(note = "Use the associated-const version instead."))]
    #[inline]
    pub fn all() -> CharRange {
        CharRange::new('\0', char::MAX)
    }
}

impl Default for CharRange {
    /// `CharRange` defaults to
    #[cfg_attr(feature = "associated-consts", doc = "`CharRange::ALL`")]
    #[cfg_attr(not(feature = "associated-consts"), doc = "`CharRange::all()`")]
    /// . [Read More](https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default)
    #[inline]
    fn default() -> CharRange {
        CharRange::new('\0', char::MAX)
    }
}

/// Common ranges
#[cfg(feature = "associated-consts")]
impl CharRange {
    /// All Unicode characters.
    ///
    /// `U+0000` through `U+10FFFF`
    pub const ALL: CharRange = CharRange {
        start: '\0',
        end: char::MAX,
    };

    /// Basic Latin characters. Roughly equals the printable ascii range.
    ///
    /// `U+0020` through `U+007F`
    pub const BASIC_LATIN: CharRange = CharRange {
        start: '\u{0020}',
        end: '\u{007F}',
    };

    /// The basic multilingual plane. <http://unicode.org/roadmaps/bmp/>
    ///
    /// This contains most "normal" characters, and is the range of UTF-16 codepoints.
    ///
    /// `U+0000` through `U+FFFF`
    pub const BASIC_MULTILINGUAL_PLANE: CharRange = CharRange {
        start: '\u{0000}',
        end: '\u{FFFF}',
    };

    /// The two private use planes. <http://www.unicode.org/faq/private_use.html>
    ///
    /// Note that the last two characters on each plane _are not_ private use characters.
    ///
    /// `U+F0000` through `U+10FFFF`
    pub const PRIVATE_USE_PLANES: CharRange = CharRange {
        start: '\u{F0000}',
        end: '\u{10FFFF}',
    };
}

/// Common collection-like fn
impl CharRange {
    /// Create an iterator over this range of characters.
    #[inline]
    pub fn iter(&self) -> CharIter {
        CharIter::from(self)
    }

    /// Create a reverse iterator over this range of characters.
    #[inline]
    pub fn rev(&self) -> iter::Rev<CharIter> {
        self.iter().rev()
    }

    /// Does this range include a character?
    #[inline]
    pub fn contains(&self, char: char) -> bool {
        self.start <= char && char <= self.end
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
