package eventloop

import (
	"context"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
)

type OneShotAction[T any] func() T
type ResponseAction[T any, O any] func(request T) O

func WrapOneShotAction[T any](action OneShotAction[T]) lib.SessionAction {
	return func(ctx context.Context) {
		response := action()
		ctx.Value(lib.WsWriterCtxKey{}).(chan any) <- response
	}
}

func WrapResponseAction[T any, O any](action ResponseAction[T, O]) lib.SessionAction {
	return func(ctx context.Context) {
		request := (<-ctx.Value(lib.WsReaderCtxKey{}).(chan any)).(T)

		response := action(request)
		ctx.Value(lib.WsWriterCtxKey{}).(chan any) <- response
	}
}
