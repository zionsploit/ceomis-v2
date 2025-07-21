FROM rust:alpine3.20 AS builder

# Install build dependencies for Alpine
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Set working directory
WORKDIR /app

# Copy all 
COPY . .

# Build the entire workspace
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk update && apk add --no-cache \
    ca-certificates-bundle \
    libssl3

# Create app user
RUN adduser -D -u 1001 appuser

# Set working directory
WORKDIR /app

# Copy binaries from workspace
COPY --from=builder /app/target/release/ /app/bin/

# Copy any migration files if needed
COPY --from=builder /app/migration /app/migration

# Copy env file
COPY --from=builder /app/.env /app/.env

# Create directories for potential runtime needs
RUN mkdir -p /app/data /app/logs

# Change ownership to app user
RUN chown -R appuser:appuser /app

# Switch to app user
USER appuser

# Expose common ports (adjust as needed)
EXPOSE 3001

# Default command - adjust based on your main binary
CMD ["./bin/server"]

# Alternative commands:
# CMD ["./bin/files-api"]
# CMD ["sh", "-c", "./bin/migration && ./bin/server"]
# CMD ["tail", "-f", "/dev/null"]