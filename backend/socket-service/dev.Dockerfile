# Define the base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/local/app

# Copy the application code to the container
COPY . .

RUN apt-get update && apt-get -y install cmake

# Install cargo-watch
RUN cargo install cargo-watch

# Build the application (Debug mode)
RUN cargo build -p socket-service

# Start the application and watch changes
CMD ["cargo", "watch", "-x", "run -p socket-service", "-w", "./socket-service/"]
