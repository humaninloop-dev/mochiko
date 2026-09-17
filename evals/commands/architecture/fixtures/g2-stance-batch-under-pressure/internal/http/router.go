package http

import (
	"net/http"

	"github.com/go-chi/chi/v5"

	"fieldnote/internal/dispatch"
	"fieldnote/internal/partners"
	"fieldnote/internal/store"
	"fieldnote/internal/web"
)

// Router wires every route. The public booking form is mounted directly on the root
// router with only the request-id and logging middleware in front of it; the
// authenticated group adds the session check and the role check.
func Router(s *store.Store) http.Handler {
	r := chi.NewRouter()
	r.Use(RequestID, StructuredLog) // GI-005: one log line per request, request id as a Sentry tag
	r.Get("/healthz", func(w http.ResponseWriter, _ *http.Request) { w.WriteHeader(200) })

	// Unauthenticated (FEAT-003). No limiter: the 2026-07-09 scraper hit this at ~1 req/s
	// for an hour and every request became a `requested` job.
	r.Post("/book/{slug}", dispatch.PublicBooking(s))

	r.Group(func(r chi.Router) {
		r.Use(web.RequireSession(s))
		r.With(web.Role("dispatcher")).Post("/jobs", dispatch.CreateJob(s))
		r.With(web.Role("dispatcher")).Post("/jobs/{id}/assign", dispatch.AssignJob(s))
		r.With(web.Role("dispatcher")).Put("/settings/partners", partners.SaveEndpoints(s))
		r.Get("/me/jobs", dispatch.MyJobs(s))
		r.Post("/me/jobs/{id}/done", dispatch.MarkDone(s)) // enqueues partner_webhook (FEAT-005)
	})

	r.Handle("/*", web.Static())
	return r
}
