# Event Loop and Message Adaptor Guide

This project provides thread-safe event loops and a WebSocket message adaptor that follows the protocol described in `docs/protocol.md`.

## Components

1. **ThreadSafeEventLoop** (`lib/eventloop.py`)
  - Lightweight wrapper over `asyncio`
  - Runs in a dedicated thread
  - Exposes thread-safe coroutine submission

2. **MessageAdaptor** (`lib/adaptor.py`)
  - Parses the protocol envelope `{session_id, local_timestamp, payload}`
  - Routes `instruction`, `response`, and `event` messages to registered handlers
  - Can automatically send protocol-compliant responses

3. **Action Module** (`action/__init__.py`)
  - Manages the outbound loop for proactive events
  - Provides async `send_event` and sync `submit_event`

4. **Service Module** (`service/__init__.py`)
  - Manages the inbound loop for instructions
  - Register instruction handlers and bridge them to the adaptor

## Quick Start

```python
from rmcs_actions_client.action import get_action_loop, send_event
from rmcs_actions_client.service import get_service_loop, register_instruction, handle_instruction_message
from rmcs_actions_client.lib.adaptor import MessageAdaptor

get_action_loop()
get_service_loop()

async def fetch_network_handler(payload: dict) -> dict:
   return {"example_nic": {"nic": "eth0"}}

register_instruction("fetch_network", fetch_network_handler)

adaptor = MessageAdaptor()
adaptor.register_instruction_handler("fetch_network", handle_instruction_message)

# Later in a websocket context
# await send_event(ws, "heartbeat", {"ts": time.time()})
# await adaptor.start_listening(ws)
```

## Message Shapes (per protocol)

- Envelope: `{ "session_id": string, "local_timestamp": number, "payload": {...} }`
- Instruction payload: `{ "type": "instruction", "instruction": "fetch_network", "message": {...} }`
- Response payload: `{ "type": "response", "message": {...} }`
- Event payload: `{ "type": "event", "event": "heartbeat", "detail": {...} }`

## API Notes

- `ThreadSafeEventLoop.submit(coro)` runs a coroutine on the dedicated loop safely from any thread.
- `MessageAdaptor.start_listening(ws)` consumes websocket messages and dispatches them; it sends responses when instruction handlers return data.
- `send_event(ws, event, detail)` emits a protocol-compliant event; `submit_event` offers a sync-friendly wrapper.
- `register_instruction(name, handler)` registers handlers for incoming instructions; `handle_instruction_message` bridges adaptor messages to your registry.
