//! Character data tables used in UNIC.

use unic_char_range::CharRange;

/// A mapping from characters to some associated data.
///
/// For the set case, use `()` as the associated value.
pub enum CharDataTable<V: 'static> {
    #[doc(hidden)] Direct(&'static [(char, V)]),
    #[doc(hidden)] Range(&'static [(CharRange, V)]),
}

impl<V: Copy> CharDataTable<V> {
    /// Find the associated data for a character in this table.
    pub fn find(&self, needle: char) -> Option<V> {
        match *self {
            CharDataTable::Direct(table) => {
                table.binary_search_by_key(&needle, |&(k, _)| k)
                    .map(|idx| table[idx].1)
                    .ok()
            }
            CharDataTable::Range(table) => {
                table.binary_search_by(|&(range, _)| range.cmp(needle))
                    .map(|idx| table[idx].1)
                    .ok()
            }
        }
    }
}

impl<V: Copy + Default> CharDataTable<V> {
    /// Find the associated data for a character in this table, or the default value if not entered.
    pub fn find_defaulting(&self, needle: char) -> V {
        self.find(needle).unwrap_or_else(Default::default)
    }
}
