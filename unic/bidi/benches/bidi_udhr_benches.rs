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

#![cfg(all(test, feature = "bench_it"))]
#![feature(test)]

extern crate test;

use test::Bencher;

use unic_bidi::BidiInfo;

const LTR_TEXTS: &[&str] = &[
    include_str!("../../../external/unicode/udhr/txt/udhr_acu_1.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_auc.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_eng.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_knc.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_krl.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_lot.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_mly_latn.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_piu.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_qug.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_snn.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_tiv.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_uig_latn.txt"),
];

const BIDI_TEXTS: &[&str] = &[
    include_str!("../../../external/unicode/udhr/txt/udhr_aii.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_arb.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_mly_arab.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_pes_1.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_skr.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_urd.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_pes_2.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_uig_arab.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_urd_2.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_heb.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_pnb.txt"),
    include_str!("../../../external/unicode/udhr/txt/udhr_ydd.txt"),
];

fn bench_bidi_info_new(b: &mut Bencher, texts: &[&str]) {
    for text in texts {
        b.iter(|| {
            BidiInfo::new(text, None);
        });
    }
}

fn bench_reorder_line(b: &mut Bencher, texts: &[&str]) {
    for text in texts {
        let bidi_info = BidiInfo::new(text, None);
        b.iter(|| {
            for para in &bidi_info.paragraphs {
                let line = para.range.clone();
                bidi_info.reorder_line(para, line);
            }
        });
    }
}

#[bench]
fn bench_1_bidi_info_new_for_ltr_texts(b: &mut Bencher) {
    bench_bidi_info_new(b, LTR_TEXTS);
}

#[bench]
fn bench_2_bidi_info_new_for_bidi_texts(b: &mut Bencher) {
    bench_bidi_info_new(b, BIDI_TEXTS);
}

#[bench]
fn bench_3_reorder_line_for_ltr_texts(b: &mut Bencher) {
    bench_reorder_line(b, LTR_TEXTS);
}

#[bench]
fn bench_4_reorder_line_for_bidi_texts(b: &mut Bencher) {
    bench_reorder_line(b, BIDI_TEXTS);
}
