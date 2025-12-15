package share

import (
	"context"

	"github.com/bytedance/sonic"
	"go.uber.org/zap"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
)

type OneShotAction[T any] func(request T)
type ResponseAction[T any, O any] func(request T) O

func WrapOneShotAction[T any](action OneShotAction[T]) lib.SessionAction {
	return func(ctx context.Context) {
		request := (<-ctx.Value(lib.ResponseReaderCtxKey{}).(chan sonic.NoCopyRawMessage))
		var req T
		err := sonic.Unmarshal(request, &req)
		if err != nil {
			logger.Logger().Error("Failed to unmarshal request in OneShotAction", zap.Error(err))
			return
		}
		(action)(req)
	}
}

func WrapResponseAction[T any, O any](action ResponseAction[T, O]) lib.SessionAction {
	return func(ctx context.Context) {
		request := (<-ctx.Value(lib.ResponseReaderCtxKey{}).(chan sonic.NoCopyRawMessage))
		var req T
		if len(request) == 0 {
			req = *new(T)
		} else {
			err := sonic.Unmarshal(request, &req)
			if err != nil {
				logger.Logger().Error("Failed to unmarshal request in ResponseAction", zap.Error(err))
				return
			}
		}

		response := (action)(req)
		wrapped := NewMessage(ctx, NewResponse(response))
		ctx.Value(lib.WsWriterCtxKey{}).(chan any) <- wrapped
	}
}
