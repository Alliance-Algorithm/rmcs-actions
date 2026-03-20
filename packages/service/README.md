# RMCS Action Server

This package contains the RMCS Actions backend service. It exposes the HTTP API,
serves Swagger UI, accepts robot WebSocket connections, and stores service data
in SQLite.

The package is written in Rust.

## Environment

The service reads configuration from `.env` through `dotenvy`. Start from the
example file:

```sh
cp .env.example .env
```

Supported variables:

- `BIND_ADDR`: optional listener address for the HTTP and WebSocket server.
  Defaults to `0.0.0.0:3000`.
- `DATABASE_URL`: SQLite connection string used for startup and runtime access.
- `LOG_DIR`: required directory for service log files. The service creates it if
  needed and writes rotating JSON logs to `LOG_DIR/service.log`.
- `STORAGE_DIR`: required directory for service-managed storage. The service
  creates it if needed.

The service logs to both stdout and `LOG_DIR/service.log`. File logs rotate at
10 MB and keep up to five compressed archives. The default root log level is
`Info`.

## Running

On NixOS, the package provides its own development shell:

```sh
nix develop ./packages/service
```

Run the service with:

```sh
cargo run
```

The service starts on `BIND_ADDR` and exposes:

- `/api`: OpenAPI-backed HTTP API
- `/swagger`: Swagger UI
- `/ws/:robot_uuid`: robot WebSocket endpoint

The published OpenAPI server URL is the relative path `/api`, so Swagger UI and
generated clients resolve the host and scheme from the incoming request instead
of the internal bind address.

## Database Behavior

At startup the service ensures the `robots` and `network_info` tables exist.

`network_info.robot_uuid` now has a foreign key to `robots.uuid`, and SQLite
foreign key enforcement is enabled on every pooled connection. Deleting a robot
therefore deletes the related `network_info` row automatically.

If an existing database has an older `network_info` table without that foreign
key, the service performs an in-place migration during startup by recreating the
table and copying the rows into the new schema.

## API Behavior

Malformed request payloads now return HTTP `400 Bad Request` instead of falling
through to an internal error response.
