# ---- Build Stage ----
FROM rust:1.82-slim as builder
WORKDIR /app

# Install required packages for building
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential

# Copy source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install only necessary runtime packages
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /usr/local/bin

# Copy the compiled binary
COPY --from=builder /app/target/release/skdresses-backend .

# Copy the .env file
COPY .env .

# ✅ Create logs directory inside the container
RUN mkdir -p /usr/local/bin/logs

# Make the binary executable
RUN chmod +x skdresses-backend

# Start the app
CMD ["./skdresses-backend"]

#docker run -p 8080:8080 --env-file .env skdresses-backend