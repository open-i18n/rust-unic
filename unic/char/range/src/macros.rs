#[macro_export]
/// Convenience macro for the initialization of `CharRange`s.
///
/// # Syntax
///
/// ```
/// # #[macro_use] extern crate unic_char_range;
/// # fn main() {
/// chars!('a'..'z'); // Iterate the half open range including 'a' and excluding 'z'
/// chars!('a'..='z'); // Iterate the closed range including 'a' and including 'z'
/// chars!(..); // Iterate all characters
/// # }
/// ```
///
/// `chars!('a'..='z')` and `chars!(..)` are constant-time expressions, and can be used
/// where such are required, such as in the initialization of constant data structures.
///
/// Note that because an `expr` capture cannot be followed by a `..`/`..=`,
/// this macro captures token trees. This means that if you want to pass more than one token,
/// you must parenthesize it (e.g. `chars!('\0' ..= (char::MAX)`).
macro_rules! chars {
    ( $low:tt .. $high:tt ) => ( $crate::CharRange::open_right($low, $high) );
    ( $low:tt ..= $high:tt ) => ( $crate::CharRange { low: $low, high: $high } );
    ( .. ) => ( chars!( '\0' ..= (::std::char::MAX) ) );
}
