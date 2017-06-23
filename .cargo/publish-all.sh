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


# List of components, in order of dependency
COMPONENTS="
    ucd/core
    ucd/utils
    ucd/age
    ucd/bidi
    ucd/normal
    ucd
    bidi
    normal
    idna/punycode
    idna
"


-() {
    cmd="$@"
    echo
    echo "`tput bold; tput setaf 4`============`tput sgr0` "
    echo "`tput bold; tput setaf 4`   Executing`tput sgr0` $cmd"
    echo
    $cmd
    echo "`tput bold; tput setaf 4`   Succeeded`tput sgr0` $cmd"
    echo "`tput bold; tput setaf 4`------------`tput sgr0` "
    echo
}


# Steps

- cargo update --verbose

# First test all components and stop if anything goes wrong
for component in $COMPONENTS; do
    - cargo test    --verbose --manifest-path components/$component/Cargo.toml
done

# Then publish all components, and ignore failures (usually because of the version being released already)
for component in $COMPONENTS; do
    - cargo publish --verbose --manifest-path components/$component/Cargo.toml || true
done

- cargo publish --verbose
