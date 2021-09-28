FROM rust:1-buster as builder
COPY . /build
WORKDIR /build
RUN : && \
  apt-get update && \
  apt-get install -y \
    musl-tools \
  && \
  :
RUN : && \
  rustup component add rustfmt && \
  rustup target add x86_64-unknown-linux-musl && \
  RUN_CODE_BUILDER=TRUE cargo build --bin example --release --target x86_64-unknown-linux-musl && \
  :

FROM gcr.io/distroless/static-debian10
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/example /app
CMD ["/app"]
