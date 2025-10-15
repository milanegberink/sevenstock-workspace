FROM rust:slim AS planner

WORKDIR /app

RUN cargo install cargo-chef

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM rust:slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    libssl-dev \
    ca-certificates \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef

COPY --from=planner /app/recipe.json .

RUN cargo chef cook --release --recipe-path recipe.json --bin web-server

COPY . .

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

COPY --from=builder /app/target/release/web-server .
COPY --from=builder /app/web-folder/apps/oauth-login/build /app/web-folder/apps/oauth-login/build

ENV RUST_LOG=debug

CMD ["./web-server"]
