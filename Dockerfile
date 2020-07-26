# Build stage
FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

# We need this for the alpine runtime second stage
#ENV RUSTFLAGS="-Clinker=musl-gcc"
#ENV TARGET="x86_64-unknown-linux-musl"
ENV TARGET="x86_64-unknown-linux-gnu"

WORKDIR /usr/src/rustynail

# Fetch the dependencies for layer caching
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release --target=${TARGET}
RUN rm -f target/${TARGET}/release/deps/rustynail*

# Build the app
COPY . .
RUN cargo build --release --target=${TARGET}
RUN cargo install --path .

# Runtime stage
#FROM alpine:latest
FROM debian:buster-slim

RUN apt-get update \
 && apt-get install -y sqlite3 libpq5 \
 && rm -rf /var/lib/apt/lists/*

# Run as an unprivileged user
RUN groupadd -g 1000 rustynail
RUN useradd -g 1000 -u 1000 -m -s /bin/sh -d /home/rustynail rustynail

WORKDIR /home/rustynail

COPY --from=cargo-build /usr/local/cargo/bin/rustynail /usr/local/bin/rustynail

USER rustynail

CMD rustynail
