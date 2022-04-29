#!/bin/sh

setup_main() {
  local target
  target=$1
  if [ -z "$target" ]; then
    echo "usage: setup-test.sh <install-dir>"
    exit 1
  fi
  mkdir -p $target

  install_cargo_llvm_cov
}
install_cargo_llvm_cov() {
  local version
  version=$(
    curl --silent "https://api.github.com/repos/taiki-e/cargo-llvm-cov/releases/latest" | \
    grep '"tag_name":' | \
    sed -E 's/.*"v([^"]+)".*/\1/' \
  )

  curl -L -o $target/cargo-llvm-cov.tar.gz https://github.com/taiki-e/cargo-llvm-cov/releases/download/v${version}/cargo-llvm-cov-aarch64-unknown-linux-gnu.tar.gz
  cd $target
  tar xvzf cargo-llvm-cov.tar.gz
  rm cargo-llvm-cov.tar.gz
  cd -
}

setup_main "$@"
