package events

import (
	"context"
	"time"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop/share"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"github.com/bytedance/sonic"
)

type HeartbeatEventRequest struct{}
type HeartbeatEventResponse struct{}

const HeartbeatEventName = "heartbeat"

var HeartbeatEventEmitter = EventEmitter{
	Name:   HeartbeatEventName,
	Action: HeartbeatAction,
}

func HeartbeatAction(ctx context.Context) {
	writer := ctx.Value(lib.WsWriterCtxKey{}).(chan any)
	reader := ctx.Value(lib.ResponseReaderCtxKey{}).(chan sonic.NoCopyRawMessage)

	writer <- share.NewMessage(ctx, share.NewEvent(HeartbeatEventName, HeartbeatEventRequest{}))

	for {
		select {
		case <-ctx.Done():
			return
		case <-time.After(5 * time.Second):
			writer <- share.NewMessage(ctx, share.NewResponse(HeartbeatEventResponse{}))
		case msg, ok := <-reader:
			if ok {
				var heartbeatResp HeartbeatEventResponse
				err := sonic.Unmarshal(msg, &heartbeatResp)
				if err != nil {
					continue
				}
				logger.Logger().Debug("Received heartbeat response")
			} else {
				return
			}
		}
	}
}
