# Use the official Rust image as the base image
FROM rust:latest as builder

# Create a new directory for the application
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new directory for the source code
RUN mkdir src && echo "fn main() { println!(\"dummy\"); }" > src/main.rs

# Build the dependencies
RUN cargo build --release
RUN rm -f target/release/deps/rust_service*

# Copy the actual source code
COPY ./src ./src

# Build the final release
RUN cargo build --release

# Use a smaller image for the final build
FROM debian:buster-slim

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/rust_service /usr/local/bin/rust_service

# Expose the port on which the service will run
EXPOSE 3030

# Command to run the application
CMD ["rust_service"]
