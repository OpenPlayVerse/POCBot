FROM rust:1.72.0-slim-bookworm

# Set the working directory
WORKDIR /app

# Copy code to the thingy
copy . .

# Install openss;
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Install Deps
RUN cargo install --path .

# Run the pocbot executable
CMD ["pocbot"]
