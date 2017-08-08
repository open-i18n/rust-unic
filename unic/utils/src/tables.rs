//! Trait and impls for character data tables used in UNIC.

use std::cmp;

/// A table from characters to data associated with those characters.
pub trait CharDataTable<V> {
    /// Find the associated data for a character in this table.
    fn find(&self, needle: char) -> Option<V>;

    /// Find the associated data for a character in this table or use a provided default.
    fn find_or(&self, needle: char, default: V) -> V {
        self.find(needle).unwrap_or(default)
    }

    /// Find the associated data for a character in this table or generate a default.
    fn find_or_else<F>(&self, needle: char, f: F) -> V
    where
        F: FnOnce() -> V,
    {
        self.find(needle).unwrap_or_else(f)
    }
}

impl<'a, V> CharDataTable<&'a V> for &'a [(char, V)] {
    fn find(&self, needle: char) -> Option<&'a V> {
        self.binary_search_by_key(&needle, |&(k, _)| k)
            .map(|idx| &self[idx].1)
            .ok()
    }
}

impl<'a, V> CharDataTable<&'a V> for &'a [(char, char, V)] {
    fn find(&self, needle: char) -> Option<&'a V> {
        self.binary_search_by(|&(low, high, _)| if low > needle {
            cmp::Ordering::Greater
        } else if high < needle {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }).map(|idx| &self[idx].2)
            .ok()
    }
}

impl CharDataTable<()> for &'static [(char, char)] {
    fn find(&self, needle: char) -> Option<()> {
        self.binary_search_by(|&(low, high)| if low > needle {
            cmp::Ordering::Greater
        } else if high < needle {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }).map(|_| ())
            .ok()
    }
}
