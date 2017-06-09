# Copyright 2017 The UNIC Project Developers.
#
# See the COPYRIGHT file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
# or http://opensource.org/licenses/MIT>, at your option. This file may not be
# copied, modified, or distributed except according to those terms.


import sys


_SURROGATE_CODEPOINTS_RANGE = (0xD800, 0xDFFF + 1)
_TRAILING_SURROGATE_START = 0xDC00


def is_surrogate(n):
    return _SURROGATE_CODEPOINTS_RANGE[0] <= n < _SURROGATE_CODEPOINTS_RANGE[1]


def cp_to_char(codepoint):
    if sys.maxunicode == 0x10FFFF:
        return unichr(codepoint)
    else:
        return ("\\U%08x" % codepoint).decode('unicode-escape')


def codepoints_from_string(unistr):
    if sys.maxunicode == 0x10FFFF:
        return [ord(ch) for ch in unistr]
    else:
        return [cp for cp in _codepoints_from_utf16(unistr)]


def _is_leading_surrogate(code_unit):
    return _SURROGATE_CODEPOINTS_RANGE[0] <= code_unit < _TRAILING_SURROGATE_START


def _is_trailing_surrogate(code_unit):
    return _TRAILING_SURROGATE_START <= code_unit < _SURROGATE_CODEPOINTS_RANGE[1]


def _decode_surrogate_pair(leading, trailing):
    return ((leading - _SURROGATE_CODEPOINTS_RANGE[0]) << 10) + (trailing - 0xDC00) + 0x10000


def _codepoints_from_utf16(unistr):
    assert sys.maxunicode == 0xFFFF

    leading = None
    for code_unit_char in unistr:
        code_unit = ord(code_unit_char)

        # If leading surrogate is seen, handle that first
        if leading is not None:
            if _is_trailing_surrogate(code_unit):
                # Found a surrogate pair
                yield _decode_surrogate_pair(leading, code_unit)
                leading = None
                continue
            else:
                # Orphan leading surrogate
                yield leading
                leading = None

        # Then handle current code_unit, if not part of the pair
        if _is_leading_surrogate(code_unit):
            leading = code_unit
        else:
            yield code_unit

    # Orphan leading surrogate at the end of input
    if leading is not None:
        yield leading
