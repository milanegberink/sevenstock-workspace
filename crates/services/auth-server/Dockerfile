FROM rust:slim AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y --no-install-recommends protobuf-compiler ca-certificates && rm -rf /var/lib/apt/lists/*

COPY . .

# RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release --bin auth-server

COPY crates/services/auth-server/src ./crates/services/auth-server/src

FROM debian:bookworm-slim

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/auth-server .

CMD ["./auth-server"]
