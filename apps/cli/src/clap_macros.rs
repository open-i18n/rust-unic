// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! unic_arg_enum {
    (@as_item $($i:item)*) => ($($i)*);

    (@impls ( $($tts:tt)* ) -> ($e:ident, $($v:ident),+)) => {
        unic_arg_enum!(@as_item
        $($tts)*

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> ::std::result::Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.replace("-", "").eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v)),+,
                    _ => Err({
                        let v = vec![
                            $(stringify!($v),)+
                        ];
                        format!("valid values: {}",
                            v.join(" ,"))
                    }),
                }
            }
        }
        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($e::$v => write!(f, stringify!($v)),)+
                }
            }
        }
        impl $e {
            #[allow(dead_code)]
            pub fn variants() -> [&'static str; _clap_count_exprs!($(stringify!($v)),+)] {
                [
                    $(stringify!($v),)+
                ]
            }
        });
    };

    ($(#[$($m:meta),+])+ pub enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        unic_arg_enum!(@impls
            ($(#[$($m),+])+
            pub enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };

    ($(#[$($m:meta),+])+ enum $e:ident { $($v:ident $(=$val:expr)*),+  } ) => {
        unic_arg_enum!(@impls
            ($(#[$($m),+])+
            enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };

    (pub enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        unic_arg_enum!(@impls
            (pub enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };

    (enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        unic_arg_enum!(@impls
            (enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
}
