FROM rust:1.89-slim-bullseye as builder

RUN apt-get update && apt-get install -y libssl-dev pkg-config --no-install-recommends && rm -rf /var/lib/apt/lists/*

ENV ACCESS_TOKEN_SECRETS_DIR=.keys/access-secrets
ENV REFRESH_TOKEN_SECRETS_DIR=.keys/refresh-secrets
ENV MFA_TOKEN_SECRETS_DIR=.keys/mfa-secrets
ENV DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/sevenstock
ENV REDIS_URL=redis://127.0.0.1


WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY crates/ ./crates/

RUN cargo fetch

COPY . .

RUN cargo build --release --package web-server

FROM debian:12.11-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates --no-install-recommends && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/member_one /usr/local/bin/app

CMD ["app"]
