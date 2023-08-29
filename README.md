
# My Rust Project

## Description

This is a Rust project featuring an HTTP API and WebSocket server using Tokio, Warp, and other popular Rust libraries.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/owenkellogg/platform-explorer.git
    ```

2. Navigate to the project directory:

    ```bash
    cd platform-explorer/packages/server
    ```

3. Install the dependencies:

    ```bash
    cargo build
    ```

## Configuration

This project uses environment variables for configuration. Create a `.env` file in the root directory of the project and populate it with the necessary variables:

- HTTP_PORT: Port on which the HTTP server will run (e.g., 3030).
- WS_PORT: Port on which the WebSocket server will run (e.g., 8080).

Example `.env` file:

```
HTTP_PORT=3030
WS_PORT=8080
```

## Running the Application

To run the application:

```bash
cargo run
```

## Testing

To run tests:

```bash
cargo test
```

## License

MIT License. See `LICENSE` for more details.