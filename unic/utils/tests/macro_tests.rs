#[macro_use]
extern crate unic_utils;

use unic_utils::char_property::EnumeratedCharProperty;

char_property! {
    pub enum Property {
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

impl unic_utils::char_property::PartialCharProperty for Property {
    fn of(_: char) -> Option<Self> {
        None
    }
}

#[test]
fn basic_macro_use() {
    assert_eq!(Property::AbbrVariant, abbr_names::AV);
    assert_eq!(Property::LongVariant, abbr_names::LV);
    assert_eq!(Property::DisplayVariant, abbr_names::DV);
    assert_eq!(Property::EmptyVariant, abbr_names::EV);

    assert_eq!(Property::LongVariant, long_names::Long_Variant);

    assert_eq!(Property::AbbrVariant.abbr_name(), "AV");
    assert_eq!(Property::LongVariant.abbr_name(), "LV");
    assert_eq!(Property::DisplayVariant.abbr_name(), "DV");
    assert_eq!(Property::EmptyVariant.abbr_name(), "EV");

    assert_eq!(format!("{}", Property::AbbrVariant), "AV");
    assert_eq!(format!("{}", Property::LongVariant), "Long Variant");
    assert_eq!(format!("{}", Property::DisplayVariant), "The one and only DISPLAY VARIANT!");
    assert_eq!(format!("{}", Property::EmptyVariant), "EV");
}

#[test]
fn fromstr_ignores_case() {
    use abbr_names::LV;
    assert_eq!("long_variant".parse(), Ok(LV));
    assert_eq!("lOnG_vArIaNt".parse(), Ok(LV));
    assert_eq!("LoNg_VaRiAnT".parse(), Ok(LV));
    assert_eq!("LONG_VARIANT".parse(), Ok(LV));
}
