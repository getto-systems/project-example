from rust:1-bullseye

copy . /apps
workdir /apps

RUN : && \
  apt-get update && \
  apt-get install -y \
    cmake \
    musl-tools \
    protobuf-compiler \
  && \
  rustup component add rustfmt && \
  rustup target add x86_64-unknown-linux-musl && \
  RUN_CODE_BUILDER=TRUE cargo build --bin proxy --release --target x86_64-unknown-linux-musl && \
  :
