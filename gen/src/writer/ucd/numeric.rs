use std::collections::BTreeMap;
use std::convert::TryInto;
use std::path::Path;

use crate::source::ucd::readme::UNICODE_VERSION;
use crate::source::ucd::unicode_data::UNICODE_DATA;

use crate::writer::common::emit_unicode_version;
use crate::writer::utils::tables::ToDirectCharTable;
use crate::writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_digit_decimal_numeric(dir);
}

fn emit_digit_decimal_numeric(dir: &Path) {
    let mut str_reprs = String::new();
    // we pack as much info as possible into a u32, since due to char's alignment we get 32 bits of
    // info for "free" - even with V=u8, size_of::<[(char, u8); N]>() = 64 * N
    let map: BTreeMap<char, u32> = UNICODE_DATA
        .entries
        .iter()
        .filter_map(|x| {
            let num_str = x.numeric_numeric_value.as_deref()?;
            let i = match x.digit_numeric_value {
                Some(d) => {
                    assert!(matches!(d, 0..=9));
                    let is_decimal = x.decimal_numeric_value.is_some();
                    (1u32 << 31) | ((is_decimal as u32) << 30) | (d as u32)
                }
                None => {
                    // if we made V something with `&'static str`, it would almost double the size
                    // of the table in static memory. instead keep the string data separate, which
                    // allows packing a short idx/len into a u32, and also lets us deduplicate data
                    let idx = str_reprs.find(num_str).unwrap_or_else(|| {
                        let i = str_reprs.len();
                        str_reprs.push_str(num_str);
                        i
                    });
                    let idx: u16 = idx.try_into().unwrap();
                    let len: u16 = num_str.len().try_into().unwrap();
                    assert_eq!(len >> 15, 0);
                    ((len as u32) << 16) | (idx as u32)
                }
            };
            Some((x.character, i))
        })
        .collect();

    write(dir, "numeric_strs.rsv", &format!("{:?}", str_reprs));
    write(
        dir,
        "numeric_values.rsv",
        &map.to_direct_char_table(|val, f| write!(f, "{:#x?}", val)),
    );
}
