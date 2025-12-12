# Service Event Design

Consider the protocol between the bot and the service,
we choose to release the control over response & input
type to the background function themselves.

## Control Flow

First, the service creates a session hub
for a single connection with a robot instance.

The session hub is a `HashMap<Uuid, Action>`.
The uuid identifies the specific running session,
and the [`Action`](../../packages/service/src/service/action.rs)
is a wrapper for the background asynchronous task.

The session hub itself will hold a `mpsc::Receiver`
to receive messages prompted from the tasks,
and create multiple `mpsc::channel`s to send messages
to the tasks.
