from __future__ import annotations

import asyncio
import threading


TASK_LOOP = asyncio.new_event_loop()


def _run_loop() -> None:
	"""Run the task event loop in a dedicated background thread."""
	asyncio.set_event_loop(TASK_LOOP)
	TASK_LOOP.run_forever()


# Start the loop once, in the background, for scheduling tasks thread-safely.
LOOP_THREAD = threading.Thread(
	target=_run_loop,
	name="rmcs-actions-task-loop",
	daemon=True,
)
LOOP_THREAD.start()