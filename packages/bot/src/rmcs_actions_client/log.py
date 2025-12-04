from __future__ import annotations

import logging

from pathlib import Path


def init_logger(path: Path):
    _logger = logging.getLogger("rmcs_actions_client")
    _logger.setLevel(logging.DEBUG)
    _console_handler = logging.StreamHandler()
    _console_handler.setLevel(logging.INFO)
    _formatter = logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
    _console_handler.setFormatter(_formatter)
    _logger.addHandler(_console_handler)
    path.mkdir(parents=True, exist_ok=True)
    _file_handler = logging.FileHandler(path / "rmcs_actions_client.log")
    _file_handler.setLevel(logging.DEBUG)
    _file_handler.setFormatter(_formatter)
    _logger.addHandler(_file_handler)


GL = logging.getLogger("rmcs_actions_client")
