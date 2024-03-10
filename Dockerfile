# Start from the official Rust image
FROM rust:latest as builder

# Install musl-tools
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

# Target the musl architecture for a fully static binary
RUN rustup target add x86_64-unknown-linux-musl

# Set the working directory
WORKDIR /usr/src/app

# Add labels for OCI annotations
LABEL org.opencontainers.image.source="https://github.com/storopoli/stoic-quotes" \
    org.opencontainers.image.description="Stoic Quotes" \
    org.opencontainers.image.licenses="MIT"

# Copy project's Cargo.toml file
COPY ./Cargo.toml ./

# This dummy build is to cache dependencies so they don't need to be rebuilt
# every time your source changes
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -f target/x86_64-unknown-linux-musl/release/deps/stoic*

# Copy project's source code and other relevant folders to the Docker image
COPY ./src ./src
COPY ./assets ./assets
COPY ./data ./data
COPY ./templates ./templates

# Build application for release target musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Start a new stage from a slim version of Debian to reduce the size of the final image
FROM debian:buster-slim

WORKDIR /usr/src/app

# Copy the binary from the builder stage to the new stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/stoic-quotes /usr/local/bin/stoic-quotes
# Copy the assets/ from the builder stage to the new stage
COPY --from=builder /usr/src/app/assets /usr/src/app/assets

# Expose port 3000
EXPOSE 3000

# Command to run the binary
CMD ["stoic-quotes"]

