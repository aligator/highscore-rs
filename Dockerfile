FROM rust:nightly-bookworm as builder

WORKDIR /app

COPY . /app

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp /app/target/release/highscore-rs /app

FROM debian:bookworm-slim

ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apt update && apt upgrade --yes && apt install --yes libsqlite3-0 && rm -rf /var/lib/apt/lists/*

ENV DATABASE_URL=./highscore.sqlite
ENV ROCKET_ADDRESS=0.0.0.0

RUN groupadd --gid ${GROUP_ID} highscore && \
    useradd --uid ${USER_ID} --gid highscore --shell /bin/bash --create-home highscore && \
    mkdir /home/highscore/data && \
    chown -R highscore:highscore /home/highscore/data

USER $USER_ID:$GROUP_ID

COPY --from=builder /app/highscore-rs /usr/local/bin

EXPOSE 8000:8000
WORKDIR /home/highscore/data
VOLUME ["/home/highscore/data"]


ENTRYPOINT ["highscore-rs"]