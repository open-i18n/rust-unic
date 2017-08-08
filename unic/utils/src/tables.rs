//! bananas

use std::cmp;

/// bananas
pub trait CharBsearchTable<V: 'static> {
    /// bananas
    fn find(&self, needle: char) -> Option<&V>;

    /// bananas
    fn binary_search_or<'a>(&'a self, needle: char, default: &'a V) -> &'a V {
        self.find(needle).unwrap_or(default)
    }

    /// bananas
    fn binary_search_or_else<'a>(&'a self, needle: char, f: fn() -> &'a V) -> &'a V {
        self.find(needle).unwrap_or_else(f)
    }
}

impl<V: 'static> CharBsearchTable<V> for &'static [(char, V)] {
    fn find(&self, needle: char) -> Option<&V> {
        self.binary_search_by_key(&needle, |&(k, _)| k)
            .map(|idx| &self[idx].1)
            .ok()
    }
}

impl<V: 'static> CharBsearchTable<V> for &'static [(char, char, V)] {
    fn find(&self, needle: char) -> Option<&V> {
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

impl CharBsearchTable<()> for &'static [(char, char)] {
    fn find(&self, needle: char) -> Option<&()> {
        self.binary_search_by(|&(low, high)| if low > needle {
            cmp::Ordering::Greater
        } else if high < needle {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }).map(|_| {
                const UNIT: &() = &();
                UNIT
            })
            .ok()
    }
}
