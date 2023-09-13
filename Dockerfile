# Build stage
FROM rust:1.72.0-slim-bullseye as builder
WORKDIR /app
# Install lld and clang
RUN apt-get update && apt-get install lld clang -y
# Copy the source code
COPY . .
# Set sqlx offline
ENV SQLX_OFFLINE true
# Build the binary
RUN cargo build --release

# Run stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the binary from the build stage
COPY --from=builder /app/target/release/server zero2prod
# Copy the configuration file
COPY configuration.yaml configuration.yaml
# Run the binary
ENTRYPOINT [ "./zero2prod" ]