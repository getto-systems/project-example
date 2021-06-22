#!/bin/sh

setup_grcov() {
    mkdir tmp

    local version
    version=$(
        curl --silent "https://api.github.com/repos/mozilla/grcov/releases/latest" |
            grep '"tag_name":' |
            sed -E 's/.*"v([^"]+)".*/\1/'
    )
    curl https://github.com/mozilla/grcov/releases/download/v${version}/grcov-linux-x86_64.tar.bz2 > tmp/grcov.tar.bz2
    tar -xjf tmp/grcov.tar.bz2

    ls -R tmp

    mv tmp/grcov/bin/grcov ${CARGO_HOME}/bin
}

setup_grcov
