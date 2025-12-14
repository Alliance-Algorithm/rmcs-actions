package eventloop

import (
	"context"

	"github.com/bytedance/sonic"
	"go.uber.org/zap"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
)

type EventloopBackend struct {
	SendJson func(ctx context.Context, v any) error
	RecvJson func(ctx context.Context, v *sonic.NoCopyRawMessage) error
}

func ServeOn(ctx context.Context, backend EventloopBackend) {
	serveEventloop(ctx, backend)
}

func serveEventloop(ctx context.Context, backend EventloopBackend) {
	send := make(chan any, 10)
	recv := make(chan any, 10)

	go eventloopSendJson(ctx, backend, send)
	go eventloopRecvJson(ctx, backend, recv)

	// Wait for context cancellation
	<-ctx.Done()
	close(send)
	logger.Logger().Info("Event loop shutting down")
}

func eventloopSendJson(ctx context.Context, backend EventloopBackend, send chan any) {
	for {
		select {
		case <-ctx.Done():
			logger.Logger().Info("Send goroutine shutting down")
			return
		case msg, ok := <-send:
			if !ok {
				return
			}
			logger.Logger().Debug("Sending JSON message", zap.Any("message", msg))
			err := backend.SendJson(ctx, msg)
			if err != nil {
				logger.Logger().Error("Failed to send JSON", zap.Error(err))
				return
			}
		}
	}
}

func eventloopRecvJson(ctx context.Context, backend EventloopBackend, response chan any) {
	defer close(response)
	for {
		select {
		case <-ctx.Done():
			logger.Logger().Info("Receive goroutine shutting down")
			return
		default:
		}

		var event sonic.NoCopyRawMessage
		err := backend.RecvJson(ctx, &event)
		logger.Logger().Debug("Received JSON message", zap.Any("message", event))
		if err != nil {
			if ctx.Err() != nil {
				logger.Logger().Info("Receive context cancelled")
			} else {
				logger.Logger().Error("Failed to receive JSON", zap.Error(err))
			}
			return
		}
		select {
		case <-ctx.Done():
			return
		case response <- event:
		}
	}
}
