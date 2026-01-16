FROM rust:1.92-slim AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build

COPY Cargo.toml Cargo.lock ./

COPY src src/

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/file-archive-utils /usr/local/bin/validate

WORKDIR /assets

CMD ["/usr/local/bin/validate"]