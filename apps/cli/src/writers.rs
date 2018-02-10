// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::str::Chars;

use super::Result;

// TODO(FUTURE_RUST): Replace with char::MAX_UTF*_LEN when available.
const MAX_UTF8_LEN: usize = 4; // x u8 per char
const MAX_UTF16_LEN: usize = 2; // x u16 per char

macro_rules! write_join_with_space {
    ($output:expr, $values:expr, $write_fn:expr) => {{
        let mut first = true;
        for v in $values {
            if !first {
                write!($output, " ")?;
            }
            first = false;
            ($write_fn)($output, v)?;
        }
        Ok(())
    }};
}

// == Single Char ==

pub fn write_char_as_codepoint<W: io::Write>(output: &mut W, ch: char) -> Result<()> {
    write!(output, "U+{:04X}", ch as u32)
}

pub fn write_char_as_utf8_hex<W: io::Write>(output: &mut W, ch: char) -> Result<()> {
    let mut buf = [0; MAX_UTF8_LEN];
    let utf8 = ch.encode_utf8(&mut buf).as_bytes();
    write_join_with_space!(output, utf8.iter(), |output: &mut W, b8| -> Result<()> {
        write!(output, "0x{:02X}", b8)
    })
}

pub fn write_char_as_utf16_hex<W: io::Write>(output: &mut W, ch: char) -> Result<()> {
    let mut buf = [0; MAX_UTF16_LEN];
    let utf16 = ch.encode_utf16(&mut buf);
    write_join_with_space!(output, utf16.iter(), |output: &mut W, b16| -> Result<()> {
        write!(output, "0x{:04X}", b16)
    })
}

// == Multiple Chars ==

pub fn write_as_codepoints<'a, W: io::Write>(output: &mut W, chars: Chars<'a>) -> Result<()> {
    write_join_with_space!(output, chars, write_char_as_codepoint)
}

pub fn write_as_utf8_hex<'a, W: io::Write>(output: &mut W, chars: Chars<'a>) -> Result<()> {
    write_join_with_space!(output, chars, write_char_as_utf8_hex)
}

pub fn write_as_utf16_hex<'a, W: io::Write>(output: &mut W, chars: Chars<'a>) -> Result<()> {
    write_join_with_space!(output, chars, write_char_as_utf16_hex)
}

pub fn write_with_control_n_unicode_braces_escape<'a, W: io::Write>(
    output: &mut W,
    chars: Chars<'a>,
) -> Result<()> {
    write!(output, "\"")?;
    for ch in chars {
        write!(output, "{}", ch.escape_default())?;
    }
    write!(output, "\"")?;
    Ok(())
}

pub fn write_with_control_braces_escape<'a, W: io::Write>(
    output: &mut W,
    chars: Chars<'a>,
) -> Result<()> {
    write!(output, "\"")?;
    for ch in chars {
        write!(output, "{}", ch.escape_debug())?;
    }
    write!(output, "\"")?;
    Ok(())
}

pub fn write_with_all_braces_escape<'a, W: io::Write>(
    output: &mut W,
    chars: Chars<'a>,
) -> Result<()> {
    write!(output, "\"")?;
    for ch in chars {
        write!(output, "{}", ch.escape_unicode())?;
    }
    write!(output, "\"")?;
    Ok(())
}
