# Start from the official Rust image
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy project's Cargo.toml file
COPY ./Cargo.toml ./

# This dummy build is to cache dependencies so they don't need to be rebuilt
# every time your source changes
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/stoic_quotes*

# Copy project's source code and other relevant folders to the Docker image
COPY ./src ./src
COPY ./assets ./assets
COPY ./data ./data
COPY ./templates ./templates

# Build application for release
RUN cargo build --release

# Start a new stage from a slim version of Debian to reduce the size of the final image
FROM debian:buster-slim

# Copy the binary from the builder stage to the new stage
COPY --from=builder /usr/src/app/target/release/stoic_quotes /usr/local/bin/stoic_quotes

# Expose port 3000
EXPOSE 3000

# Command to run the binary
CMD ["stoic_quotes"]

