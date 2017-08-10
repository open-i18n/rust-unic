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
extern crate unic_utils;


use unic_utils::char_property::EnumeratedCharProperty;


char_property! {
    pub enum MyProp {
        /// Required
        AbbrVariant {
            abbr => AV,
        }
        /// Required
        LongVariant {
            abbr => LV,
            long => Long_Variant,
        }
        /// Required
        DisplayVariant {
            abbr => DV,
            display => "The one and only DISPLAY VARIANT!",
        }
        /// Required
        EmptyVariant {
            abbr => EV,
        }
    }

    pub mod abbr_names for abbr;
    pub mod long_names for long;
}


impl unic_utils::char_property::PartialCharProperty for MyProp {
    fn of(_: char) -> Option<Self> {
        None
    }
}


#[test]
fn basic_macro_use() {
    assert_eq!(MyProp::AbbrVariant, abbr_names::AV);
    assert_eq!(MyProp::LongVariant, abbr_names::LV);
    assert_eq!(MyProp::DisplayVariant, abbr_names::DV);
    assert_eq!(MyProp::EmptyVariant, abbr_names::EV);

    assert_eq!(MyProp::LongVariant, long_names::Long_Variant);

    assert_eq!(MyProp::AbbrVariant.abbr_name(), "AV");
    assert_eq!(MyProp::LongVariant.abbr_name(), "LV");
    assert_eq!(MyProp::DisplayVariant.abbr_name(), "DV");
    assert_eq!(MyProp::EmptyVariant.abbr_name(), "EV");

    assert_eq!(format!("{}", MyProp::AbbrVariant), "AV");
    assert_eq!(format!("{}", MyProp::LongVariant), "Long Variant");
    assert_eq!(
        format!("{}", MyProp::DisplayVariant),
        "The one and only DISPLAY VARIANT!"
    );
    assert_eq!(format!("{}", MyProp::EmptyVariant), "EV");
}

#[test]
fn fromstr_ignores_case() {
    use abbr_names::LV;
    assert_eq!("long_variant".parse(), Ok(LV));
    assert_eq!("lOnG_vArIaNt".parse(), Ok(LV));
    assert_eq!("LoNg_VaRiAnT".parse(), Ok(LV));
    assert_eq!("LONG_VARIANT".parse(), Ok(LV));
}
