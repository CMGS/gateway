// Package redis provides fleet-shared control-plane sessions.
package redis

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	redisclient "github.com/redis/go-redis/v9"

	"github.com/cocoonstack/gateway/control-plane/internal/kv"
)

const keyPrefix = "gateway:control-plane:session:"

var _ kv.Sessions = (*Sessions)(nil)

type Sessions struct {
	client *redisclient.Client
}

func Connect(ctx context.Context, rawURL string) (*Sessions, error) {
	opts, err := redisclient.ParseURL(rawURL)
	if err != nil {
		return nil, fmt.Errorf("parse redis url: %w", err)
	}
	client := redisclient.NewClient(opts)
	if err := client.Ping(ctx).Err(); err != nil {
		_ = client.Close()
		return nil, fmt.Errorf("ping redis: %w", err)
	}
	return &Sessions{client: client}, nil
}

func (s *Sessions) Put(ctx context.Context, session kv.Session) error {
	body, err := json.Marshal(session)
	if err != nil {
		return fmt.Errorf("encode session: %w", err)
	}
	ttl := time.Until(time.Unix(session.ExpiresAt, 0))
	if ttl <= 0 {
		return kv.ErrNotFound
	}
	if err := s.client.Set(ctx, keyPrefix+session.ID, body, ttl).Err(); err != nil {
		return fmt.Errorf("store session: %w", err)
	}
	return nil
}

func (s *Sessions) Get(ctx context.Context, id string) (kv.Session, error) {
	body, err := s.client.Get(ctx, keyPrefix+id).Bytes()
	if errors.Is(err, redisclient.Nil) {
		return kv.Session{}, kv.ErrNotFound
	}
	if err != nil {
		return kv.Session{}, fmt.Errorf("read session: %w", err)
	}
	var session kv.Session
	if err := json.Unmarshal(body, &session); err != nil {
		return kv.Session{}, fmt.Errorf("decode session: %w", err)
	}
	if session.ExpiresAt <= time.Now().Unix() {
		_ = s.Delete(ctx, id)
		return kv.Session{}, kv.ErrNotFound
	}
	return session, nil
}

func (s *Sessions) Delete(ctx context.Context, id string) error {
	if err := s.client.Del(ctx, keyPrefix+id).Err(); err != nil {
		return fmt.Errorf("delete session: %w", err)
	}
	return nil
}

func (s *Sessions) Close() error { return s.client.Close() }
