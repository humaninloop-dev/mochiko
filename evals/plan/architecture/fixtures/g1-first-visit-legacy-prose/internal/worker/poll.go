package worker

import (
	"context"
	"log"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"

	"fieldnote/internal/notify"
)

// Poller claims one due row at a time with FOR UPDATE SKIP LOCKED and runs its handler.
// At-least-once: a handler that crashed after its side effect will run again, so every
// handler checks its own event before acting (D-002).
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
			retryLater(ctx, p.pool, job, err) // 1, 2, 4, 8 min, then every 15 min for 2 h
			continue
		}
		done(ctx, p.pool, job)
	}
}

// SendETASMS is the send_eta_sms handler (FEAT-002): idempotent on the job's sms_sent event,
// ETA from Mapbox (a failure degrades to an ETA-less text, D-004), then one Twilio send.
func SendETASMS(pool *pgxpool.Pool, eta *notify.Mapbox, sms *notify.Twilio) Handler {
	return func(ctx context.Context, job Job) error {
		if alreadySent(ctx, pool, job.JobID) {
			return nil
		}
		j := loadJob(ctx, pool, job.AccountID, job.JobID)
		body := "Your technician " + j.TechnicianName + " is on the way."
		if mins, err := eta.Minutes(ctx, j.TechnicianLastPos, j.Address); err == nil {
			body = "Your technician " + j.TechnicianName + " will arrive in about " + itoa(mins) + " minutes."
		}
		sid, err := sms.Send(ctx, j.CustomerPhone, body)
		if err != nil {
			return err
		}
		return recordEvent(ctx, pool, job.JobID, "sms_sent", sid)
	}
}
