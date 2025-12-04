from __future__ import annotations

from pathlib import Path

from pydantic import BaseModel


class Configuration(BaseModel):
    logging: LoggingConfig


class LoggingConfig(BaseModel):
    daemon: LogConfig
    server: LogConfig

class LogConfig(BaseModel):
    directory: Path
