# Multi-stage Dockerfile for Zerodha CLI
# Builds a production-ready Rust binary in a clean environment

# Stage 1: Builder
FROM rust:1.82-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy workspace manifest
COPY Cargo.toml Cargo.lock ./

# Copy core library (dependencies first for layer caching)
COPY core/ ./core/

# Copy CLI binary
COPY cli/ ./cli/

# Build release binary (optimized)
RUN cargo build --release --workspace

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 -s /bin/bash zerodha

WORKDIR /home/zerodha

# Copy binary from builder stage
COPY --from=builder /build/target/release/kite /usr/local/bin/kite

# Set ownership
RUN chown -R zerodha:zerodha /home/zerodha /usr/local/bin/kite

# Switch to non-root user
USER zerodha

# Set default command
ENTRYPOINT ["/usr/local/bin/kite"]
CMD ["--help"]
