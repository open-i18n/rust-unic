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
// rustc:1.17.0 cannot find data_table_path, so ignore this test for now.
/// ```ignore
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
/// # Limitations
///
/// Due to [rust-lang/rust#24189](https://github.com/rust-lang/rust/issues/24189), (fixed in
/// [rust-lang/rust#42913](https://github.com/rust-lang/rust/pull/42913), landing in 1.20),
/// exactly one attribute line must be used on each variant.
///
/// On 1.20 or higher, one or more may be used, and the restriction can be relaxed back to
/// the intended zero or more by replacing `$(#[$variant_meta:meta])+` with
/// `$(#[$variant_meta:meta])*` and `$(#[$variant_meta])+` with `$(#[$variant_meta])*`.
// TODO: Once adopting 1.20, fix the macro to work with zero attributes on variants (see above).
#[macro_export]
macro_rules! char_property {

    // == Enumerated Property == //

    (
        $(#[$name_meta:meta])*
        pub enum $name:ident {
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

        $(#[$abbr_mod_meta:meta])*
        pub mod $abbr_mod:ident for abbr;

        $(#[$long_mod_meta:meta])*
        pub mod $long_mod:ident for long;

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

        char_property! {
            __impl FromStr for $name;
            $(
                $abbr => $name::$variant;
                $long => $name::$variant;
            )*
        }

        char_property! {
            __impl CharProperty for $name;
            $abbr_name;
            $long_name;
            $human_name;
        }

        char_property! {
            __impl Display for $name by EnumeratedCharProperty
        }

        impl $crate::EnumeratedCharProperty for $name {
            fn all_values() -> &'static [$name] {
                const VALUES: &[$name] = &[ $($name::$variant,)+ ];
                VALUES
            }
            fn abbr_name(&self) -> &'static str {
                match *self { $( $name::$variant => stringify!($abbr), )* }
            }
            fn long_name(&self) -> &'static str {
                match *self { $( $name::$variant => stringify!($long), )* }
            }
            fn human_name(&self) -> &'static str {
                match *self { $( $name::$variant => $human, )* }
            }
        }
    };

    // == Binary Property == //

    (
        $(#[$name_meta:meta])*
        pub struct $name:ident(bool) {
            abbr => $abbr_name:expr;
            long => $long_name:expr;
            human => $human_name:expr;

            data_table_path => $data_path:expr;
        }

        $(#[$is_fn_meta:meta])*
        pub fn $is_fn:ident(char) -> bool;

    ) => {
        $(#[$name_meta])*
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
        pub struct $name(bool);

        $(#[$is_fn_meta])*
        pub fn $is_fn(ch: char) -> bool {
            $name::of(ch).as_bool()
        }

        impl $name {
            /// Get (struct) property value of the character.
            pub fn of(ch: char) -> Self {
                use $crate::unic_utils::CharDataTable;
                const TABLE: CharDataTable<()> = include!($data_path);
                $name(TABLE.contains(ch))
            }

            /// Get boolean property value of the character.
            pub fn as_bool(&self) -> bool { self.0 }
        }

        char_property! {
            __impl FromStr for $name;
            // Yes
            y => $name(true);
            yes => $name(true);
            t => $name(true);
            true => $name(true);
            // No
            n => $name(false);
            no => $name(false);
            f => $name(false);
            false => $name(false);
        }

        char_property! {
            __impl CharProperty for $name;
            $abbr_name;
            $long_name;
            $human_name;
        }

        impl $crate::TotalCharProperty for $name {
            fn of(ch: char) -> Self { Self::of(ch) }
        }

        impl $crate::BinaryCharProperty for $name {
            fn bool(&self) -> bool { self.as_bool() }
        }

        impl From<$name> for bool {
            fn from(prop: $name) -> bool { prop.as_bool() }
        }

        char_property! {
            __impl Display for $name by BinaryCharProperty
        }
    };

    // == Shared == //

    (
        __impl CharProperty for $name:ident;
        $abbr:expr;
        $long:expr;
        $human:expr;
    ) => {
        impl $crate::CharProperty for $name {
            fn prop_abbr_name() -> &'static str { $abbr }
            fn prop_long_name() -> &'static str { $long }
            fn prop_human_name() -> &'static str { $human }
        }
    };

    (
        __impl FromStr for $name:ident;
        $( $id:ident => $value:expr; )*
    ) => {
        #[allow(unreachable_patterns)]
        impl ::std::str::FromStr for $name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    // This stringify! should be moved out of this block to the call site. See the
                    // test failure https://travis-ci.org/behnam/rust-unic/builds/275758001 for why
                    // this is done here. This can be reverted at 1.20 adoption time.
                    $( stringify!($id) => Ok($value), )*
                    $( s if ::std::ascii::AsciiExt::eq_ignore_ascii_case(s, stringify!($id))
                         => Ok($value), )*
                    _ => Err(()),
                }
            }
        }
    };

    ( __impl Display for $name:ident by $trait:ident ) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::$trait::human_name(self).fmt(f)
            }
        }
    };
}
