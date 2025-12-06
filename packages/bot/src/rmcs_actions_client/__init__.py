from __future__ import annotations

import asyncio

from rmcs_actions_client.client import RobotClient
from rmcs_actions_client.load_config import load_config
from rmcs_actions_client.log import init_logger


async def async_main() -> None:
    """Main entry point for the robot daemon."""
    # Load configuration
    config = load_config()

    # Initialize logger
    init_logger(config.logging.daemon.directory)

    # Create and run client using configured URLs
    client = RobotClient(config)
    await client.run()


def main() -> None:
    """Synchronous entry point."""
    asyncio.run(async_main())
