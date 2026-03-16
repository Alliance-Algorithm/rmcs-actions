# Live-Update Feature

Remote binary update mechanism for edge bot devices. The service sends an update instruction with an artifact URL to a connected bot over WebSocket. The bot downloads, validates, and atomically replaces its own executable, then restarts in-place.

## End-to-End Flow

```
Frontend                  Service                    Bot
   │                         │                        │
   │  POST /action/          │                        │
   │  update_binary          │                        │
   │────────────────────────>│                        │
   │                         │  WS: update_binary     │
   │                         │  instruction           │
   │                         │───────────────────────>│
   │                         │                        │ 1. Resolve exec path
   │                         │                        │ 2. Download binary
   │                         │                        │ 3. Validate ELF magic
   │                         │                        │ 4. chmod 0755
   │                         │                        │ 5. Atomic rename
   │                         │                        │
   │                         │  WS: response          │
   │                         │<───────────────────────│
   │  200 OK                 │                        │
   │<────────────────────────│                        │
   │                         │                        │ 6. Wait for WS flush
   │                         │                        │ 7. syscall.Exec
   │                         │                        │    (process restarts)
   │                         │                        │
   │                         │  WS: reconnect         │
   │                         │<───────────────────────│
```

## Wire Protocol

### Instruction (Service → Bot)

```json
{
  "instruction": "update_binary",
  "message": {
    "artifact_url": "https://artifacts.example.com/bot/v1.2.3/bot-linux-amd64"
  }
}
```

### Response (Bot → Service)

**Success:**
```json
{
  "status": "post_update",
  "message": "success, restarting..."
}
```

**Error:**
```json
{
  "status": "error",
  "message": "downloaded file is not a valid ELF binary"
}
```

## REST API Endpoints

### `POST /action/update_binary`

Update a single bot's binary.

**Request:**
```json
{
  "robot_id": "550e8400-e29b-41d4-a716-446655440000",
  "artifact_url": "https://artifacts.example.com/bot/v1.2.3/bot-linux-amd64"
}
```

**Response:**
```json
{
  "status": "post_update",
  "message": "success, restarting..."
}
```

### `POST /action/update_binary_all`

Update all connected bots. Returns per-robot results with individual status and message fields.

**Request:**
```json
{
  "artifact_url": "https://artifacts.example.com/bot/v1.2.3/bot-linux-amd64"
}
```

**Response:**
```json
{
  "status": "ok",
  "results": [
    {
      "robot_id": "550e8400-e29b-41d4-a716-446655440000",
      "status": "post_update",
      "message": "success, restarting..."
    },
    {
      "robot_id": "660e8400-e29b-41d4-a716-446655440001",
      "status": "error",
      "message": "downloaded file is not a valid ELF binary"
    }
  ]
}
```

The overall `status` is `"ok"` when every bot succeeds and `"partial_failure"` when any bot reports an error or returns an unrecognised response.

## Safety Guarantees

| Mechanism | Purpose |
|---|---|
| **Same-directory temp file** | Ensures temp file and target are on the same filesystem, which is required for `os.Rename` to be atomic |
| **ELF magic validation** | Checks first 4 bytes (`\x7fELF`) to prevent replacing the binary with an HTML error page or other invalid content |
| **Atomic rename** | `os.Rename` on the same filesystem is an atomic operation at the VFS level — the old binary is fully replaced in a single syscall |
| **Temp file cleanup** | On any error path, the temp file is removed before returning |

## Restart Semantics

The bot uses `syscall.Exec(execPath, os.Args, os.Environ())` to restart:

- **In-place replacement**: The current process image is replaced with the new binary. The PID remains the same.
- **Write-completion gated**: A goroutine waits for the send-done signal from the eventloop (indicating the WebSocket response has been flushed) before calling `syscall.Exec`, instead of relying on a fixed delay.
- **Re-initialization**: The new binary runs `main()` from scratch, re-authenticates with the service, and re-establishes the WebSocket connection via the existing retry loop.
- **No rollback (v1)**: If the new binary fails to start, the bot stays down. Rollback is a future enhancement.

## Batch Update Behavior

When using `update_binary_all`:

- The instruction is sent to each connected bot sequentially.
- Per-bot results are collected and returned in the response, including individual status and error messages.
- Bot restarts will drop the WebSocket connection, which is expected — the bot reconnects automatically after restart.
