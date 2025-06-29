# Step 1: Use Go base image
FROM golang:1.20-alpine AS builder

WORKDIR /app

# Step 2: Copy source code
COPY . .

# Step 3: Generate go.mod inside container (if not already present)
RUN [ ! -f go.mod ] && go mod init skdresses-frontend || true

ENV GOPROXY=https://proxy.golang.org,direct
# Ensure go.mod and go.sum are up to date

#RUN echo "module skdresses-frontend\ngo 1.20" > go.mod

# Step 4: Download dependencies
RUN go mod tidy

# Step 5: Build the Go app
RUN go build -o frontend

# Step 6: Final slim image
FROM alpine:latest

WORKDIR /root/

# Copy from build stage
COPY --from=builder /app/frontend .
COPY templates/ templates/
COPY static/ static/

EXPOSE 8080

# Set entrypoint
ENTRYPOINT ["./frontend"]
