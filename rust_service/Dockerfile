# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the manifest files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() { println!(\"dummy\"); }" > src/main.rs

# Build dependencies
RUN cargo build --release
RUN rm -f target/release/deps/rust_service*

# Copy the actual source code
COPY ./src ./src

# Build the final release
RUN cargo build --release

# Stage 2: Use an Ubuntu-based image with newer GLIBC versions for runtime
FROM ubuntu:22.04

# Install necessary libraries (if any)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust_service /usr/local/bin/rust_service

# Expose the port that the service will listen on
EXPOSE 3030

# Command to run the application
CMD ["rust_service"]
