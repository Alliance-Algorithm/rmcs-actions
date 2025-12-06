"""
Thread-safe event loop utilities for running async coroutines from sync contexts.

This module provides a simple wrapper around asyncio for managing separate event loops
in threads, allowing thread-safe execution of coroutines.
"""

from __future__ import annotations

import asyncio
import threading

from collections.abc import Coroutine
from concurrent.futures import Future
from typing import Any
from typing import TypeVar


T = TypeVar("T")


class ThreadSafeEventLoop:
    """
    A thread-safe event loop wrapper that runs in a dedicated thread.
    
    This class manages an asyncio event loop in a separate thread and provides
    utilities to submit coroutines for execution from any thread.
    """

    def __init__(self, name: str = "EventLoop") -> None:
        """
        Initialize a new thread-safe event loop.
        
        Args:
            name: Name of the thread for debugging purposes
        """
        self._name = name
        self._loop: asyncio.AbstractEventLoop | None = None
        self._thread: threading.Thread | None = None
        self._started = threading.Event()
        self._stopped = threading.Event()

    def start(self) -> None:
        """Start the event loop in a new thread."""
        if self._thread is not None and self._thread.is_alive():
            return

        self._stopped.clear()
        self._thread = threading.Thread(target=self._run_loop, name=self._name, daemon=True)
        self._thread.start()
        self._started.wait()  # Wait for loop to be ready

    def _run_loop(self) -> None:
        """Internal method to run the event loop in the thread."""
        self._loop = asyncio.new_event_loop()
        asyncio.set_event_loop(self._loop)
        self._started.set()

        try:
            self._loop.run_forever()
        finally:
            try:
                self._cancel_all_tasks()
                self._loop.run_until_complete(self._loop.shutdown_asyncgens())
                self._loop.run_until_complete(self._loop.shutdown_default_executor())
            finally:
                asyncio.set_event_loop(None)
                self._loop.close()
                self._stopped.set()

    def _cancel_all_tasks(self) -> None:
        """Cancel all running tasks in the loop."""
        if self._loop is None:
            return

        tasks = [task for task in asyncio.all_tasks(self._loop) if not task.done()]
        if not tasks:
            return

        for task in tasks:
            task.cancel()

        self._loop.run_until_complete(asyncio.gather(*tasks, return_exceptions=True))

    def stop(self, timeout: float = 5.0) -> None:
        """
        Stop the event loop and wait for the thread to finish.
        
        Args:
            timeout: Maximum time to wait for the loop to stop (in seconds)
        """
        if self._loop is None or not self._thread or not self._thread.is_alive():
            return

        self._loop.call_soon_threadsafe(self._loop.stop)
        self._stopped.wait(timeout=timeout)

    def run_coroutine_threadsafe(self, coro: Coroutine[Any, Any, T]) -> Future[T]:
        """
        Submit a coroutine to the event loop for execution.
        
        This is similar to asyncio.run_coroutine_threadsafe but uses the managed loop.
        
        Args:
            coro: The coroutine to execute
            
        Returns:
            A Future that will contain the result of the coroutine
            
        Raises:
            RuntimeError: If the event loop is not running
        """
        if self._loop is None or not self._loop.is_running():
            raise RuntimeError(f"Event loop '{self._name}' is not running")

        return asyncio.run_coroutine_threadsafe(coro, self._loop)

    def submit(self, coro: Coroutine[Any, Any, T]) -> Future[T]:
        """
        Alias for run_coroutine_threadsafe.
        
        Args:
            coro: The coroutine to execute
            
        Returns:
            A Future that will contain the result of the coroutine
        """
        return self.run_coroutine_threadsafe(coro)

    @property
    def is_running(self) -> bool:
        """Check if the event loop is currently running."""
        return self._loop is not None and self._loop.is_running()

    @property
    def loop(self) -> asyncio.AbstractEventLoop | None:
        """Get the underlying event loop (use with caution)."""
        return self._loop

    def __enter__(self) -> ThreadSafeEventLoop:
        """Context manager entry."""
        self.start()
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Context manager exit."""
        self.stop()
