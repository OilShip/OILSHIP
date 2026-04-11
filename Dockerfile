FROM rust:1.78-slim-bookworm AS builder
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY rust-toolchain.toml ./
COPY Cargo.toml ./
COPY Cargo.lock* ./
COPY programs/oilship/Cargo.toml programs/oilship/
COPY watch/Cargo.toml watch/

RUN mkdir -p programs/oilship/src watch/src && \
    echo "fn main() {}" > watch/src/main.rs && \
    echo "" > programs/oilship/src/lib.rs && \
    cargo build --release -p oilship-watch || true

COPY programs programs
COPY watch watch

RUN cargo build --release -p oilship-watch

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -u 1001 -m oilship

COPY --from=builder /app/target/release/oilship-watch /usr/local/bin/oilship-watch

USER oilship
WORKDIR /home/oilship

ENTRYPOINT ["oilship-watch"]
