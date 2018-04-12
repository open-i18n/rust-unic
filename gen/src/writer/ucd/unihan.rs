// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::path::Path;

use source::ucd::readme::UNICODE_VERSION;
use source::ucd::unihan::numeric_values::UNIHAN_NUMERIC_VALUES_DATA;
use source::ucd::unihan::readings::UNIHAN_READINGS_DATA;
use source::ucd::unihan::variants::UNIHAN_VARIANTS_DATA;

use writer::common::emit_unicode_version;
use writer::utils::tables::ToDirectCharTable;
use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_unihan_numeric_values_tables(dir);
    emit_unihan_readings_tables(dir);
    emit_unihan_variants_tables(dir);
}

fn emit_unihan_numeric_values_tables(dir: &Path) {
    let mut accounting_numeric_map = BTreeMap::default();
    let mut primary_numeric_map = BTreeMap::default();
    let mut other_numeric_map = BTreeMap::default();

    for entry in UNIHAN_NUMERIC_VALUES_DATA.entries.iter() {
        if let Some(value) = entry.accounting_numeric {
            accounting_numeric_map.insert(entry.character, value);
        }
        if let Some(value) = entry.primary_numeric {
            primary_numeric_map.insert(entry.character, value);
        }
        if let Some(value) = entry.other_numeric {
            other_numeric_map.insert(entry.character, value);
        }
    }

    write(
        dir,
        "accounting_numeric_map.rsv",
        &accounting_numeric_map.to_direct_char_table(|record, f| write!(f, "{}", record)),
    );
    write(
        dir,
        "primary_numeric_map.rsv",
        &primary_numeric_map.to_direct_char_table(|record, f| write!(f, "{}", record)),
    );
    write(
        dir,
        "other_numeric_map.rsv",
        &other_numeric_map.to_direct_char_table(|record, f| write!(f, "{}", record)),
    );
}

fn emit_unihan_readings_tables(dir: &Path) {
    let mut cantonese_map = BTreeMap::default();
    let mut definition_map = BTreeMap::default();
    let mut hangul_map = BTreeMap::default();
    let mut hanyu_pinlu_map = BTreeMap::default();
    let mut hanyu_pinyin_map = BTreeMap::default();
    let mut japanese_kun_map = BTreeMap::default();
    let mut japanese_on_map = BTreeMap::default();
    let mut korean_map = BTreeMap::default();
    let mut mandarin_map = BTreeMap::default();
    let mut tang_map = BTreeMap::default();
    let mut vietnamese_map = BTreeMap::default();
    let mut xhc_1983_map = BTreeMap::default();

    for entry in UNIHAN_READINGS_DATA.entries.iter() {
        if let Some(ref value) = entry.cantonese {
            cantonese_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.definition {
            definition_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.hangul {
            hangul_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.hanyu_pinlu {
            hanyu_pinlu_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.hanyu_pinyin {
            hanyu_pinyin_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.japanese_kun {
            japanese_kun_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.japanese_on {
            japanese_on_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.korean {
            korean_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.mandarin {
            mandarin_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.tang {
            tang_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.vietnamese {
            vietnamese_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.xhc_1983 {
            xhc_1983_map.insert(entry.character, value);
        }
    }

    write(
        dir,
        "cantonese_map.rsv",
        &cantonese_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "definition_map.rsv",
        &definition_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "hangul_map.rsv",
        &hangul_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "hanyu_pinlu_map.rsv",
        &hanyu_pinlu_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "hanyu_pinyin_map.rsv",
        &hanyu_pinyin_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "japanese_kun_map.rsv",
        &japanese_kun_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "japanese_on_map.rsv",
        &japanese_on_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "korean_map.rsv",
        &korean_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "mandarin_map.rsv",
        &mandarin_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "tang_map.rsv",
        &tang_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "vietnamese_map.rsv",
        &vietnamese_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
    write(
        dir,
        "xhc_1983_map.rsv",
        &xhc_1983_map.to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
}

fn emit_unihan_variants_tables(dir: &Path) {
    let mut semantic_variants_map = BTreeMap::default();
    let mut simplified_variant_map = BTreeMap::default();
    let mut specialized_semantic_variants_map = BTreeMap::default();
    let mut traditional_variant_map = BTreeMap::default();
    let mut z_variants_map = BTreeMap::default();

    for entry in UNIHAN_VARIANTS_DATA.entries.iter() {
        if let Some(ref value) = entry.semantic_variants {
            semantic_variants_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.simplified_variant {
            simplified_variant_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.specialized_semantic_variants {
            specialized_semantic_variants_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.traditional_variant {
            traditional_variant_map.insert(entry.character, value);
        }
        if let Some(ref value) = entry.z_variants {
            z_variants_map.insert(entry.character, value);
        }
    }

    write(
        dir,
        "semantic_variants_map.rsv",
        &semantic_variants_map.to_direct_char_table(|record, f| write!(f, "{:?}", record)),
    );
    write(
        dir,
        "simplified_variant_map.rsv",
        &simplified_variant_map.to_direct_char_table(|record, f| write!(f, "'{}'", record)),
    );
    write(
        dir,
        "specialized_semantic_variants_map.rsv",
        &specialized_semantic_variants_map.to_direct_char_table(|record, f| write!(f, "{:?}", record)),
    );
    write(
        dir,
        "traditional_variant_map.rsv",
        &traditional_variant_map.to_direct_char_table(|record, f| write!(f, "'{}'", record)),
    );
    write(
        dir,
        "z_variants_map.rsv",
        &z_variants_map.to_direct_char_table(|record, f| write!(f, "{:?}", record)),
    );
}
