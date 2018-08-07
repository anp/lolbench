#!/usr/bin/env bash

root_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

run() {
    cd "$root_dir/$1" && cargo test
}

set -xe

run extractor
run support
run marky_mark
run
