package http

import (
	"net/http"

	"github.com/go-chi/chi/v5"

	"fieldnote/internal/billing"
	"fieldnote/internal/dispatch"
	"fieldnote/internal/partners"
	"fieldnote/internal/store"
	"fieldnote/internal/web"
)

// Router wires every route. The unauthenticated surface is the booking form and the
// Stripe webhook, both behind RateLimit; the webhook additionally verifies its signature
// inside the handler.
func Router(s *store.Store, wh *billing.Webhook) http.Handler {
	r := chi.NewRouter()
	r.Use(RequestID, StructuredLog)
	r.Get("/healthz", func(w http.ResponseWriter, _ *http.Request) { w.WriteHeader(200) })

	r.Group(func(r chi.Router) {
		r.Use(RateLimit(60, "1m")) // per IP and per slug; added 2026-08-22
		r.Post("/book/{slug}", dispatch.PublicBooking(s))
		r.Method("POST", "/webhooks/stripe", wh)
	})

	r.Group(func(r chi.Router) {
		r.Use(web.RequireSession(s), web.RequireEntitlement(s))
		r.With(web.Role("dispatcher")).Post("/jobs", dispatch.CreateJob(s))
		r.With(web.Role("dispatcher")).Post("/jobs/{id}/assign", dispatch.AssignJob(s))
		r.With(web.Role("dispatcher")).Put("/settings/partners", partners.SaveEndpoints(s))
		r.With(web.Role("dispatcher")).Post("/billing/checkout", billing.Checkout(s))
		r.Get("/me/jobs", dispatch.MyJobs(s))
		r.Post("/me/jobs/{id}/done", dispatch.MarkDone(s))
	})

	r.Handle("/*", web.Static())
	return r
}
