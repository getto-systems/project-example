#!/bin/sh

setup_main() {
    setup_toolchain
    setup_grcov
}
setup_toolchain() {
    rustup install nightly
    rustup component add --toolchain nightly llvm-tools-preview
    rustup component add --toolchain nightly rustfmt
    cargo install rustfilt
    mkdir tmp
}
setup_grcov() {
    local version
    version=$(
        curl --silent "https://api.github.com/repos/mozilla/grcov/releases/latest" |
            grep '"tag_name":' |
            sed -E 's/.*"v([^"]+)".*/\1/'
    )
    curl -sSL https://github.com/mozilla/grcov/releases/download/v${version}/grcov-linux-x86_64.tar.bz2 > tmp/grcov.tar.bz2
    tar -xvjf tmp/grcov.tar.bz2

    mv grcov ${CARGO_HOME}/bin
}

setup_main
