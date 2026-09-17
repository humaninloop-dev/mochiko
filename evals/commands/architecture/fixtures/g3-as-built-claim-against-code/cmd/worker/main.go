package main

import (
	"context"
	"log"
	"os"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"

	"fieldnote/internal/notify"
	"fieldnote/internal/partners"
	"fieldnote/internal/worker"
)

// worker: polls the jobs table every 5 s and runs one handler per job kind.
// Registered kinds: send_eta_sms (FEAT-002), partner_webhook (FEAT-005).
func main() {
	ctx := context.Background()
	pool, err := pgxpool.New(ctx, os.Getenv("DATABASE_URL"))
	if err != nil {
		log.Fatalf("db: %v", err)
	}
	sms := notify.NewTwilio(os.Getenv("TWILIO_SID"), os.Getenv("TWILIO_TOKEN"), os.Getenv("TWILIO_FROM"))
	eta := notify.NewMapbox(os.Getenv("MAPBOX_TOKEN"))

	p := worker.NewPoller(pool, 5*time.Second)
	p.Handle("send_eta_sms", worker.SendETASMS(pool, eta, sms))
	p.Handle("partner_webhook", partners.Deliver(pool))
	log.Println("worker: polling jobs")
	p.Run(ctx)
}
