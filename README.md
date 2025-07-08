# Toodles - Simple TODO API

```
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠠⠒⠲⠤⠄⠩⠭⠭⠀⠀⠐⠒⠒⠀⠈⠉⠉⢆⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠠⠁⠀⠀⠠⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀
⠀⠀⠀⠠⠁⠀⠀⠠⠁⠀⠀⠀⠀⣀⠤⠴⣤⣤⣀⠀⠀⠀⠀⠀⢢⠀⠀⠀
⠀⠀⠠⠁⠀⠀⠠⠁⠀⠀⠀⡔⠈⠀⠀⠀⠱⠩⢫⢎⢆⠀⠀⠀⠀⠡⠀⠀
⠀⠠⠁⠀⠀⠠⠁⠀⠀⢀⠎⠀⠀⠀⠀⠀⠀⡇⡇⡆⡆⠆⠀⠀⠀⠀⠱⡀
⢠⠁⠀⠀⠠⠁⠀⠀⠀⡌⠀⠀⠀⠀⠀⠀⠀⠃⡇⡇⠀⢸⠀⠀⠀⠀⠀⡱
⠘⠅⠒⠠⠁⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⡜⡰⢠⠁⠇⡜⠀⠀⠀⠀⡰⠁
⠀⠈⢆⠀⠡⡀⠀⠀⠀⠱⠀⠀⠀⠀⡠⠊⠔⡡⢂⠌⡰⠁⠀⠀⠀⡰⠁⠀
⠀⠀⠈⢂⠀⠱⡀⠀⠀⠀⠑⢄⡐⠬⡐⠥⣊⠔⡡⠈⠀⠀⠀⠀⡐⠀⠀⠀
⠀⠀⠀⠀⢢⠀⠐⡄⠀⠀⠀⠀⠉⠛⠚⠛⠂⠁⠀⠀⠀⠀⠀⡐⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠡⡀⠈⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠈⠐⠬⣆⣀⣀⡀⠀⠀⠤⠠⠤⠀⠀⠐⠒⠀⠀⠀⠀⠀⠀
```

A simple, lightweight TODO API written in Rust using Actix-web and SQLite for task management.

## Features

- **RESTful API**: Clean HTTP endpoints for CRUD operations
- **SQLite persistence**: Lightweight database storage with automatic setup
- **Async performance**: Built on Actix-web for high-performance async operations
- **Simple deployment**: Single binary with embedded database
- **Easy testing**: Included shell scripts for quick API testing

## Installation

### Prerequisites
- Rust and Cargo installed ([Install Rust](https://rustup.rs/))

### Build from source
```bash
git clone <repository-url>
cd toodles
cargo build --release
```

## Usage

### Basic Commands

```bash
# Build the project
cargo build

# Quick syntax check
cargo check

# Run the server (starts on localhost:8080)
cargo run
```

### API Endpoints

Once the server is running, you can interact with these endpoints:

- `GET /` - Get all todos
- `POST /new` - Create a new todo (send title in request body)
- `DELETE /del/{id}` - Delete a todo by ID

### Testing with Included Scripts

```bash
# Get all todos
./get.sh

# Create a new todo
./post.sh "Buy groceries"

# Delete a todo (replace 1 with actual ID)
./del.sh 1
```

### Manual API Testing

```bash
# Get all todos
curl -i http://127.0.0.1:8080

# Create a new todo
curl -i -X POST http://127.0.0.1:8080/new -d "Learn Rust"

# Delete a todo
curl -i -X DELETE http://127.0.0.1:8080/del/1
```

## How It Works

The application uses a simple architecture:

1. **Main Server**: Actix-web server handling HTTP requests on port 8080
2. **Data Layer**: SQLite database with automatic table creation
3. **API Endpoints**: RESTful routes for todo operations
4. **Persistence**: Local `data.db` file for storing todos

## Database Schema

```sql
CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL
);
```

## Development

### Code Structure

- `src/main.rs`: HTTP server and route handlers
- `src/dao.rs`: Data Access Object for SQLite operations
- `*.sh`: Test scripts for API endpoints

### Development Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests (when available)
cargo test

# Run in development mode
cargo run
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Disclaimer

This software is provided for educational and development purposes. Use responsibly and ensure proper security measures in production environments.
