package eventloop

import (
	"context"
	"sync"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"github.com/bytedance/sonic"
	"github.com/google/uuid"
	"go.uber.org/zap"
)

type SessionHub struct {
	sessions map[uuid.UUID]*Session
	lock     sync.RWMutex
	ctx      context.Context
}

func NewSessionHub(ctx context.Context) *SessionHub {
	return &SessionHub{
		sessions: make(map[uuid.UUID]*Session),
		lock:     sync.RWMutex{},
		ctx:      ctx,
	}
}

type Session struct {
	RobotId   uuid.UUID
	SessionId uuid.UUID
	// This channel is used to communicate with internal
	// running goroutine.
	// The internal goroutine will directly write to
	// the websocket connection.
	// And this channel is used to send messages
	// to the internal goroutine.
	readCh chan sonic.NoCopyRawMessage
}

func (s *SessionHub) NewSession(action lib.SessionAction) *Session {
	sessionId := uuid.New()

	return s.NewSessionWithId(sessionId, action)
}

func (s *SessionHub) NewSessionWithId(sessionId uuid.UUID, action lib.SessionAction) *Session {
	ctx := context.WithValue(s.ctx, lib.SessionIdCtxKey{}, sessionId)
	internalCh := make(chan sonic.NoCopyRawMessage)

	var robotId uuid.UUID
	if v := ctx.Value(lib.RobotIdCtxKey{}); v != nil {
		if rid, ok := v.(uuid.UUID); ok {
			robotId = rid
		}
	}

	session := &Session{
		RobotId:   robotId,
		SessionId: sessionId,
		readCh:    internalCh,
	}

	s.lock.Lock()
	s.sessions[session.SessionId] = session
	s.lock.Unlock()

	go func() {
		defer func() {
			s.DeleteSession(session.SessionId)
		}()
		actionCtx := context.WithValue(ctx, lib.ResponseReaderCtxKey{}, internalCh)
		(action)(actionCtx)
	}()

	return session
}

func (s *SessionHub) GetSession(sessionId uuid.UUID) (*Session, bool) {
	s.lock.RLock()
	session, ok := s.sessions[sessionId]
	s.lock.RUnlock()
	return session, ok
}

func (s *SessionHub) DeleteSession(sessionId uuid.UUID) {
	s.lock.Lock()
	delete(s.sessions, sessionId)
	s.lock.Unlock()
	logger.Logger().Info("Session deleted", zap.String("session_id", sessionId.String()))
}
