FROM ekidd/rust-musl-builder:1.51.0 AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release

# final. application layer
FROM busybox:musl
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/{} ./
CMD ["./{}"]