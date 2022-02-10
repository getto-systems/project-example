#!/bin/sh

setup_main() {
    setup_toolchain
}
setup_toolchain() {
    rustup install nightly
    rustup component add --toolchain nightly llvm-tools-preview
    rustup component add --toolchain nightly rustfmt
    cargo install rustfilt
    mkdir tmp
}

setup_main
