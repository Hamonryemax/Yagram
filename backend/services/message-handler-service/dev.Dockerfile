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
RUN cargo build -p message-handler-service

# Start the application and watch changes
CMD ["cargo", "watch", "-x", "run -p message-handler-service", "-w", "./message-handler-service/"]

