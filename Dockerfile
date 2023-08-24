# Use the official Alpine Linux image as the base image
FROM alpine:latest

# Install required packages
RUN apk update && apk add --no-cache \
  curl \
  unzip

# Set the working directory
WORKDIR /app

# Fetch the Build Files.zip
RUN curl -L -o Build_Files.zip https://nightly.link/OpenPlayVerse/POCBot/workflows/rust/rust-rewrite/Build%20Files.zip

# Unzip the files
RUN unzip Build_Files.zip

# Set permissions for the executable
RUN chmod +x pocbot

# Run the pocbot executable
CMD ["./pocbot"]
