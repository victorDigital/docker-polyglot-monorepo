# Multi-stage Dockerfile for Rust worker
FROM rust:1.92 AS development

WORKDIR /app

# Install cargo-binstall for faster binary installation
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo-watch using binstall (faster and more reliable)
RUN cargo binstall -y cargo-watch

# Copy manifest files
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {println!(\"placeholder\");}" > src/main.rs && \
    cargo build && \
    rm -rf src

# Copy source code
COPY . .

# Build in debug mode for development
RUN cargo build

# Expose port
EXPOSE 8080

# Run with cargo-watch for hot reloading
CMD ["cargo", "watch", "-x", "run"]

# Production stage
FROM rust:1.92 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS production

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api-rust /app/api-rust

EXPOSE 8080

CMD ["/app/api-rust"]
