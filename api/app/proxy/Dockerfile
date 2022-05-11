FROM asia-docker.pkg.dev/getto-projects/example/base:latest as builder
COPY . /build
WORKDIR /build
RUN : && \
  RUN_CODE_BUILDER=TRUE cargo build --bin proxy --release --target x86_64-unknown-linux-musl && \
  :

FROM gcr.io/distroless/static-debian10
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/proxy /app
CMD ["/app"]
