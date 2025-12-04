from __future__ import annotations

import os

from pathlib import Path

import yaml

from dotenv import load_dotenv

from rmcs_actions_client.config import Configuration


load_dotenv()


def load_config() -> Configuration:
    config_path = _load_config()
    with open(config_path, "r", encoding="utf-8") as f:
        cfg = yaml.safe_load(f)
    
    validated_cfg =  Configuration(**cfg)
    return validated_cfg

def _load_config() -> Path:
    possible_path = Path(__file__).parent.parent.parent / "client.config.yaml"
    if possible_path.exists():
        return possible_path
    if os.environ.get("CONFIG_PATH"):
        path = Path(os.environ["CONFIG_PATH"])
        if path.exists():
            return path
    raise FileNotFoundError("Configuration file not found.")
