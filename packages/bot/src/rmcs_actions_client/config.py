from __future__ import annotations

from pathlib import Path

from pydantic import BaseModel


class Configuration(BaseModel):
    logging: LoggingConfig
    storage: StorageConfig
    server: ServerConfig


class LoggingConfig(BaseModel):
    daemon: LogConfig
    server: LogConfig


class LogConfig(BaseModel):
    directory: Path


class StorageConfig(BaseModel):
    directory: Path


class ServerConfig(BaseModel):
    http: str
    websocket: str
