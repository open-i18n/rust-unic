use std::collections::BTreeMap;
use std::fmt::Display;

/// A simple binary search array slice.
///
/// Output format:
///
/// ```text
/// &[
///     ('char', Value),
///     ('char', Value),
/// ]
/// ```
///
/// Where
///
/// - `'char'` is a `char::escape_unicode` literal for the character
/// - `Value` is the result of running `display_fn` over the associated value
///
/// It is guaranteed that the `'char'` of one entry will always be ordered before the `'char'` of
/// the next range (such that the array slice is fit for a binary search).
pub trait ToSingleBSearchMap<T> {
    /// Convert this mapping to a `String`.
    fn to_single_bsearch_map<F, D>(&self, display_fn: F) -> String
    where
        F: Fn(&T) -> D,
        D: Display;

    /// A simple default for when the associated value already impls `fmt::Display`.
    ///
    /// Intended to be used when the associated value is a string representing the desired output.
    fn to_single_bsearch_map_default(&self) -> String
    where
        for<'a> &'a T: Display,
    {
        // FIXME: This format call shouldn't be needed
        self.to_single_bsearch_map(|t| format!("{}", t))
    }
}

impl<T> ToSingleBSearchMap<T> for BTreeMap<char, T> {
    fn to_single_bsearch_map<F, D>(&self, display_fn: F) -> String
    where
        F: Fn(&T) -> D,
        D: Display,
    {
        let entries = self.iter();
        let mut out = String::from("&[\n");

        for (char, property) in entries {
            out.push_str(&format!(
                "    ('{}', {}),\n",
                char.escape_unicode(),
                display_fn(property),
            ));
        }

        out.push_str("]");
        out
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use super::ToSingleBSearchMap;

    #[test]
    fn simple_single_bsearch_map() {
        let mut map: BTreeMap<char, &'static str> = Default::default();
        map.insert('a', "A");
        map.insert('b', "B");
        map.insert('c', "C");
        map.insert('z', "Z");
        map.insert('y', "Y");
        map.insert('x', "X");
        assert_eq!(
            map.to_single_bsearch_map_default(),
            "\
&[
    ('\\u{61}', A),
    ('\\u{62}', B),
    ('\\u{63}', C),
    ('\\u{78}', X),
    ('\\u{79}', Y),
    ('\\u{7a}', Z),
]"
        );
    }
}
