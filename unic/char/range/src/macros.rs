#[macro_export]
/// Convenience macro to allow simple construction of character ranges.
///
/// # Syntax
///
/// ```
/// # #[macro_use] extern crate unic_char_range;
/// # fn main() {
/// chars!('a'..='z'); // iterates the inclusive range 'a' through 'z'
/// # }
/// ```
macro_rules! chars {
    // $:expr can only be terminated by `=>`, `,`, `;` so use a $:tt
    ( $start:tt ..= $end:tt ) => ( $crate::CharRange::closed_range($start, $end) );
}

#[cfg(test)]
mod test {
    use std::char;

    #[test]
    fn char_inclusive_iteration_works() {
        let mut target = 'a' as u32 - 1;

        for char in chars!('a'..='z') {
            target += 1;
            assert_eq!(Some(char), char::from_u32(target));
        }

        assert_eq!(target, 'z' as u32, "All characters were iterated");
    }
}
