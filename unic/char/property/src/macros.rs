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
///         /// Zero or more documentation or other attributes.
///         RustName {
///             abbr => AbbrName,
///             long => Long_Name,
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
/// // We also need to impl `PartialCharProperty` or `TotalCharProperty` manually.
/// # impl unic_char_property::PartialCharProperty for MyProp {
/// #     fn of(_: char) -> Option<Self> { None }
/// # }
/// #
/// # fn main() {}
/// ```
///
/// # Syntax (Binary Property)
///
/// ```
/// #[macro_use] extern crate unic_char_property;
/// # #[macro_use] extern crate unic_char_range;
///
/// char_property! {
///     /// This is the newtype used for the character property.
///     pub struct MyProp(bool) {
///         abbr => "AbbrPropName";
///         long => "Long_Property_Name";
///         human => "Human-Readable Property Name";
///
///         // Unlike an enumerated property, a binary property will handle the table for you.
///         data_table_path => "../tests/tables/property_table.rsv";
///     }
///
///     /// A function that returns whether the given character has the property or not.
///     pub fn is_prop(char) -> bool;
/// }
///
/// // You may also want to create a trait for easy access to the properties you define.
/// # fn main() {}
/// ```
///
/// # Effect
///
/// - Implements the `CharProperty` trait and appropriate range trait
/// - Implements `FromStr` accepting either the abbr or long name, ascii case insensitive
/// - Implements `Display` using the `human` string
/// - Populates the module `abbr_names` with `pub use` bindings of variants to their abbr names
///   (Enumerated properties only)
/// - Populates the module `long_names` with `pub use` bindings of variants to their long names
///   (Enumerated properties only)
/// - Maintains all documentation comments and other `#[attributes]` as would be expected
///   (with some limitations, listed below)
///
#[macro_export]
macro_rules! char_property {

    // == Enumerated Property == //

    (
        $(#[$prop_meta:meta])*
        pub enum $prop_name:ident {
            abbr => $prop_abbr:expr;
            long => $prop_long:expr;
            human => $prop_human:expr;

            $(
                $(#[$variant_meta:meta])*
                $variant_name:ident {
                    abbr => $variant_abbr:ident,
                    long => $variant_long:ident,
                    human => $variant_human:expr,
                }
            )*
        }

        $(#[$abbr_mod_meta:meta])*
        pub mod $abbr_mod:ident for abbr;

        $(#[$long_mod_meta:meta])*
        pub mod $long_mod:ident for long;

    ) => {
        $(#[$prop_meta])*
        #[allow(bad_style)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $prop_name {
            $( $(#[$variant_meta])* $variant_name, )*
        }

        $(#[$abbr_mod_meta])*
        #[allow(bad_style)]
        pub mod $abbr_mod {
            $( pub use super::$prop_name::$variant_name as $variant_abbr; )*
        }

        $(#[$long_mod_meta])*
        #[allow(bad_style)]
        pub mod $long_mod {
            $( pub use super::$prop_name::$variant_name as $variant_long; )*
        }

        char_property! {
            __impl FromStr for $prop_name;
            $(
                $variant_abbr => $prop_name::$variant_name;
                $variant_long => $prop_name::$variant_name;
            )*
        }

        char_property! {
            __impl CharProperty for $prop_name;
            $prop_abbr;
            $prop_long;
            $prop_human;
        }

        char_property! {
            __impl Display for $prop_name by EnumeratedCharProperty
        }

        impl $crate::EnumeratedCharProperty for $prop_name {
            fn all_values() -> &'static [$prop_name] {
                const VALUES: &[$prop_name] = &[
                    $( $prop_name::$variant_name, )*
                ];
                VALUES
            }
            fn abbr_name(&self) -> &'static str {
                match *self {
                    $( $prop_name::$variant_name => stringify!($variant_abbr), )*
                }
            }
            fn long_name(&self) -> &'static str {
                match *self {
                    $( $prop_name::$variant_name => stringify!($variant_long), )*
                }
            }
            fn human_name(&self) -> &'static str {
                match *self {
                    $( $prop_name::$variant_name => $variant_human, )*
                }
            }
        }
    };

    // == Binary Property == //

    (
        $(#[$prop_meta:meta])*
        pub struct $prop_name:ident(bool) {
            abbr => $prop_abbr:expr;
            long => $prop_long:expr;
            human => $prop_human:expr;

            data_table_path => $data_path:expr;
        }

        $(#[$is_fn_meta:meta])*
        pub fn $is_fn:ident(char) -> bool;

    ) => {
        $(#[$prop_meta])*
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
        pub struct $prop_name(bool);

        $(#[$is_fn_meta])*
        pub fn $is_fn(ch: char) -> bool {
            $prop_name::of(ch).as_bool()
        }

        impl $prop_name {
            /// Get (struct) property value of the character.
            pub fn of(ch: char) -> Self {
                use $crate::tables::CharDataTable;
                const TABLE: CharDataTable<()> = include!($data_path);
                $prop_name(TABLE.contains(ch))
            }

            /// Get boolean property value of the character.
            pub fn as_bool(&self) -> bool { self.0 }
        }

        char_property! {
            __impl FromStr for $prop_name;
            // Yes
            y => $prop_name(true);
            yes => $prop_name(true);
            t => $prop_name(true);
            true => $prop_name(true);
            // No
            n => $prop_name(false);
            no => $prop_name(false);
            f => $prop_name(false);
            false => $prop_name(false);
        }

        char_property! {
            __impl CharProperty for $prop_name;
            $prop_abbr;
            $prop_long;
            $prop_human;
        }

        impl $crate::TotalCharProperty for $prop_name {
            fn of(ch: char) -> Self { Self::of(ch) }
        }

        impl $crate::BinaryCharProperty for $prop_name {
            fn as_bool(&self) -> bool { self.as_bool() }
        }

        impl From<$prop_name> for bool {
            fn from(prop: $prop_name) -> bool { prop.as_bool() }
        }

        char_property! {
            __impl Display for $prop_name by BinaryCharProperty
        }
    };

    // == Shared == //

    (
        __impl CharProperty for $prop_name:ident;
        $prop_abbr:expr;
        $prop_long:expr;
        $prop_human:expr;
    ) => {
        impl $crate::CharProperty for $prop_name {
            fn prop_abbr_name() -> &'static str { $prop_abbr }
            fn prop_long_name() -> &'static str { $prop_long }
            fn prop_human_name() -> &'static str { $prop_human }
        }
    };

    (
        __impl FromStr for $prop_name:ident;
        $( $id:ident => $value:expr; )*
    ) => {
        #[allow(unreachable_patterns)]
        impl $crate::__str::FromStr for $prop_name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // This is a temporary solution to enable case-insensitive matching under `no_std`
                // mode. Work is in progress to provide these functionalities under `libcore`.
                //
                // TODO: Drop whenever case-insensitive str matching is available in `libcore`.
                //
                // See: <https://github.com/rust-lang/rust/pull/44042>
                pub fn str_eq_ignore_ascii_case(this: &str, that: &str) -> bool {
                    this.len() == that.len() &&
                        this.bytes().zip(that.bytes()).all(|(a, b)| {
                            u8_to_ascii_lowercase(a) == u8_to_ascii_lowercase(b)
                        })
                }

                // From: <https://github.com/rust-lang/rust/.../src/libcore/num/mod.rs#L3473>
                // TODO: Drop after rust 1.21.0.
                static ASCII_LOWERCASE_MAP: [u8; 256] = [
                    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
                    b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'',
                    b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
                    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
                    b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
                    b'@',
                          b'a', b'b', b'c', b'd', b'e', b'f', b'g', //.
                    b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
                    b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
                    b'x', b'y', b'z',
                                     b'[', b'\\', b']', b'^', b'_', //.
                    b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
                    b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
                    b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
                    b'x', b'y', b'z', b'{', b'|', b'}', b'~', 0x7f,
                    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
                    0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
                    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
                    0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
                    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
                    0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
                    0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
                    0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
                    0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
                    0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
                    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7,
                    0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
                    0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7,
                    0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
                    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
                    0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
                ];

                // From: <https://github.com/rust-lang/rust/../src/libcore/num/mod.rs#L2321>
                // TODO: Drop after rust 1.21.0.
                #[inline]
                pub fn u8_to_ascii_lowercase(this: u8) -> u8 {
                    ASCII_LOWERCASE_MAP[this as usize]
                }


                match s {
                    // This stringify! should be moved out of this block to the call site. See the
                    // test failure https://travis-ci.org/behnam/rust-unic/builds/275758001 for why
                    // this is done here. This can be reverted at 1.20 adoption time.
                    $( stringify!($id) => Ok($value), )*
                    $( s if str_eq_ignore_ascii_case(s, stringify!($id)) => Ok($value), )*
                    _ => Err(()),
                }
            }
        }
    };

    ( __impl Display for $prop_name:ident by $trait:ident ) => {
        impl $crate::__fmt::Display for $prop_name {
            fn fmt(&self, f: &mut $crate::__fmt::Formatter) -> $crate::__fmt::Result {
                $crate::$trait::human_name(self).fmt(f)
            }
        }
    };
}
