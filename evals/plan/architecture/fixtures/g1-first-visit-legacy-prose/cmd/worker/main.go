package main

import (
	"context"
	"log"
	"os"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"

	"fieldnote/internal/notify"
	"fieldnote/internal/worker"
)

// worker: polls the jobs table every 5 s (D-002) and runs one handler per job kind.
// The only kind today is send_eta_sms (FEAT-002). Never migrates (D-001).
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
	log.Println("worker: polling jobs")
	p.Run(ctx)
}
