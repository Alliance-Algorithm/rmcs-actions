# Action Protocol

This page defines the protocol used in the message interaction
between the server and the robot daemon.

The frontend's API is documented elsewhere.

## Presuppose

-   The daemon _shall_ save its __robot id__ which is unique
    and is bound to it's MAC address.
    The robot id is used to identify the robot.
-   A robot _shall not_ run more than one daemon in
    one or more container or on the host machine.

## Connection

The server maintains a websocket service at `/ws`,
and the bot's daemon serves as a websocket client.

### Establishment

Before starting the long connection, the robot _shall_
first try to identify itself.
This is done by:
1.  Fetch local storage to get a __cached__ robot id.
2.  If there's no predefined robot id found,
    send a request to `/ident/whoami` providing necessary
    identification messages including MAC address.
    The server will grant the robot a unique __robot id__
    and the robot _shall_ store it properly.  
    For more information, see the server's documentation
    or visit [here](#whoami) to get a quick review.

The bot's daemon will then try to connect to the `/ws/:robot-id` endpoint
after startup (which is triggered either manually or by the container.)

The connection _shall_ be a long WebSocket connection,
which is only allowed to be closed manually or explicitly.

### Closing

The robot's daemon will _not_ proactively send closing messages.
The server _shall_ view the robot as "offline" when the connection
is closed.

The robot's daemon _shall_ accept "shutdown" or "restart"
instructions to close or restart the daemon itself.

### Format

The connection _shall_ send and receive JSON messages.

## Communication

Given that Websocket connection is asynchronous,
each conversation in a long connection _shall_ be identified with a unique identifier,
whose detail is free to the implementation.

The message pack shares this format:
```jsonc
{
    "session_id": <unique session id>,
    "local_timestamp": <local UTC+0 timestamp>,
    "payload": <arbitrary payload object>
}
```

### Closing a Communication

A communication is closed if either of the side sends a close message with payload of:
```jsonc
{
    "type": "close"
}
```

Either part will drop the session controller once send the message,
and the other part _shall_ close it and drop the session controller then.

### Server Instructions

This section defines instructions sent by the server.

All instructions share this payload format:
```json
{
    "type": "instruction",
    "content": {
        "instruction": "<instruction endpoint>",
        "message": <message payload>
    }
}
```
And their responses share this response format:
```json
{
    "type": "response",
    "content": <response payload>
}
```

#### Rename (`sync_robot_id`)

**Name**: Rename  
**Endpoint**: `sync_robot_id`  
**Description**: Force the robot to change its __robot id__.

**Message**:
```json
{
    "id": "<new robot id>"
}
```

**NO RESPONSE**

#### Get Network Info (`fetch_network`)

**Name**: Get network info  
**Endpoint**: `fetch_network`  
**Description**: Request the robot to send its network info.

**Message**:
```json
{ }
```

**Response**:
```jsonc
{
    "<NIC>": {
        "nic": "<NIC>",
        "stats": {
            "speed": 0.0,
            "up": true,
            "mtu": 10, // nullable
            "duplex": "<full|half|unknown>" // nullable
        },
        "io": {
            "incoming": 0.0,
            "outgoing": 0.0,
            "incoming_errs": 0.0,
            "outgoing_errs": 0.0,
            "incoming_drops": 0.0,
            "outgoing_drops": 0.0,
        },
        "addrs": {
            "ipv4": <ip addr>, // nullable
            "ipv6": <ip addr>, // nullable
            "mac": <ip addr>,  // nullable
            "unknown": [ <ip addr> ]
        }
    }
}
```
Where `ip addr` is:
```jsonc
{
    "address": "<address>",
    "netmask": "<netmask>",
    "broadcast": "<broadcast>",
    "family": 0   // AF family
}
```

### Daemon Events

This section defines daemon events pushed towards server.

All events share the following format:
```json
{
    "type": "event",
    "content": {
        "event": "<event type>",
        "detail": <event detail object>
    }
}
```

#### Heartbeat (`heartbeat`)

**Name**: Heartbeat  
**Event Type**: `heartbeat`  
**Description**: Heartbeat event, signalling the status of the daemon.

**Detail**:
```json
{ }
```

**Response**:
```json
{ }
```

## Appendix

### Whoami

**WARNING:** THIS SECTION MIGHT BE OUTDATED.

**Method**: `POST`
**Endpoint**: `/ident/whoami`
**Request**:
```jsonc
{
    "username": "<current login username>",
    "mac": "<MAC address>",
}
```
**Response**:
```jsonc
{
    "robot_id": "<robot id>"
}
```
