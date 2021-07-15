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
  rustup target add x86_64-unknown-linux-musl && \
  cargo build --release --target x86_64-unknown-linux-musl && \
  :

FROM gcr.io/distroless/static-debian10
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/example-api /app
CMD ["/app"]
