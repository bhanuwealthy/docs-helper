# Use a Rust base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Create a new lightweight image
FROM ubuntu:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled executable from the builder stage
COPY --from=builder /usr/src/app/target/release/cp_docs .

# Set the entry point
ENTRYPOINT ["./cp_docs"]
