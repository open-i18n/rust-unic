/// Convenience macro for declaring a enumerated character property.
///
/// Syntax:
///
/// ```rust
/// # #[macro_use] extern crate unic_ucd_utils;
/// char_property! {
///     /// Any amount of doc comments describing the property
///     pub enum PropertyName {
///         /// Exactly one line describing the variant
///         RustName: Abbr "Display Name",
///     };
///
///     /// Any amount of doc comments describing the module
///     pub mod module_name;
/// }
/// # fn main() {}
/// ```
///
/// Of course, any number (one or more) of variants may be included, each terminated by a comma.
/// Once ***[rust-lang/rust#42913]*** reaches stable (1.20), one or more doc comment lines
/// can be used on each variant. Additionally, at that time we can remove the restriction that at
/// least one line doc comment line must be present.
///
// TODO: Formalize this with a trait?
/*
trait UnicodeCharacterProperty : Clone + Copy + Debug + Display + PartialEq + Eq + Hash {
    /// Abbreviated name of this property
    fn abbr_name(&self) -> &'static str;
    /// Long name of this property
    fn name(&self) -> &'static str;

    // And optionally
    /// Get the property value for this character.
    fn of(ch: char) -> Self;
}
*/
// In that case this macro and that trait should probably be in core.
//
/// `PropertyName::abbr_name(&self) -> &'static str` and
/// `PropertyName::name(&self) -> &'static str` are provided by this macro, as well as
/// `#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]`. The order of variants is unchanged,
/// meaning `#[derive(PartialOrd, Ord)]` can be used on the enum declaration if it makes sense.
/// ```std::fmt::Display` is also implemented to write the display name of the property.
///
/// The module `module_name` is populated with `pub use` aliases for the variants in the abbr form.
/// This module will likely be removed in favor of [`Associated Consts`][rust-lang/rust#42809] once
/// it reaches stable (1.20).
///
/// [rust-lang/rust#42913]: https://github.com/rust-lang/rust/pull/42913
/// [rust-lang/rust#42809]: https://github.com/rust-lang/rust/pull/42809
#[macro_export]
macro_rules! char_property {
    {
        $(#[$_name:meta])*
        pub enum $name:ident {
            $(
                $(#[$_variant:meta])+
                $variant:ident: $abbr:ident $long:expr,
            )+
        };

        $(#[$_alias:meta])*
        pub mod $alias:ident;
    }
    =>
    {
        $(#[$_name])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $(#[$_variant])*
                $variant
            ),+
        }

        impl $name {
            /// Abbreviated name of this property
            pub fn abbr_name(&self) -> &'static str {
                match *self {
                    $($name::$variant => stringify!($abbr)),+
                }
            }
            /// Name of this property
            pub fn name(&self) -> &'static str {
                match *self {
                    $($name::$variant => $long),+
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        $(#[$_alias])*
        pub mod $alias {
            $(pub use super::$name::$variant as $abbr;)+
        }
    };
}

#[cfg(test)]
mod tests {
    char_property! {
        /// A very
        /// well documented
        /// character property
        pub enum Property {
            ///
            Variant1: V1 "Variant_1",
            ///
            Variant2: V2 "Variant 2",
            ///
            Variant3: V3 "Variant-3",
        };

        pub mod cd_abbr;
    }

    #[test]
    fn abbr_name() {
        assert_eq!(Property::Variant1.abbr_name(), "V1");
        assert_eq!(Property::Variant2.abbr_name(), "V2");
        assert_eq!(Property::Variant3.abbr_name(), "V3");
    }

    #[test]
    fn long_name() {
        assert_eq!(Property::Variant1.name(), "Variant_1");
        assert_eq!(Property::Variant2.name(), "Variant 2");
        assert_eq!(Property::Variant3.name(), "Variant-3");
    }

    #[test]
    fn abbr_mod() {
        assert_eq!(Property::Variant1, cd_abbr::V1);
        assert_eq!(Property::Variant2, cd_abbr::V2);
        assert_eq!(Property::Variant3, cd_abbr::V3);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Property::Variant1), Property::Variant1.name());
        assert_eq!(format!("{}", Property::Variant2), Property::Variant2.name());
        assert_eq!(format!("{}", Property::Variant3), Property::Variant3.name());
    }
}
