# Multi-stage Dockerfile for Lazabot
# Stage 1: Build Rust binary
FROM rust:1.90-slim as rust-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release

# Copy actual source code
COPY src/ src/

# Build the actual application
RUN cargo build --release

# Stage 2: Node.js runtime for Playwright
FROM node:22-slim as node-builder

# Install Playwright system dependencies
RUN apt-get update && apt-get install -y \
    libnss3 \
    libatk-bridge2.0-0 \
    libdrm2 \
    libxkbcommon0 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libgbm1 \
    libxss1 \
    libasound2 \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Stage 3: Final runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libnss3 \
    libatk-bridge2.0-0 \
    libdrm2 \
    libxkbcommon0 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libgbm1 \
    libxss1 \
    libasound2 \
    curl \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Create lazabot user
RUN useradd -r -s /bin/bash -d /opt/lazabot -m lazabot

# Set working directory
WORKDIR /opt/lazabot

# Copy Rust binary from builder
COPY --from=rust-builder /app/target/release/lazabot /opt/lazabot/bin/

# Copy Node.js runtime and dependencies
COPY --from=node-builder /usr/local/bin/node /usr/local/bin/
COPY --from=node-builder /usr/local/lib/node_modules /usr/local/lib/node_modules
COPY --from=node-builder /app/node_modules /opt/lazabot/node_modules

# Copy application files
COPY scripts/ /opt/lazabot/scripts/
COPY config/ /opt/lazabot/config/

# Create necessary directories
RUN mkdir -p /opt/lazabot/{data,logs} && \
    chown -R lazabot:lazabot /opt/lazabot

# Set permissions
RUN chmod +x /opt/lazabot/bin/lazabot

# Expose port
EXPOSE 8081

# Switch to lazabot user
USER lazabot

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/health || exit 1

# Default command (can be overridden)
CMD ["/opt/lazabot/bin/lazabot"]
