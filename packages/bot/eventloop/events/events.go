package events

import "github.com/bytedance/sonic"

type EventMessage struct {
	Event  string                 `json:"event"`
	Detail sonic.NoCopyRawMessage `json:"detail"`
}
