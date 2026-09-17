package worker

import (
	"context"
	"log"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
)

// Poller claims one due row at a time with FOR UPDATE SKIP LOCKED and runs its handler.
// At-least-once: every handler checks its own event before acting. Two kinds today:
// send_eta_sms (FEAT-002) and partner_webhook (FEAT-005).
type Poller struct {
	pool     *pgxpool.Pool
	every    time.Duration
	handlers map[string]Handler
}

type Handler func(ctx context.Context, job Job) error

func NewPoller(pool *pgxpool.Pool, every time.Duration) *Poller {
	return &Poller{pool: pool, every: every, handlers: map[string]Handler{}}
}

func (p *Poller) Handle(kind string, h Handler) { p.handlers[kind] = h }

func (p *Poller) Run(ctx context.Context) {
	t := time.NewTicker(p.every)
	defer t.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-t.C:
			p.tick(ctx)
		}
	}
}

func (p *Poller) tick(ctx context.Context) {
	for {
		job, ok := claim(ctx, p.pool)
		if !ok {
			return
		}
		h, known := p.handlers[job.Kind]
		if !known {
			log.Printf("worker: no handler for %s, marking failed", job.Kind)
			fail(ctx, p.pool, job, "no handler")
			continue
		}
		if err := h(ctx, job); err != nil {
			retryLater(ctx, p.pool, job, err)
			continue
		}
		done(ctx, p.pool, job)
	}
}
