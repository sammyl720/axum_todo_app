# Axum Todo App

This is a simple Todo application built using Rust and the Axum framework. The application provides a basic web interface to manage a list of tasks.

## Features

- Add, edit, and delete tasks
- Mark tasks as completed
- Basic web interface with HTML, CSS, and JavaScript
- Docker support for easy deployment

## Getting Started

### Prerequisites

- Rust
- Docker (optional)

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/sammyl720/axum_todo_app.git
   cd axum_todo_app
   ```

2. Create a `.env` file with the following content:

   ```sh
   DATABASE_URL="sqlite:todos.db"
   PORT=3000
   ```

3. Build and run the application:

   ```sh
   cargo run
   ```

4. Open your web browser and navigate to `http://localhost:3000`.

### Using Docker

1. Build the Docker image:

   ```sh
   docker build -t axum_todo_app .
   ```

2. Run the Docker container:

   ```sh
   docker run -p 8080:8080 axum_todo_app
   ```

   By default, the Docker container exposes port 8080. You can override this by setting the `PORT` environment variable in the Dockerfile if needed.

## Environment Variables

The following environment variables should be specified or included in a local `.env` file when running the application locally (not in Docker):

- `DATABASE_URL`
- `PORT`

Example `.env` file:

```env
DATABASE_URL="sqlite:todos.db"
PORT=3000
```

## Project Structure

- `src/`: Contains the source code for the application
- `public/`: Contains static files (HTML, CSS, JS)
- `migrations/`: Database migration files
- `Dockerfile`: Docker configuration

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
