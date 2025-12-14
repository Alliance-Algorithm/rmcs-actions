package eventloop

import (
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop/events"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop/share"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"github.com/bytedance/sonic"
	"go.uber.org/zap"
)

func (s *SessionHub) startDispatch() {
	messageChan := s.ctx.Value(lib.WsReaderCtxKey{}).(chan sonic.NoCopyRawMessage)

	go s.startEmitters()

	for {
		select {
		case <-s.ctx.Done():
			return
		case msg, ok := <-messageChan:
			if !ok {
				logger.Logger().Error("Message channel closed unexpectedly")
				return
			}
			var event share.Message
			err := sonic.Unmarshal(msg, &event)
			if err != nil {
				logger.Logger().Error("Failed to unmarshal incoming message", zap.Error(err))
				continue
			}
			s.dispatchEvent(event)
		}
	}
}

func (s *SessionHub) dispatchEvent(event share.Message) {
	payload := event.Payload
	if payload.IsUnknown() {
		logger.Logger().Warn("Received unknown message type", zap.String("type", event.Payload.Type))
		return
	}

	sessionId := event.SessionID
	logger.Logger().Debug("Dispatching event", zap.String("session_id", sessionId.String()))

	s.lock.RLock()
	session, exists := s.sessions[sessionId]
	s.lock.RUnlock()

	if !exists {
		logger.Logger().Warn("No session found for event", zap.String("session_id", sessionId.String()))
		return
	}

	if payload.IsClose() {
		logger.Logger().Info("Session closed by remote", zap.String("session_id", sessionId.String()))
		close(session.readCh)
		session.CancelFunc()
		s.DeleteSession(sessionId)
	} else if payload.IsInstruction() {
		// Not implemented yet
	} else if payload.IsResponse() {
		session.readCh <- payload.Content
	} else if payload.IsEvent() {
		logger.Logger().Warn("Impossible for client to receive event messages, ignoring")
	}
}

func (s *SessionHub) startEmitters() {
	for _, emitter := range events.AutoStart {
		s.NewSession(emitter.Action)
	}
}
