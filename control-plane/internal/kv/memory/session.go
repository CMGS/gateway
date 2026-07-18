// Package memory provides the dependency-free development session store.
package memory

import (
	"context"
	"sync"
	"time"

	"github.com/cocoonstack/gateway/control-plane/internal/kv"
)

var _ kv.Sessions = (*Sessions)(nil)

type Sessions struct {
	mu       sync.RWMutex
	sessions map[string]kv.Session
}

func New() *Sessions {
	return &Sessions{sessions: make(map[string]kv.Session)}
}

func (s *Sessions) Put(_ context.Context, session kv.Session) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.sessions[session.ID] = session
	return nil
}

func (s *Sessions) Get(_ context.Context, id string) (kv.Session, error) {
	s.mu.RLock()
	session, ok := s.sessions[id]
	s.mu.RUnlock()
	if !ok {
		return kv.Session{}, kv.ErrNotFound
	}
	if session.ExpiresAt <= time.Now().Unix() {
		s.mu.Lock()
		delete(s.sessions, id)
		s.mu.Unlock()
		return kv.Session{}, kv.ErrNotFound
	}
	return session, nil
}

func (s *Sessions) Delete(_ context.Context, id string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	delete(s.sessions, id)
	return nil
}

func (s *Sessions) Close() error { return nil }
