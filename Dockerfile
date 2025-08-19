# ---- Builder Stage ----
FROM rustlang/rust:nightly as builder

# Set working directory
WORKDIR /usr/src/app

# Copy Cargo files first (cache dependencies)
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build in release mode
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /usr/src/app/target/release/resume_back /app/

# Expose the port (Koyeb sets PORT=8080)
ENV PORT=8080
EXPOSE 8080

# Start the application
CMD ["./resume_back"]
