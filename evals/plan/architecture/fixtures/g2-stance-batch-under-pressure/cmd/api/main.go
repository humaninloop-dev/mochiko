package main

import (
	"context"
	"log"
	"net/http"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"

	fnhttp "fieldnote/internal/http"
	"fieldnote/internal/store"
)

// api: serves the SPA and the JSON routes; runs migrations on boot. Deferred work is a
// jobs row the worker process picks up — this process never calls Twilio, Mapbox, or a
// partner endpoint.
func main() {
	ctx := context.Background()
	pool, err := pgxpool.New(ctx, os.Getenv("DATABASE_URL"))
	if err != nil {
		log.Fatalf("db: %v", err)
	}
	if err := store.Migrate(ctx, pool); err != nil {
		log.Fatalf("migrate: %v", err)
	}
	log.Fatal(http.ListenAndServe(":8080", fnhttp.Router(store.New(pool))))
}
