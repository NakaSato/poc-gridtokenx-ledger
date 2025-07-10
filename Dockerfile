# Multi-stage Dockerfile for Thai Energy Trading System
# Stage 1: Build stage
FROM rust:1.82-bullseye AS builder

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    cmake \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY pallets/ ./pallets/

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Copy benches directory for benchmarks
COPY benches/ ./benches/

# Build dependencies (this will be cached)
RUN cargo build --release && rm -rf src

# Copy the actual source code
COPY src/ ./src/
COPY examples/ ./examples/
COPY benches/ ./benches/

# Build the actual application
RUN cargo build --release

# Stage 2: Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 app

# Create necessary directories
RUN mkdir -p /app/data /app/logs /app/config \
    && chown -R app:app /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/ledger-core /app/ledger-core
COPY --from=builder /app/target/release/api-server /app/api-server

# Copy configuration files
COPY docker/config/ /app/config/

# Make binaries executable
RUN chmod +x /app/ledger-core /app/api-server

# Switch to app user
USER app

# Set working directory
WORKDIR /app

# Expose ports
EXPOSE 8080 9944 30333

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["./ledger-core"]
