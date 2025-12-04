from __future__ import annotations

import time

import schedule

from dotenv import load_dotenv

import rmcs_actions_client.daemon.tasks as tasks

from rmcs_actions_client.daemon.task_wrapper import make_task
from rmcs_actions_client.load_config import load_config
from rmcs_actions_client.log import GL
from rmcs_actions_client.log import init_logger


load_dotenv()


def main() -> None:
    config = load_config()
    init_logger(config.logging.daemon.directory)

    GL.info("Starting RMCS Actions Client Daemon...")

    schedule.every(10).seconds.do(lambda: GL.debug("Daemon is running..."))
    schedule.every(2).seconds.do(lambda: make_task(tasks.heartbeat_task))

    while True:
        schedule.run_pending()
        time.sleep(0.5)
