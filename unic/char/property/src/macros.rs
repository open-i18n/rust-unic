// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


/// Macro for declaring a character property.
///
/// # Syntax (Enumerated Property)
///
/// ```
/// #[macro_use]
/// extern crate unic_char_property;
///
/// // First we define the type itself.
/// char_property! {
///     /// This is the enum type created for the character property.
///     pub enum MyProp {
///         abbr => "AbbrPropName";
///         long => "Long_Property_Name";
///         human => "Human-Readable Property Name";
///
///         /// Exactly one attribute
///         RustName {
///             abbr => AbbrName,
///             long => Long_Name,
///             // human is optional and may be included on _all_ traits or left off of _all_ traits
///             human => "&'static str that is a nicer presentation of the name",
///         }
///     }
///
///     /// Module aliasing property value abbreviated names.
///     pub mod abbr_names for abbr;
///
///     /// Module aliasing property value long names.
///     pub mod long_names for long;
/// }
///
/// // We also need to impl `PartialCharProperty` or `CompleteCharProperty` manually.
/// # impl unic_char_property::PartialCharProperty for MyProp {
/// #     fn of(_: char) -> Option<Self> { None }
/// # }
/// #
/// # fn main() {}
/// ```
///
/// # Effect
///
/// - Implements the `CharProperty` trait and appropriate range trait
/// - Implements `FromStr` accepting either the abbr or long name, ascii case insensitive
/// - Populates the module `abbr_names` with `pub use` bindings of variants to their abbr names
/// - Populates the module `long_names` with `pub use` bindings of variants to their long names
/// - Maintains all documentation comments and other `#[attributes]` as would be expected
///   (with some limitations, listed below)
///
/// # Limitations
///
/// Due to [rust-lang/rust#24189](https://github.com/rust-lang/rust/issues/24189), (fixed in
/// [rust-lang/rust#42913](https://github.com/rust-lang/rust/pull/42913), landing in 1.20),
/// exactly one attribute line must be used on each variant.
///
/// On 1.20 or higher, one or more may be used, and the restriction can be relaxed back to
/// the intended zero or more by replacing `$(#[$variant_meta:meta])+` with
/// `$(#[$variant_meta:meta])*` and `$(#[$variant_meta])+` with `$(#[$variant_meta])*`.
// TODO: Once adopting 1.20, fix the macro to work with zero attributes on variants (see above)
#[macro_export]
macro_rules! char_property {
    (
        $(#[$name_meta:meta])* pub enum $name:ident {
            abbr => $abbr_name:expr;
            long => $long_name:expr;
            human => $human_name:expr;

            $(
                $(#[$variant_meta:meta])+
                $variant:ident {
                    abbr => $abbr:ident,
                    long => $long:ident,
                    human => $human:expr,
                }
            )*
        }

        $(#[$abbr_mod_meta:meta])* pub mod $abbr_mod:ident for abbr;
        $(#[$long_mod_meta:meta])* pub mod $long_mod:ident for long;
    ) => {
        $(#[$name_meta])*
        #[allow(bad_style)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $name {
            $( $(#[$variant_meta])+ $variant, )*
        }

        $(#[$abbr_mod_meta])*
        #[allow(bad_style)]
        pub mod $abbr_mod {
            $( pub use super::$name::$variant as $abbr; )*
        }

        $(#[$long_mod_meta])*
        #[allow(bad_style)]
        pub mod $long_mod {
            $( pub use super::$name::$variant as $long; )*
        }

        #[allow(unreachable_patterns)]
        impl ::std::str::FromStr for $name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($abbr) => Ok($name::$variant),
                        stringify!($long) => Ok($name::$variant),
                    )*
                    $(
                        str if ::std::ascii::AsciiExt::eq_ignore_ascii_case(str, stringify!($abbr))
                            => Ok($name::$variant),
                        str if ::std::ascii::AsciiExt::eq_ignore_ascii_case(str, stringify!($long))
                            => Ok($name::$variant),
                    )*
                    _ => Err(()),
                }
            }
        }

        impl $crate::CharProperty for $name {
            fn prop_abbr_name() -> &'static str { $abbr_name }
            fn prop_long_name() -> &'static str { $long_name }
            fn prop_human_name() -> &'static str { $human_name }
        }

        impl $crate::EnumeratedCharProperty for $name {
            fn all_values() -> &'static [$name] {
                const VALUES: &[$name] = &[
                    $($name::$variant,)+
                ];
                VALUES
            }

            fn abbr_name(&self) -> &'static str {
                match *self {
                    $( $name::$variant => stringify!($abbr), )*
                }
            }

            fn long_name(&self) -> &'static str {
                match *self {
                    $( $name::$variant => stringify!($long), )*
                }
            }

            fn human_name(&self) -> &'static str {
                match *self {
                    $( $name::$variant => $human, )*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::EnumeratedCharProperty::human_name(self).fmt(f)
            }
        }
    };
}
