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
/// # }
/// ```
///
/// `chars!(..=)` is a constant-time expression, and can be used where such are required,
/// such as in the initialization of constant data structures.
macro_rules! chars {
    ( $low:tt .. $high:tt ) => ( $crate::CharRange { low: $low, high: $high } );
    ( $low:tt ..= $high:tt ) => ( $crate::CharRange::open_right($low, $high) );
}
