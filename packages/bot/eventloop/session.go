package eventloop

import (
	"context"
	"sync"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/bytedance/sonic"
	"github.com/google/uuid"
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
	RobotId    string
	SessionId  uuid.UUID
	CancelFunc func()
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
	ctx := context.WithValue(s.ctx, lib.SessionIdCtxKey{}, sessionId)

	return s.newSessionWithId(ctx, action)
}

func (s *SessionHub) newSessionWithId(ctx context.Context, action lib.SessionAction) *Session {
	internalCh := make(chan sonic.NoCopyRawMessage)
	// Safely extract RobotId from context; default to empty string if missing
	var robotId string
	if v := ctx.Value(lib.RobotIdCtxKey{}); v != nil {
		if rid, ok := v.(string); ok {
			robotId = rid
		}
	}
	sessionId := ctx.Value(lib.SessionIdCtxKey{}).(uuid.UUID)
	ctx, cancelFunc := context.WithCancel(ctx)
	session := &Session{
		RobotId:    robotId,
		SessionId:  sessionId,
		CancelFunc: cancelFunc,
		readCh:     internalCh,
	}

	s.lock.Lock()
	s.sessions[session.SessionId] = session
	s.lock.Unlock()

	go func() {
		defer cancelFunc()
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
}
