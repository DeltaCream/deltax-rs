# syntax=docker/dockerfile:1

# Declare APP_NAME and RUST_VERSION as build arguments and before any FROM instruction
ARG APP_NAME=deltax
ARG RUST_VERSION=1.85.0

###############
# Base Stage  #
###############
# Base stage for installing tools
FROM rust:${RUST_VERSION}-slim AS base

WORKDIR /app

# Install build dependencies and cargo-chef using APT
RUN apt-get update && apt-get install -y \
    clang \
    lld \
    build-essential \
    git \
    perl \
    make \
    libpq-dev \
    pkg-config \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install cargo-chef --locked

#################
# Planner Stage #
#################

# Planner stage for analyzing dependencies
FROM base AS planner
WORKDIR /app

# Copy manifests to analyze dependencies
COPY Cargo.toml Cargo.lock ./

# Generate the recipe file for cargo-chef
RUN cargo chef prepare --recipe-path recipe.json

#################
# Cacher Stage  #
#################
# Cacher stage for building and caching dependencies
FROM base AS cacher
WORKDIR /app

# Copy the generated recipe file
COPY --from=planner /app/recipe.json ./

# Build and cache dependencies
RUN cargo chef cook --recipe-path recipe.json

################
# Build Stage  #
################
# Build stage for compiling the application
FROM base AS build
WORKDIR /app

# Copy static assets (webpages and files)
COPY static ./static

# Copy source code and manifests
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Copy cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Compile the application (add --release for release build)
RUN cargo build --locked

#################
# Final Stage   #
#################
# Final stage for running the application
FROM debian:bookworm-slim AS final
# FROM alpine:3.18 AS final

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    libssl3 \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a non-privileged user (but don't switch just yet)
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/usr/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Copy the compiled binary from the build stage
ARG APP_NAME=deltax
COPY --from=build /app/target/debug/${APP_NAME} /bin/server
# Use the release build instead of debug
# COPY --from=build /app/target/release/${APP_NAME} /bin/server

# Ensure static files are in the final image
COPY --from=build /app/static ./static

# Grant permissions after copying binary (you need to do this as root *before* switching to the non-privileged user)
RUN chmod +x /bin/server

# Switch to the non-privileged user
USER appuser

# Expose the application port
EXPOSE 8080

# Set the environment to development
ENV RUST_LOG=debug

# Set the command to run the application
CMD ["/bin/server"]