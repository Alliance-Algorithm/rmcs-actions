package lib

// SendEnvelope wraps a message sent through the WsWriter channel with
// an optional Done channel.  When Done is non-nil, the eventloop send
// goroutine closes it after the WebSocket write completes, allowing
// the sender to block until the message has been flushed.
type SendEnvelope struct {
	Payload any
	Done    chan struct{}
}

type RobotIdCtxKey struct{}
type SessionIdCtxKey struct{}
type WsWriterCtxKey struct{}
type WsReaderCtxKey struct{}
type ResponseReaderCtxKey struct{}
type WsSendDoneCtxKey struct{}
