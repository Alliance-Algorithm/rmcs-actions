# Event Loop and Adaptor Architecture

This document describes how the thread-safe event loops, session context management, and the protocol-aware WebSocket adaptor work together in `rmcs_actions_client` to support asynchronous request-response interactions.

## Overview

- **ThreadSafeEventLoop (`lib/eventloop.py`)**: a thin wrapper around `asyncio` that runs a dedicated loop in its own daemon thread and exposes thread-safe coroutine submission.
- **MessageAdaptor (`lib/adaptor.py`)**: parses the protocol envelope (`session_id`, `local_timestamp`, `payload`) from `docs/protocol.md`, routes messages by `payload.type` (`instruction`, `response`, `event`), and can emit protocol-compliant responses.
- **SessionContext and SessionManager (`lib/session.py`)**: manages bidirectional, session-scoped communication channels allowing handlers to send and await messages within a single session, simulating WebSocket-like iteration.
- **Action module (`action/__init__.py`)**: owns an outbound loop for proactive protocol events; provides async `send_event` and sync-friendly `submit_event` helpers.
- **Service module (`service/__init__.py`)**: owns an inbound loop for protocol instructions; registers instruction handlers and bridges them to the adaptor via `handle_instruction_message`.

## Event Loop Design

- Each loop instance is isolated per thread (`ThreadSafeEventLoop`) to avoid cross-contamination between send and receive paths.
- Lifecycle:
  - `start()`: spins up a daemon thread, installs a new event loop, and runs it forever.
  - `submit(coro)`: forwards to `asyncio.run_coroutine_threadsafe` on the managed loop, returning a `Future` usable from any thread.
  - `stop()`: schedules loop stop, cancels pending tasks, and shuts down async generators and default executors.
- Context manager support (`with ThreadSafeEventLoop(...)`): ensures start/stop symmetry.

## Session Context Design

The `SessionContext` and `SessionManager` modules enable handlers to maintain async request-response interactions within a single protocol session:

- **SessionRecvQueue**: an asyncio-backed queue that handlers can iterate over (like `async for msg in queue`) to receive messages targeted to their session.
- **SessionContext**: represents the bidirectional context of one session; handlers receive this as a parameter and can:
  - `await ctx.send(instruction="...", payload={...})` to send outbound messages within the session
  - `await ctx.recv()` or `async for msg in ctx` to await incoming responses/events tied to the same `session_id`
  - Call `ctx.close()` when done
- **SessionManager**: holds a registry of active `SessionContext` objects and routes incoming responses and events to their respective sessions via `deliver_response` and `deliver_event`.

## Adaptor Message Flow with Sessions

- **Instruction routing**: when an instruction arrives, the adaptor:
  1. Creates or retrieves a `SessionContext` for that `session_id` via the manager
  2. Passes both the `Message` and the `SessionContext` to the instruction handler
  3. Automatically sends a response if the handler returns a payload
  4. Closes the session when the handler completes
- **Response routing**: incoming responses are delivered to the active session's recv queue, allowing handlers to await them
- **Event routing**: events targeted to a session are also delivered to its recv queue
- **Context manager support**: handlers can use `async with session_ctx` for automatic cleanup

## Action Module Integration

- Maintains a singleton outbound loop (`ActionLoop`).
- `send_event(ws, event, detail, session_id?)` builds an event envelope via the adaptor helper and sends it asynchronously on the outbound loop.
- `submit_event(...)` is a synchronous wrapper that submits `send_event` to the loop and waits for completion (with timeout) from non-async contexts.
- Use when pushing daemon-originated protocol events such as `heartbeat`.

## Service Module Integration

- Maintains a singleton inbound loop (`ServiceLoop`).
- `register_instruction(name, handler)`: stores async handlers with signature `async def handler(payload, session_ctx)` that accept the instruction payload and an optional `SessionContext`.
- `handle_instruction_message(message, session_ctx)`: adaptor-facing bridge that resolves the handler, executes it with the session context, and surfaces its return value back to the adaptor for optional response emission.
- `submit_instruction_task(...)`: run a handler from sync code via the inbound loop when needed.
- Handlers can use `session_ctx.recv()`, `session_ctx.send()`, or `async for msg in session_ctx` to interact bidirectionally within the session.

## Protocol Alignment (per `docs/protocol.md`)

- Envelope: `{ "session_id": string, "local_timestamp": number, "payload": {...} }`
- `instruction` payload: `{ "type": "instruction", "instruction": "<endpoint>", "message": { ... } }`
- `response` payload: `{ "type": "response", "message": { ... } }` (sent by the adaptor when instruction handlers return data)
- `event` payload: `{ "type": "event", "event": "<event>", "detail": { ... } }`
- The adaptor is agnostic to specific instruction/event names; users register handlers to map protocol endpoints (e.g., `sync_robot_id`, `fetch_network`, `heartbeat`).

## Usage Patterns

### Simple Request-Response
```python
async def fetch_network(payload: dict, session_ctx) -> dict:
    """One-shot handler; ignore session_ctx."""
    return {"eth0": {...}}

adaptor.register_instruction_handler("fetch_network", handle_instruction_message)
```

### Multi-Step Interactions
```python
async def interactive_handler(payload: dict, session_ctx) -> dict:
    """Handler that sends and awaits multiple messages within a session."""
    if session_ctx is None:
        return {"error": "session required"}
    
    # Send a sub-request
    await session_ctx.send(instruction="sub_request", payload={"key": "value"})
    
    # Await the response
    response = await session_ctx.recv()
    # response.type is "response", response.payload contains data
    
    return {"status": "success", "result": response.payload}
```

### Iteration over Session Messages
```python
async def streaming_handler(payload: dict, session_ctx) -> dict:
    """Iterate over multiple session messages."""
    if session_ctx is None:
        return {"error": "session required"}
    
    count = 0
    async for msg in session_ctx:  # Iterate like a WebSocket
        if msg.type == "response":
            count += 1
            if count >= 5:
                break
    
    return {"processed": count}
```

### Connection Flow
- Create a `MessageAdaptor`, register instruction/event handlers
- `await adaptor.start_listening(websocket)` inside an async context
- Call `send_event` (async) or `submit_event` (sync) with the outbound WebSocket to push protocol events
- `stop()` on the adaptor to end the listen loop, then `shutdown_action_loop()` / `shutdown_service_loop()` to tear down event loops

## Extensibility Notes

- Instruction handlers receive both a `Message` and an optional `SessionContext`, allowing opt-in to session-scoped interactions.
- Handlers are plain async callables; users can encapsulate business logic without modifying the adaptor.
- Additional message types can be added by extending `MessageType` and adding a dispatch branch; the parsing logic defaults unknown types to `event` with a warning.
- Sessions are automatically closed after instruction handlers complete; handlers may explicitly call `ctx.close()` for early exit.
- The adaptor keeps minimal state (handler registries, session manager, and the current WebSocket) to stay lightweight and testable.
