package share

import (
	"context"
	"time"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/google/uuid"
)

type SenderMessage struct {
	SessionID      uuid.UUID `json:"session_id"`
	LocalTimestamp int64     `json:"local_timestamp"`
	Payload        any       `json:"payload"`
}

type Response struct {
	// This will always be "response"
	TypeName string `json:"type"`
	Content  any    `json:"content"`
}

func NewResponse(content any) *Response {
	return &Response{
		TypeName: "response",
		Content:  content,
	}
}

type Event struct {
	// This will always be "event"
	TypeName string        `json:"type"`
	Content  eventInternal `json:"content"`
}

type eventInternal struct {
	Event  string `json:"event"`
	Detail any    `json:"detail"`
}

func NewEvent(eventName string, detail any) *Event {
	return &Event{
		TypeName: "event",
		Content: eventInternal{
			Event:  eventName,
			Detail: detail,
		},
	}
}

func NewMessage(ctx context.Context, payload any) *SenderMessage {
	sessionId := ctx.Value(lib.SessionIdCtxKey{}).(uuid.UUID)
	return &SenderMessage{
		SessionID:      sessionId,
		LocalTimestamp: time.Now().UnixMilli(),
		Payload:        payload,
	}
}
