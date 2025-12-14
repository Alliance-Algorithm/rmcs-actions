package eventloop

import "context"

type OneShotAction[T any] func() T
type ResponseAction[T any, O any] func(request T) O

func WrapOneShotAction[T any](action OneShotAction[T]) SessionAction {
	return func(ctx context.Context) {
		response := action()
		ctx.Value(WsWriterCtxKey{}).(chan any) <- response
	}
}

func WrapResponseAction[T any, O any](action ResponseAction[T, O]) SessionAction {
	return func(ctx context.Context) {
		request := (<-ctx.Value(WsReaderCtxKey{}).(chan any)).(T)

		response := action(request)
		ctx.Value(WsWriterCtxKey{}).(chan any) <- response
	}
}
