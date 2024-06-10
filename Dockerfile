# Use the official Rust image as the base image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin todo_app
WORKDIR /todo_app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Copy the migrations directory
COPY migrations ./migrations

# Copy the public directory if it exists
COPY public ./public

# Build the application
RUN cargo build --release

# Use the same base image for the final image to ensure compatibility
FROM rust:latest

# Install required dependencies
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Create a new user for running the application
RUN adduser --disabled-login --gecos '' appuser

# Copy the compiled application from the builder stage
COPY --from=builder /todo_app/target/release/todo_app /usr/local/bin/todo_app

# Copy the migrations directory
COPY --from=builder /todo_app/migrations /migrations

# Set the environment variables
ENV DATABASE_URL=sqlite::memory:
ENV PORT=8080

# Expose the port the app runs on
EXPOSE 8080

# Run the application as the appuser
USER appuser
CMD ["todo_app"]
