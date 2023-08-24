# Use the official Alpine Linux image as the base image
FROM debian:unstable-slim

RUN apt-get update && apt-get install -y libssl3

# Set the working directory
WORKDIR /app

COPY target/release/pocbot /app/pocbot

# Debuggign
RUN ls -la /app

# Set permissions for the executable
RUN chmod +x ./pocbot

# Run the pocbot executable
CMD ["./pocbot"]
