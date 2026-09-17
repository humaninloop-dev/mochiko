package main

import (
	"context"
	"log"
	"net/http"
	"os"

	"github.com/go-chi/chi/v5"
	"github.com/jackc/pgx/v5/pgxpool"

	"fieldnote/internal/dispatch"
	"fieldnote/internal/store"
	"fieldnote/internal/web"
)

// api: serves the SPA and the JSON routes. Runs migrations on boot (D-001); never sends
// SMS itself — it writes a jobs row and the worker process does the sending (D-002).
func main() {
	ctx := context.Background()
	pool, err := pgxpool.New(ctx, os.Getenv("DATABASE_URL"))
	if err != nil {
		log.Fatalf("db: %v", err)
	}
	if err := store.Migrate(ctx, pool); err != nil {
		log.Fatalf("migrate: %v", err)
	}

	r := chi.NewRouter()
	r.Get("/healthz", func(w http.ResponseWriter, _ *http.Request) { w.WriteHeader(200) })

	// Unauthenticated: the public booking form (FEAT-003).
	r.Post("/book/{slug}", dispatch.PublicBooking(store.New(pool)))

	// Authenticated: session cookie, account-scoped repository.
	r.Group(func(r chi.Router) {
		r.Use(web.RequireSession(pool))
		r.Post("/jobs", dispatch.CreateJob(store.New(pool)))
		r.Post("/jobs/{id}/assign", dispatch.AssignJob(store.New(pool)))
		r.Get("/me/jobs", dispatch.MyJobs(store.New(pool)))
	})

	r.Handle("/*", web.Static())
	log.Fatal(http.ListenAndServe(":8080", r))
}
