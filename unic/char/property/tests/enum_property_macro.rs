// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#[macro_use]
extern crate unic_char_property;


use unic_char_property::PartialCharProperty;


char_property! {
    pub enum MyProp {
        abbr => "mp";
        long => "My_Prop";
        human => "My Property";

        /// Required
        Variant1 {
            abbr => V1,
            long => Variant_1,
            human => "Variant 1",
        }

        /// Required
        Variant2 {
            abbr => V2,
            long => Variant_2,
            human => "Variant 2",
        }

        /// Required
        Variant3 {
            abbr => V3,
            long => Variant_3,
            human => "Variant 3",
        }
    }

    pub mod abbr for abbr;
    pub mod long for long;
}


impl PartialCharProperty for MyProp {
    fn of(_: char) -> Option<Self> {
        None
    }
}


#[test]
fn basic_macro_use() {
    use unic_char_property::EnumeratedCharProperty;

    assert_eq!(MyProp::Variant1, abbr::V1);
    assert_eq!(MyProp::Variant2, abbr::V2);
    assert_eq!(MyProp::Variant3, abbr::V3);

    assert_eq!(MyProp::Variant1, long::Variant_1);
    assert_eq!(MyProp::Variant2, long::Variant_2);
    assert_eq!(MyProp::Variant3, long::Variant_3);

    assert_eq!(MyProp::Variant1.abbr_name(), "V1");
    assert_eq!(MyProp::Variant2.abbr_name(), "V2");
    assert_eq!(MyProp::Variant3.abbr_name(), "V3");

    assert_eq!(MyProp::Variant1.long_name(), "Variant_1");
    assert_eq!(MyProp::Variant2.long_name(), "Variant_2");
    assert_eq!(MyProp::Variant3.long_name(), "Variant_3");

    assert_eq!(MyProp::Variant1.human_name(), "Variant 1");
    assert_eq!(MyProp::Variant2.human_name(), "Variant 2");
    assert_eq!(MyProp::Variant3.human_name(), "Variant 3");
}

#[test]
fn fromstr_ignores_case() {
    use abbr::V1;

    assert_eq!("variant_1".parse(), Ok(V1));
    assert_eq!("VaRiAnT_1".parse(), Ok(V1));
    assert_eq!("vArIaNt_1".parse(), Ok(V1));
    assert_eq!("VARIANT_1".parse(), Ok(V1));
}
