FROM rust:1.81.0-alpine AS builder

WORKDIR /usr/src/bnjfolio-dev
COPY . .

# Update system packages and install necessary dependencies.
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    gcc \
    curl

ARG LEPTOS_OUTPUT_NAME
ARG LEPTOS_SITE_ROOT
ARG LEPTOS_SITE_PKG_DIR
ARG LEPTOS_SITE_ADDR
ARG LEPTOS_RELOAD_PORT

ENV LEPTOS_OUTPUT_NAME=$LEPTOS_OUTPUT_NAME \
    LEPTOS_SITE_ROOT=$LEPTOS_SITE_ROOT \
    LEPTOS_SITE_PKG_DIR=$LEPTOS_SITE_PKG_DIR \
    LEPTOS_SITE_ADDR=$LEPTOS_SITE_ADDR \
    LEPTOS_RELOAD_PORT=$LEPTOS_RELOAD_PORT

RUN cp ./.env.example ./.env

RUN rustup target add wasm32-unknown-unknown

# Install cargo-binstall
RUN curl -LO https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-gnu.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-gnu.tgz \
    && cp cargo-binstall /usr/local/cargo/bin

# Install necessary dependencies.
RUN cargo binstall -y cargo-leptos

# Install rust nightly and wasm

# Build the binary.
RUN cargo leptos build --release

# Second stage building, to avoid bloated binary.
FROM alpine:latest

RUN apk add --no-cache libssl3 \
    ca-certificates \
    curl

WORKDIR /app

# Copy the binary from the builder stage.
COPY --from=builder /usr/src/bnjfolio-dev/target/release/bnjfolio .
COPY --from=builder /usr/src/bnjfolio-dev/site/ ./site
COPY --from=builder /usr/src/bnjfolio-dev/.env .

CMD ["./bnjfolio"]