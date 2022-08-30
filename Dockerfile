FROM rust:1.63.0 as build

RUN USER=root cargo new --bin tagbot
WORKDIR /tagbot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build deps for cache

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Build for release

RUN rm ./target/release/deps/tagbot*
RUN cargo install --path .

# Final build

# TODO: Add luarocks and sandbox

FROM debian:buster-slim

WORKDIR /home

COPY --from=build /tagbot/target/release/tagbot ./tagbot

ENTRYPOINT './tagbot'
CMD []
