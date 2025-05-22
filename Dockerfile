FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache \
    openssl \
    ca-certificates

# Set the working directory
WORKDIR /app

# Copy the built binary from the host
COPY result/bin/pocbot /app/pocbot

# Run the pocbot executable
CMD ["./pocbot"]
