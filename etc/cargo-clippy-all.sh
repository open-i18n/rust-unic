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

# Since `cargo clippy --workspace` does not exist yet, we use this dumb
# alternative solution for now.
#
# Main downside of this approch is that there are separate `target/`
# directories used for each component, increasing the test and publish process
# time.

set -e


DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT="$DIR/.."
. "$DIR/common.sh"


- cargo clean --verbose

for crate in $COMPONENTS; do
    - cd "$ROOT/$crate"
    - cargo clippy
    - cd "$ROOT"
done

for crate in $APPS; do
    - cd "$ROOT/$crate"
    - cargo clippy
    - cd "$ROOT"
done
