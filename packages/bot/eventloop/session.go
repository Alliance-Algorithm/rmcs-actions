package eventloop

import (
	"context"
	"sync"

	"github.com/bytedance/sonic"
	"github.com/google/uuid"
)

type SessionHub struct {
	sessions map[uuid.UUID]*Session
	lock     sync.RWMutex
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
	writeCh chan sonic.NoCopyRawMessage
}

type SessionAction = func(ctx context.Context)

type RobotIdCtxKey struct{}
type SessionIdCtxKey struct{}
type WsWriterCtxKey struct{}
type WsReaderCtxKey struct{}

func (s *SessionHub) NewSession(ctx context.Context, action *SessionAction) *Session {
	sessionId := uuid.New()
	ctx = context.WithValue(ctx, SessionIdCtxKey{}, sessionId)

	return s.NewSessionWithId(ctx, action)
}

func (s *SessionHub) NewSessionWithId(ctx context.Context, action *SessionAction) *Session {
	internalCh := make(chan sonic.NoCopyRawMessage)
	robotId := ctx.Value(RobotIdCtxKey{}).(string)
	sessionId := ctx.Value(SessionIdCtxKey{}).(uuid.UUID)
	ctx, cancelFunc := context.WithCancel(ctx)

	session := &Session{
		RobotId:    robotId,
		SessionId:  sessionId,
		CancelFunc: cancelFunc,
		writeCh:    internalCh,
	}

	s.lock.Lock()
	s.sessions[session.SessionId] = session
	s.lock.Unlock()

	go func() {
		defer cancelFunc()
		actionCtx := context.WithValue(ctx, WsReaderCtxKey{}, session.writeCh)
		(*action)(actionCtx)
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
