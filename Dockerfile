FROM rust:1.81.0-alpine

# Set the working directory
WORKDIR /app

# Install dependencies
RUN apk update && apk add --no-cache \
    openssl-dev \
    pkgconfig \
    musl-dev \
    build-base

# Copy code to the working directory
COPY . .

# Build the project
RUN cargo build --release

# Run the pocbot executable
CMD ["./target/release/pocbot"]