"""Task wrapper for daemon tasks.

This module provides a wrapper for executing daemon tasks in the RMCS Actions Client.
The wrapper will launch a coroutine task each time it is called,
to avoid influencing the main daemon process.

The main daemon process is running on a synchronous event loop,
so the wrapper ensures that daemon tasks are executed asynchronously
without blocking the main loop.
"""

from __future__ import annotations

import asyncio
import inspect
import threading

from rmcs_actions_client.daemon.loop import TASK_LOOP
from rmcs_actions_client.log import GL


def make_task(function, *args, **kwargs) -> None:
    """Schedule a daemon task (sync or async) without blocking the caller.

    - If `function` is an async function, schedule it on the background
      asyncio loop using `run_coroutine_threadsafe`.
    - If `function` is a regular function, execute it in a background thread.
    """
    try:
        if inspect.iscoroutinefunction(function):
            future = asyncio.run_coroutine_threadsafe(function(*args, **kwargs), TASK_LOOP)

            def _done_callback(fut):
                try:
                    fut.result()
                except Exception:
                    GL.exception("Async task %s failed", getattr(function, "__name__", str(function)))

            future.add_done_callback(_done_callback)
        else:
            def _runner():
                try:
                    function(*args, **kwargs)
                except Exception:
                    GL.exception("Task %s failed", getattr(function, "__name__", str(function)))

            threading.Thread(target=_runner, daemon=True).start()
    except Exception:
        GL.exception("Failed to schedule task %s", getattr(function, "__name__", str(function)))
