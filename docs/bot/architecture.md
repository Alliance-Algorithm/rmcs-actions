# Robot Daemon Architecture

The robot daemon is a WebSocket client maintaining
a long connection with the web service.

- [Robot Daemon Architecture](#robot-daemon-architecture)
  - [WebSocket Connection](#websocket-connection)
  - [Eventloop](#eventloop)
  - [Session Hub](#session-hub)
  - [Event Trigger](#event-trigger)
    - [Discipline](#discipline)


## WebSocket Connection

The core of the connection is a `*websocket.Conn`.

This is wrapped by a `eventloop.EventloopBackend`
as an abstraction layer above the implementation,
offering JSON send/recv service.

After executing `eventloop.ServeOn` we won't
have any further direct interaction with
the underlying connection

## Eventloop

The eventloop works as an abstraction layer
for the connection so we could potentially
change the underlying support to,
for example, a bluetooth implementation.

The eventloop will create two goroutines
for asynchronous interaction.

**Note** The receiver will pre-parse the output
to a `sonic.NoCopyRawMessage`.

## Session Hub

Session hub is the core of the application.
It stores the handle of every sub-session running.

Each session is named `SessionAction`,
which is basically a wrapper of both the action's
name and it's code (a `func(context.Context)`)

An action requires basically two channels,
`eventloop.WsWriterCtxKey` for sending and
`eventloop.ResponseReaderCtxKey` for reading.
The behavior of the channels differ.

There's basically four types of message:
- Instruction, sent by the server.
- Event, triggered locally and post to the server.
- Response, in-session messages.
- Close, special message that some event(like heartbeat) might uses.

For **instruction**s, the write channel offers no
wrapping and is just the channel provided by
the eventloop, while the read channel is pre-parsed.
The read channel will first receive the initial request,
or the `message` field. Then, if there's any further
responses, their `content` field will be sent.
If a `close` message is sent, the receiver channel
will be closed.

For **event**s, since they're self-triggered,
no initial messages are provided.
After the initialization, the two channels
will work the same as those used in **instruction**s.

For **response**s, first their session-id will
be lookup in the session hub, and then sent
to the related action.

For **close**s, the matching action's read channel
will be closed.

## Event Trigger

The events are self-triggered, so we cannot
define a specific routine for them.

All event emitters are registered in `events.EventEmitters`.

### Discipline

- Clock-based:
  Clock-based triggers are initialized as background actions
  on startup and left for internal management.

  To automatically start the action, add an item
  to `events.AutoStart` list.
