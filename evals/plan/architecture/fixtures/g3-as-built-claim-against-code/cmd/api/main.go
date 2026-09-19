package main

import (
	"context"
	"log"
	"net/http"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/stripe/stripe-go/v79"

	"fieldnote/internal/billing"
	fnhttp "fieldnote/internal/http"
	"fieldnote/internal/store"
)

// api: serves the SPA and the JSON routes; runs migrations on boot. Deferred work is a
// jobs row the worker process picks up.
func main() {
	ctx := context.Background()
	pool, err := pgxpool.New(ctx, os.Getenv("DATABASE_URL"))
	if err != nil {
		log.Fatalf("db: %v", err)
	}
	if err := store.Migrate(ctx, pool); err != nil {
		log.Fatalf("migrate: %v", err)
	}
	stripe.Key = os.Getenv("STRIPE_SECRET_KEY")
	wh := billing.NewWebhook(store.New(pool), os.Getenv("STRIPE_WEBHOOK_SECRET"))
	log.Fatal(http.ListenAndServe(":8080", fnhttp.Router(store.New(pool), wh)))
}
