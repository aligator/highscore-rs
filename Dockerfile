FROM rust:1.72-bookworm as builder

WORKDIR /app

COPY . /app

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp /app/target/release/highscore-rs /app

FROM debian:bookworm-slim

RUN apt update && apt upgrade --yes && apt install --yes libsqlite3-0 && rm -rf /var/lib/apt/lists/*

ENV HIGHSCORE_DATABASE_URL=/data/highscore.sqlite
ENV ROCKET_ADDRESS=0.0.0.0

RUN mkdir /data

COPY --from=builder /app/highscore-rs /

EXPOSE 8000:8000
VOLUME ["/data/highscore.sqlite"]

ENTRYPOINT ["/highscore-rs"]