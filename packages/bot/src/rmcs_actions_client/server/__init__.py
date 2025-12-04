from __future__ import annotations

from dotenv import load_dotenv

from rmcs_actions_client.load_config import load_config
from rmcs_actions_client.log import init_logger


load_dotenv()

def main() -> None:
    config = load_config()
    init_logger(config.logging.server.directory)
