// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Based on *General Scope* of *Bidirectional Character Types* table.
//!
//! Reference: <http://www.unicode.org/reports/tr9/#Table_Bidirectional_Character_Types>


#[macro_use]
extern crate matches;

#[macro_use]
extern crate unic_char_range;

extern crate unic_ucd;


use unic_ucd::bidi::BidiClass as BC;
use unic_ucd::category::GeneralCategory as GC;


#[test]
fn test_from_bidi_class() {
    for cp in chars!(..) {
        match BC::of(cp) {
            // == Strong ==

            // <http://www.unicode.org/reports/tr9/#L>
            BC::LeftToRight => {
                // TODO: Impl using Script property, etc
            }

            // <http://www.unicode.org/reports/tr9/#R>
            BC::RightToLeft => {
                // TODO: Impl using Script property, etc
            }

            // <http://www.unicode.org/reports/tr9/#AL>
            BC::ArabicLetter => {
                // TODO: Impl using Script property, etc
            }

            // == Weak ==

            // <http://www.unicode.org/reports/tr9/#EN>
            BC::EuropeanNumber => {
                assert!(GC::of(cp).is_number());
            }

            // <http://www.unicode.org/reports/tr9/#ES>
            BC::EuropeanSeparator => {
                assert!(matches!(GC::of(cp), GC::MathSymbol | GC::DashPunctuation));
            }

            // <http://www.unicode.org/reports/tr9/#ET>
            BC::EuropeanTerminator => {
                assert!(
                    GC::of(cp).is_symbol()
                        || matches!(GC::of(cp), GC::Unassigned | GC::OtherPunctuation)
                );
            }

            // <http://www.unicode.org/reports/tr9/#AN>
            BC::ArabicNumber => {
                assert!(matches!(
                    GC::of(cp),
                    GC::Format | GC::OtherNumber | GC::OtherPunctuation | GC::DecimalNumber
                ));
            }

            // <http://www.unicode.org/reports/tr9/#CS>
            BC::CommonSeparator => {
                assert!(matches!(
                    GC::of(cp),
                    GC::OtherPunctuation | GC::SpaceSeparator | GC::MathSymbol
                ));
            }

            // Every NSM must be a GC=Mark
            //
            // <http://www.unicode.org/reports/tr9/#NSM>
            BC::NonspacingMark => {
                assert!(GC::of(cp).is_mark());
            }

            // <http://www.unicode.org/reports/tr9/#BN>
            BC::BoundaryNeutral => {
                assert!(GC::of(cp).is_other());
            }

            // == Neutral ==

            // <http://www.unicode.org/reports/tr9/#B>
            BC::ParagraphSeparator => {
                assert!(matches!(GC::of(cp), GC::Control | GC::ParagraphSeparator));
            }

            // <http://www.unicode.org/reports/tr9/#S>
            BC::SegmentSeparator => {
                assert!(matches!(GC::of(cp), GC::Control));
            }

            // <http://www.unicode.org/reports/tr9/#WS>
            BC::WhiteSpace => {
                assert!(matches!(
                    GC::of(cp),
                    GC::Control | GC::SpaceSeparator | GC::LineSeparator
                ));
            }

            // <http://www.unicode.org/reports/tr9/#ON>
            BC::OtherNeutral => {
                assert!(!matches!(
                    GC::of(cp),
                    GC::UppercaseLetter | GC::LowercaseLetter | GC::TitlecaseLetter
                        | GC::OtherLetter | GC::NonspacingMark | GC::SpacingMark
                        | GC::EnclosingMark | GC::DecimalNumber
                        | GC::SpaceSeparator | GC::LineSeparator
                        | GC::ParagraphSeparator
                ));
            }

            // == Explicit Formatting ==
            BC::LeftToRightEmbedding |
            BC::LeftToRightIsolate |
            BC::LeftToRightOverride |
            BC::RightToLeftEmbedding |
            BC::RightToLeftIsolate |
            BC::RightToLeftOverride |
            BC::FirstStrongIsolate |
            BC::PopDirectionalFormat |
            BC::PopDirectionalIsolate => {
                assert_eq!(GC::of(cp), GC::Format);
            }
        }
    }
}


#[test]
fn test_from_general_category() {
    for cp in chars!(..) {
        if !GC::of(cp).is_mark() {
            // Every GC!=Mark must not be an NSM
            //
            // <http://www.unicode.org/reports/tr9/#NSM>
            assert_ne!(BC::of(cp), BC::NonspacingMark);
        }
    }
}
