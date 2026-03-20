# RMCS Actions Bot

This package contains the RMCS robot daemon. The bot authenticates a robot over
HTTP, stores local robot state, and then maintains a long-lived WebSocket
connection to the RMCS Actions service.

## Configuration

Start from the example file:

```sh
cp config.example.yaml config.yaml
```

The bot reads `config.yaml` by default. Use `-config` to point at a different
file.

- `log.dir`: directory for rotating `bot.log` files. The directory is created
  automatically if it does not exist.
- `storage.dir`: directory for persistent local bot state such as the cached
  robot identifier. The directory is created automatically if it does not
  exist.
- `service.api`: HTTP API base URL of the RMCS Actions service. The example
  value points to `http://localhost:3000/api`.
- `service.websocket`: WebSocket base URL of the RMCS Actions service. The
  example value points to `ws://localhost:3000/ws`.

The example configuration uses `runtime/logs` and `runtime/storage` so local
state stays under the package directory by default.

## Running

For a local development run:

```sh
go run . -config config.yaml
```

For a compiled binary:

```sh
./bot -config config.yaml
```

## Version Reporting

The bot now supports:

```sh
./bot --version
```

Local builds print `dev` unless a version is injected at build time:

```sh
go build -ldflags="-X main.Version=v1.2.3" -o bot .
```

CI builds embed `ci-<sha>`. Tagged release builds embed the git tag, which
makes `--version` useful for checking the exact binary deployed on a robot.

## Release Artifacts

Tagged releases publish:

- `bot-linux-amd64`
- `bot-linux-arm64`
- `SHA256SUMS.txt`

Verify downloaded artifacts with:

```sh
sha256sum -c SHA256SUMS.txt
```

The release workflow still publishes the service binary separately.
