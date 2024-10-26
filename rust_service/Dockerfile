# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/rust_service

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY src ./src

# Build the release version of the service
RUN cargo build --release

# Run the service
CMD ["./target/release/rust_service"]
