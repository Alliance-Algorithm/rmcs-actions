package share

import (
	"github.com/bytedance/sonic"
	"github.com/google/uuid"
)

type Message struct {
	SessionID      uuid.UUID      `json:"session_id"`
	LocalTimestamp int64          `json:"local_timestamp"`
	Payload        MessagePayload `json:"payload"`
}

// MessagePayload represents the payload of a message
type MessagePayload struct {
	Type    string                 `json:"type"`
	Content sonic.NoCopyRawMessage `json:"content,omitempty"`
}

const (
	messageTypeInstruction = "instruction"
	messageTypeEvent       = "event"
	messageTypeResponse    = "response"
	messageTypeClose       = "close"
)

func (m *MessagePayload) IsInstruction() bool {
	return m.Type == messageTypeInstruction
}

func (m *MessagePayload) IsEvent() bool {
	return m.Type == messageTypeEvent
}

func (m *MessagePayload) IsResponse() bool {
	return m.Type == messageTypeResponse
}

func (m *MessagePayload) IsClose() bool {
	return m.Type == messageTypeClose
}

func (m *MessagePayload) IsUnknown() bool {
	return m.Type != messageTypeInstruction &&
		m.Type != messageTypeEvent &&
		m.Type != messageTypeResponse &&
		m.Type != messageTypeClose
}
