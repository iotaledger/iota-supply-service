# IOTA Supply REST API

A simple application providing the circulating and total supply of IOTA tokens.
Please note that a running fullnode instance is required for syncing the data.

# Development

## Docker Setup

### 1. Build the Docker Image
To build the Docker image (forcing a fresh build without using cached layers), run:
```sh
docker compose build --no-cache
```

### 2. Run the Docker Container
To start the container in detached mode, run:
```sh
docker compose up -d
```

### 3. Stop the Docker Container
To stop the running container and remove the associated resources, use:
```sh
docker compose down
```

---

### Environment Configuration

The application uses a pre-existing `.env` file for configuration. You can modify it if needed.

#### Configuration
- **`LOG_LEVEL`**: Logging level (e.g., `INFO`, `DEBUG`).
- **`REST_API_SOCKET_ADDRESS`**: Address where the REST API will listen (e.g., `0.0.0.0:4000`).

### Notes
- Any changes to the `.env` file will take effect the next time the container is built or restarted.

### Run the application

```sh
$ cargo run
```

### Run the tests

```sh
$ cargo test
```
