# Use the official Rust image as a base
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app
COPY .env ./.env

# Install dependencies and build the project in release mode
RUN cargo build --release

# Expose port 9009 to the host
EXPOSE 9009

# Run the application and bind it to 0.0.0.0 to allow access from outside the container
CMD ["cargo", "run", "--release", "--", "--address", "0.0.0.0", "--port", "9009"]