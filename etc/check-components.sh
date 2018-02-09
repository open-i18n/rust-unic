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

# Since `cargo publish --all` does not exist yet, we use this dumb alternative
# solution for now.
#
# Main downside of this approch is that there are separate `target/`
# directories used for each component, increasing the test and publish process
# time.

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
. "$DIR/common.sh"


# Steps

# Check all components
for component in $COMPONENTS; do

    # Require Documentation
    lib_rs="$component/src/lib.rs"
    if grep --quiet 'missing_docs' $lib_rs; then true; else
        echo "$component: missing 'missing_docs' forbid/deny"
    fi

    # Package Info
    pkg_info_rs="$component/src/pkg_info.rs"
    if [ ! -f "$pkg_info_rs" ]; then
        echo "$component: missing 'pkg_info.rs'"
    fi

done
