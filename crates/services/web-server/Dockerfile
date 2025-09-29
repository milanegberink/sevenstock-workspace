FROM rust:slim AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    libssl-dev \
    ca-certificates \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY . .

# RUN mkdir src && echo "fn main() {}" > src/main.rs

ENV SQLX_OFFLINE=true
ENV DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/sevenstock

RUN cargo build --release --bin web-server

COPY crates/services/web-server/src ./crates/services/web-server/src

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/web-server .

ENV RUST_LOG=debug

CMD ["./web-server"]
