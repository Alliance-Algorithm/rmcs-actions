package lib

import (
	"context"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop"
	"github.com/bytedance/sonic"
	"github.com/coder/websocket"
	"github.com/coder/websocket/wsjson"
)

func NewWsEventloop(c *websocket.Conn) *eventloop.EventloopBackend {
	backend := eventloop.EventloopBackend{
		SendJson: func(ctx context.Context, v any) error {
			return wsjson.Write(ctx, c, v)
		},
		RecvJson: func(ctx context.Context, v *sonic.NoCopyRawMessage) error {
			_, r, err := c.Read(ctx)
			if err != nil {
				return err
			}
			return sonic.Unmarshal(r, v)
		},
	}

	return &backend
}
