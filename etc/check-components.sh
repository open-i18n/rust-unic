#!/usr/bin/env bash

# Copyright 2017 The UNIC Project Developers.
#
# See the COPYRIGHT file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
. "$DIR/common.sh"


# Check all components
for component in $COMPONENTS; do
    lib_rs="$component/src/lib.rs"
    pkg_info_rs="$component/src/pkg_info.rs"

    # Require library attribute settings
    # TODO:
    #   warnings (all lints that are set to issue warnings)
    #   missing-copy-implementations
    #   unreachable-pub
    #   unused-results
    ATTRS='
        bad_style
        missing_debug_implementations
        missing_docs
        unconditional_recursion
        unsafe_code
        unused
    '
    for attr in $ATTRS; do
        if grep --quiet "$attr" $lib_rs; then true; else
            echo "$component: missing '$attr' attribute setting"
        fi
    done

    # Package Info
    if [ ! -f "$pkg_info_rs" ]; then
        echo "$component: missing 'pkg_info.rs'"
    fi

done
