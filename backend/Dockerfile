# syntax=docker/dockerfile:1

# # Declare APP_NAME before any FROM instruction
# ARG APP_NAME=deltax

# # Base stage for installing tools
# ARG RUST_VERSION=1.85.0
# FROM rust:${RUST_VERSION}-alpine AS base
# WORKDIR /app

# # Set environment variables after the first FROM instruction
# # ENV APP_NAME=${APP_NAME}

# # Install build dependencies and cargo-chef
# RUN apk add --no-cache clang lld musl-dev git && \
#     cargo install cargo-chef --locked

# # Install Bacon
# # RUN cargo install bacon && \
# # ln -s /root/.cargo/bin/bacon /usr/local/bin/bacon
# # RUN cargo install bacon

# # Update PATH to include Cargo's bin directory
# # ENV PATH="/root/.cargo/bin:${PATH}"

# # Ensure the bacon binary has execute permissions
# # RUN chmod +x /root/.cargo/bin/bacon

# # Planner stage for analyzing dependencies
# FROM base AS planner
# WORKDIR /app

# # Copy manifests to analyze dependencies
# COPY Cargo.toml Cargo.lock ./
# COPY src ./src

# # Generate the recipe file for cargo-chef
# RUN cargo chef prepare --recipe-path recipe.json

# # Cacher stage for building and caching dependencies
# FROM base AS cacher
# WORKDIR /app

# # Copy the generated recipe file
# COPY --from=planner /app/recipe.json ./

# # Build and cache dependencies
# RUN cargo chef cook --release --recipe-path recipe.json

# # Build stage for compiling the application
# FROM base AS build
# WORKDIR /app

# # Copy static assets (webpages and files)
# COPY static ./static

# # Copy source code and manifests
# COPY Cargo.toml Cargo.lock ./
# COPY src ./src

# # Copy cached dependencies
# COPY --from=cacher /app/target target
# COPY --from=cacher /usr/local/cargo /usr/local/cargo

# # Compile the application
# RUN cargo build --release --locked

# # Final stage for running the application
# FROM alpine:3.18 AS final

# # Create a non-privileged user (but don't switch just yet)
# ARG UID=10001
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     appuser

# ARG APP_NAME=deltax
# # Copy the compiled binary from the build stage
# COPY --from=build /app/target/release/${APP_NAME} /bin/server

# # Ensure static files are in the final image
# COPY --from=build /app/static ./static

# # Grant permissions after copying binary (you need to do this as root *before* switching to the non-privileged user)
# # RUN chown appuser:appuser /bin/server
# RUN chmod +x /bin/server

# # Switch to the non-privileged user
# USER appuser

# # Expose the application port
# EXPOSE 8080

# # Set the command to run the application
# CMD ["/bin/server"]

######### border

# Declare APP_NAME before any FROM instruction
ARG APP_NAME=deltax

# Base stage for installing tools
ARG RUST_VERSION=1.85.0
FROM rust:${RUST_VERSION}-alpine AS base

# Set environment variables
# ENV PATH="/usr/local/cargo/bin:${PATH}"

WORKDIR /app

# Install build dependencies and cargo-chef
RUN apk add --no-cache clang lld musl-dev git && \
    cargo install cargo-chef --locked

# Planner stage for analyzing dependencies
FROM base AS planner
WORKDIR /app

# Copy manifests to analyze dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Generate the recipe file for cargo-chef
RUN cargo chef prepare --recipe-path recipe.json

# Cacher stage for building and caching dependencies
FROM base AS cacher
WORKDIR /app

# Copy the generated recipe file
COPY --from=planner /app/recipe.json ./

# Build and cache dependencies
RUN cargo chef cook --recipe-path recipe.json

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

# Final stage for running the application
FROM alpine:3.18 AS final

# Create a non-privileged user (but don't switch just yet)
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

ARG APP_NAME=deltax
# Copy the compiled binary from the build stage
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

# Set the command to run the application
CMD ["/bin/server"]