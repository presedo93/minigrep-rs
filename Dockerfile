FROM rust:latest as builder

WORKDIR /minigrep

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /minigrep

COPY --from=builder /minigrep/target/x86_64-unknown-linux-musl/release/minigrep ./