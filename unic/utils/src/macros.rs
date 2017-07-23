/// Macro for declaring a character property.
///
/// # Syntax (Enumerated Property)
///
/// ```
/// # #[macro_use] extern crate unic_utils;
/// # fn main() {}
/// char_property! {
///     /// Zero or more attributes
///     pub enum PropertyName {
///         /// Exactly one attribute
///         RustName {
///             abbr => AbbrName,
///             long => Long_Name,
///             display => "&'static str that is a nicer presentation of the name",
///         }
///
///         /// All annotations on the variant are optional*
///         Variant2 {
///             abbr => V2, // *abbr is required for Enumerated Properties
///         }
///     }
///
///     /// Zero or more attributes
///     pub mod abbr_names;
///
///     /// Zero or more attributes
///     pub mod long_names;
/// }
///
/// // You must impl (Partial/Complete)CharProperty manually.
/// # impl unic_utils::char_property::PartialCharProperty for PropertyName {
/// #     fn of(_: char) -> Option<Self> { None }
/// # }
/// ```
///
/// # Effect
///
/// - Implements `CharProperty` with the `abbr` and `long` presented in the appropriate method
/// - Implements `FromStr` accepting any of the rust, abbr, or long names
/// - Implements `Display` using the given string, falling back when not provided on
///   the long name, the short name, and the rust name, in that order
/// - Populates the module `abbr_names` with `pub use` bindings of variants to their abbr names
/// - Populates the module `long_names` with `pub use` bindings of variants to their long names
/// - Maintains all documentation comments and other `#[attributes]` as would be expected
///   (with some caveats, listed below)
///
/// # Limitations
///
/// Due to [rust-lang/rust/#24189](https://github.com/rust-lang/rust/issues/24189), (fixed in
/// [rust-lang/rust/#42913](https://github.com/rust-lang/rust/pull/42913), landing in 1.20),
/// exactly one attribute line must be used on each variant. On 1.20 or higher, one or more may
/// be used, and the restriction can be relaxed back the intended zero or more by replacing
/// `$(#[$variant_meta:meta])+` with `$(#[$variant_meta:meta])*`, and
/// `$(#[$variant_meta])+` with `$(#[$variant_meta])*`, and
/// `$(#[$ident_meta:meta])+` with `$(#[$ident_meta:meta])*` and
/// `$(#[$ident_meta])+` with `$(#[$ident_meta])*`, and
/// `$(#[$rest_meta:meta])+` with `$(#[$rest_meta:meta])*`, and
/// `$(#[$rest_meta])+` with `$(#[$rest_meta])*`, and
/// `$(#[$queue_meta:meta])+` with `$(#[$queue_meta:meta])*`, and
/// `$(#[$queue_meta])+` with `$(#[$queue_meta])*`
// TODO: Once adopting 1.20, fix the macro to work with zero attributes on variants (see above)
#[macro_export]
macro_rules! char_property {
    (
        $(#[$name_meta:meta])* pub enum $name:ident {
            $( $(#[$variant_meta:meta])+ $variant:ident $tt:tt )*
        }

        $(#[$abbr_names_meta:meta])* pub mod $abbr_names:ident;
        $(#[$long_names_meta:meta])* pub mod $long_names:ident;
    ) => {
        __char_property_internal! {
            $(#[$name_meta])* pub enum $name
            $(#[$abbr_names_meta])* pub mod $abbr_names
            $(#[$long_names_meta])* pub mod $long_names

            variant [ ]
            abbr [ ]
            long [ ]
            display [ ]

            buffer [ ]
            queue [ $( $(#[$variant_meta])+ $variant $tt )* ]
        }
    };
}

#[macro_export]
macro_rules! __char_property_internal {
    // == Queue => Buffer == //
    (
        $(#[$name_meta:meta])* pub enum $name:ident
        $(#[$abbr_names_meta:meta])* pub mod $abbr_names:ident
        $(#[$long_names_meta:meta])* pub mod $long_names:ident

        variant [ $( $(#[$variant_meta:meta])+ $variant:ident ; )* ]
        abbr [ $( $abbr_variant:ident $abbr:ident ; )* ]
        long [ $( $long_variant:ident $long:ident ; )* ]
        display [ $( $display_variant:ident $display:expr ; )* ]

        buffer [  ]
        queue [
            $(#[$ident_meta:meta])+ $ident:ident $ident_tt:tt
            $( $(#[$rest_meta:meta])+ $rest:ident $rest_tt:tt )*
        ]
    ) => {
        __char_property_internal! {
            $(#[$name_meta])* pub enum $name
            $(#[$abbr_names_meta])* pub mod $abbr_names
            $(#[$long_names_meta])* pub mod $long_names

            variant [
                $( $(#[$variant_meta])+ $variant ; )*
                $(#[$ident_meta])+ $ident ;
            ]
            abbr [ $( $abbr_variant $abbr ; )* ]
            long [ $( $long_variant $long ; )* ]
            display [ $( $display_variant $display ; )* ]

            buffer [ $ident $ident_tt ]
            queue [ $( $(#[$rest_meta])+ $rest $rest_tt )* ]
        }
    };

    // == Buffer -- Abbr Name == //
    (
        $(#[$name_meta:meta])* pub enum $name:ident
        $(#[$abbr_names_meta:meta])* pub mod $abbr_names:ident
        $(#[$long_names_meta:meta])* pub mod $long_names:ident

        variant [ $( $(#[$variant_meta:meta])+ $variant:ident ; )* ]
        abbr [ $( $abbr_variant:ident $abbr:ident ; )* ]
        long [ $( $long_variant:ident $long:ident ; )* ]
        display [ $( $display_variant:ident $display:expr ; )* ]

        buffer [ $ident:ident {
            abbr => $ident_abbr:ident ,
            $( $rest:tt )*
        } ]
        queue [ $( $(#[$queue_meta:meta])+ $queue:ident $queue_tt:tt )* ]
    ) => {
        __char_property_internal! {
            $(#[$name_meta])* pub enum $name
            $(#[$abbr_names_meta])* pub mod $abbr_names
            $(#[$long_names_meta])* pub mod $long_names

            variant [ $( $(#[$variant_meta])+ $variant ; )* ]
            abbr [
                $( $abbr_variant $abbr ; )*
                $ident $ident_abbr ;
            ]
            long [ $( $long_variant $long ; )* ]
            display [ $( $display_variant $display ; )* ]

            buffer [ $ident { $( $rest )* } ]
            queue [ $( $(#[$queue_meta])+ $queue $queue_tt )* ]
        }
    };

    // == Buffer -- Long Name == //
    (
        $(#[$name_meta:meta])* pub enum $name:ident
        $(#[$abbr_names_meta:meta])* pub mod $abbr_names:ident
        $(#[$long_names_meta:meta])* pub mod $long_names:ident

        variant [ $( $(#[$variant_meta:meta])+ $variant:ident ; )* ]
        abbr [ $( $abbr_variant:ident $abbr:ident ; )* ]
        long [ $( $long_variant:ident $long:ident ; )* ]
        display [ $( $display_variant:ident $display:expr ; )* ]

        buffer [ $ident:ident {
            long => $ident_long:ident ,
            $( $rest:tt )*
        } ]
        queue [ $( $(#[$queue_meta:meta])+ $queue:ident $queue_tt:tt )* ]
    ) => {
        __char_property_internal! {
            $(#[$name_meta])* pub enum $name
            $(#[$abbr_names_meta])* pub mod $abbr_names
            $(#[$long_names_meta])* pub mod $long_names

            variant [ $( $(#[$variant_meta])+ $variant ; )* ]
            abbr [ $( $abbr_variant $abbr ; )* ]
            long [
                $( $long_variant $long ; )*
                $ident $ident_long ;
            ]
            display [ $( $display_variant $display ; )* ]

            buffer [ $ident { $( $rest )* } ]
            queue [ $( $(#[$queue_meta])+ $queue $queue_tt )* ]
        }
    };

    // == Buffer -- Display //
    (
        $(#[$name_meta:meta])* pub enum $name:ident
        $(#[$abbr_names_meta:meta])* pub mod $abbr_names:ident
        $(#[$long_names_meta:meta])* pub mod $long_names:ident

        variant [ $( $(#[$variant_meta:meta])+ $variant:ident ; )* ]
        abbr [ $( $abbr_variant:ident $abbr:ident ; )* ]
        long [ $( $long_variant:ident $long:ident ; )* ]
        display [ $( $display_variant:ident $display:expr ; )* ]

        buffer [ $ident:ident {
            display => $ident_display:expr ,
            $( $rest:tt )*
        } ]
        queue [ $( $(#[$queue_meta:meta])+ $queue:ident $queue_tt:tt )* ]
    ) => {
        __char_property_internal! {
            $(#[$name_meta])* pub enum $name
            $(#[$abbr_names_meta])* pub mod $abbr_names
            $(#[$long_names_meta])* pub mod $long_names

            variant [ $( $(#[$variant_meta])+ $variant ; )* ]
            abbr [ $( $abbr_variant $abbr ; )* ]
            long [ $( $long_variant $long ; )* ]
            display [
                $( $display_variant $display ; )*
                $ident $ident_display ;
            ]

            buffer [ $ident { $( $rest )* } ]
            queue [ $( $(#[$queue_meta])+ $queue $queue_tt )* ]
        }
    };

    // == Buffer -- Empty == //
    (
        $(#[$name_meta:meta])* pub enum $name:ident
        $(#[$abbr_names_meta:meta])* pub mod $abbr_names:ident
        $(#[$long_names_meta:meta])* pub mod $long_names:ident

        variant [ $( $(#[$variant_meta:meta])+ $variant:ident ; )* ]
        abbr [ $( $abbr_variant:ident $abbr:ident ; )* ]
        long [ $( $long_variant:ident $long:ident ; )* ]
        display [ $( $display_variant:ident $display:expr ; )* ]

        buffer [ $ident:ident {} ]
        queue [ $( $(#[$queue_meta:meta])+ $queue:ident $queue_tt:tt )* ]
    ) => {
        __char_property_internal! {
            $(#[$name_meta])* pub enum $name
            $(#[$abbr_names_meta])* pub mod $abbr_names
            $(#[$long_names_meta])* pub mod $long_names

            variant [ $( $(#[$variant_meta])+ $variant ; )* ]
            abbr [ $( $abbr_variant $abbr ; )* ]
            long [ $( $long_variant $long ; )* ]
            display [ $( $display_variant $display ; )* ]

            buffer [ ]
            queue [ $( $(#[$queue_meta])+ $queue $queue_tt )* ]
        }
    };

    // == Final formatting == //
    (
        $(#[$name_meta:meta])* pub enum $name:ident
        $(#[$abbr_names_meta:meta])* pub mod $abbr_names:ident
        $(#[$long_names_meta:meta])* pub mod $long_names:ident

        variant [ $( $(#[$variant_meta:meta])+ $variant:ident ; )* ]
        abbr [ $( $abbr_variant:ident $abbr:ident ; )* ]
        long [ $( $long_variant:ident $long:ident ; )* ]
        display [ $( $display_variant:ident $display:expr ; )* ]

        buffer [ ]
        queue [ ]
    ) => {
        $(#[$name_meta])*
        #[allow(bad_style)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $name {
            $( $(#[$variant_meta])+ $variant, )*
        }

        $(#[$abbr_names_meta])*
        #[allow(bad_style)]
        pub mod $abbr_names {
            $( pub use super::$name::$abbr_variant as $abbr; )*
        }

        $(#[$long_names_meta])*
        #[allow(bad_style)]
        pub mod $long_names {
            $( pub use super::$name::$long_variant as $long; )*
        }

        #[allow(bad_style)]
        #[allow(unreachable_patterns)]
        impl ::std::str::FromStr for $name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( stringify!($variant) => Ok($name::$variant), )*
                    $( stringify!($abbr) => Ok($name::$abbr_variant), )*
                    $( stringify!($long) => Ok($name::$long_variant), )*
                    _ => Err(()),
                }
            }
        }

        #[allow(bad_style)]
        #[allow(unreachable_patterns)]
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $( $name::$display_variant => write!(f, "{}", $display), )*
                    $( $name::$long_variant => write!(f, "{}", stringify!($long).replace('_', " ")), )*
                    _ => write!(f, "{}", match *self {
                        $( $name::$abbr_variant => stringify!($abbr), )*
                        $( $name::$variant => stringify!($variant), )*
                    })
                }
            }
        }

        #[allow(bad_style)]
        #[allow(unreachable_patterns)]
        impl $crate::char_property::EnumeratedCharProperty for $name {
            fn abbr_name(&self) -> &'static str {
                match *self {
                    $( $name::$abbr_variant => stringify!($abbr), )*
                    // No catch all variant
                    // Abbr name is required on Enumerated properties
                }
            }

            fn all_values() -> &'static [$name] {
                const VALUES: &[$name] = &[
                    $($name::$variant,)+
                ];
                VALUES
            }
        }
    };
}
