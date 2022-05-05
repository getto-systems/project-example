from rust:1-bullseye

copy . /apps
workdir /apps

run apt-get update && apt-get install -y cmake && \
    rustup component add rustfmt && \
    rustup component add llvm-tools-preview && \
    ./api/ci/setup-test.sh /usr/local/bin

run RUN_CODE_BUILDER=TRUE cargo llvm-cov
