# RMCS Actions Bot Daemon

This package offers a daemon running inside
a Docker instance containing the RMCS framework.

For compatible usage inside and outside the container,
this project contains both `requirements.txt` and a `pyproject.toml`(`uv` dialect).

It's generally not recommended to use `uv` or other environment manager
inside your RMCS runtime though it's possible.

## Permissions

If you want to enable the network post,
you must grant host network permissions to the container.

**Warning**: This may have security implications.

- If use `docker run`: add the flag `--net=host`.
- If use `docker-compose`: add the line `network_mode: "host"` to the service definition.

## Configurations

The [`client.config.yaml`](./client.config.yaml) stores some configuration for the 
client. You must provide the path by either sticking to the current directory structure
or providing a env-var `CONFIG_PATH` pointing to the file.
