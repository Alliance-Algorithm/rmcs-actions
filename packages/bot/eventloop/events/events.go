package events

import (
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/bytedance/sonic"
)

type EventMessage struct {
	Event  string                 `json:"event"`
	Detail sonic.NoCopyRawMessage `json:"detail"`
}

type EventEmitter struct {
	Name   string
	Action lib.SessionAction
}

var eventEmitters = []EventEmitter{
	HeartbeatEventEmitter,
}

var EventEmitters = func() map[string]EventEmitter {
	emitterMap := make(map[string]EventEmitter)
	for _, emitter := range eventEmitters {
		emitterMap[emitter.Name] = emitter
	}
	return emitterMap
}()

var AutoStart = []EventEmitter{
	HeartbeatEventEmitter,
}
