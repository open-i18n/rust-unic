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
///         RustName: Long_Name / Abbr "Optional display string literal",
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
/// least one line must be present.
///
/// If not specified, the display literal defaults to the Long_Name (stringified).
/// Note that at this time, either all or none of the display literals must be present.
///
// TODO: Formalize this with a trait?
/*
trait UnicodeCharacterProperty : Clone + Copy + Debug + Display + PartialEq + Eq + Hash {
    /// Abbreviated name of this property
    fn abbr_name(&self) -> &'static str;
    /// Long name of this property
    fn long_name(&self) -> &'static str;

    // And optionally
    /// Get the property value for this character.
    fn of(ch: char) -> Self;
}
*/
// In that case this macro and that trait should probably be in core.
//
/// `PropertyName::abbr_name(&self) -> &'static str` and
/// `PropertyName::long_name(&self) -> &'static str` are provided by this macro, as well as
/// `#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]`. The order of variants is unchanged,
/// meaning `#[derive(PartialOrd, Ord)]` can be used on the enum declaration if it makes sense.
///
/// The module `module_name` is populated with `pub use` aliases for the variants in the abbr form.
/// This module will likely be removed in favor of [`Associated Consts`][rust-lang/rust#42809] once
/// it reaches stable (1.20).
///
/// [rust-lang/rust#42913]: https://github.com/rust-lang/rust/pull/42913
/// [rust-lang/rust#42809]: https://github.com/rust-lang/rust/pull/42809
#[macro_export]
macro_rules! char_property {
    // Default Display impl
    {
        $(#[$_name:meta])*
        pub enum $name:ident {
            $(
                $(#[$_variant:meta])+
                $variant:ident: $long:ident / $abbr:ident,
            )+
        };

        $(#[$_alias:meta])*
        pub mod $alias:ident;
    }
    =>
    {
        char_property! {
            $(#[$_name])*
            pub enum $name {
                $(
                    $(#[$_variant])+
                    $variant: $long/$abbr stringify!($long),
                )+
            };

            $(#[$_alias])*
            pub mod $alias;
        }
    };

    // Specified Display impl
    {
        $(#[$_name:meta])*
        pub enum $name:ident {
            $(
                $(#[$_variant:meta])+
                $variant:ident: $long:ident / $abbr:ident $display:expr,
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
            /// Long name of this property
            pub fn long_name(&self) -> &'static str {
                match *self {
                    $($name::$variant => stringify!($long)),+
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $(
                        $name::$variant =>
                            write!(f, $display)
                    ),+
                }

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
        pub enum CustomDisplayProperty {
            ///
            Variant1: Variant_1 / V1 "Variant_1",
            ///
            Variant2: Variant_2 / V2 "Property=Variant_2",
            ///
            Variant3: Variant_3 / V3 "The third variant",
        };

        /// A very
        /// well documented
        /// abbreviated alias
        pub mod cd_abbr;
    }

    char_property! {
        pub enum ImplicitDisplayProperty {
            ///
            TheDisplay: The_Display / Td,
            ///
            IsImplicit: Is_Implicit / Ii,
        };
        pub mod id_abbr;
    }

    #[test]
    fn abbr_name() {
        assert_eq!(CustomDisplayProperty::Variant1.abbr_name(), "V1");
        assert_eq!(CustomDisplayProperty::Variant2.abbr_name(), "V2");
        assert_eq!(CustomDisplayProperty::Variant3.abbr_name(), "V3");
        assert_eq!(ImplicitDisplayProperty::TheDisplay.abbr_name(), "Td");
        assert_eq!(ImplicitDisplayProperty::IsImplicit.abbr_name(), "Ii");
    }

    #[test]
    fn long_name() {
        assert_eq!(CustomDisplayProperty::Variant1.long_name(), "Variant_1");
        assert_eq!(CustomDisplayProperty::Variant2.long_name(), "Variant_2");
        assert_eq!(CustomDisplayProperty::Variant3.long_name(), "Variant_3");
        assert_eq!(ImplicitDisplayProperty::TheDisplay.long_name(), "The_Display");
        assert_eq!(ImplicitDisplayProperty::IsImplicit.long_name(), "Is_Implicit");
    }

    #[test]
    fn abbr_mod() {
        assert_eq!(CustomDisplayProperty::Variant1, cd_abbr::V1);
        assert_eq!(CustomDisplayProperty::Variant2, cd_abbr::V2);
        assert_eq!(CustomDisplayProperty::Variant3, cd_abbr::V3);
        assert_eq!(ImplicitDisplayProperty::TheDisplay, id_abbr::Td);
        assert_eq!(ImplicitDisplayProperty::IsImplicit, id_abbr::Ii);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", CustomDisplayProperty::Variant1), "Variant_1");
        assert_eq!(format!("{}", CustomDisplayProperty::Variant2), "Property=Variant_2");
        assert_eq!(format!("{}", CustomDisplayProperty::Variant3), "The third variant");
        assert_eq!(format!("{}", ImplicitDisplayProperty::TheDisplay), "The_Display");
        assert_eq!(format!("{}", ImplicitDisplayProperty::IsImplicit), "Is_Implicit");
    }
}
