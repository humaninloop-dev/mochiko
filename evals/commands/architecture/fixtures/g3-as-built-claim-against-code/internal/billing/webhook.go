package billing

import (
	"encoding/json"
	"io"
	"log"
	"net/http"

	"github.com/stripe/stripe-go/v79"
	"github.com/stripe/stripe-go/v79/subscription"
	"github.com/stripe/stripe-go/v79/webhook"

	"fieldnote/internal/store"
)

// Webhook receives Stripe's subscription events at POST /webhooks/stripe.
type Webhook struct {
	s      *store.Store
	secret string
}

func NewWebhook(s *store.Store, secret string) *Webhook { return &Webhook{s: s, secret: secret} }

// ServeHTTP verifies the signature, then handles the event in the request.
//
// TODO(FEAT-006): move the fetch + upsert to a worker handler once the stripe_event
// consumer lands; for launch we process inline so entitlements update immediately.
func (w *Webhook) ServeHTTP(rw http.ResponseWriter, r *http.Request) {
	payload, err := io.ReadAll(io.LimitReader(r.Body, 1<<16))
	if err != nil {
		http.Error(rw, "read", 400)
		return
	}
	ev, err := webhook.ConstructEvent(payload, r.Header.Get("Stripe-Signature"), w.secret)
	if err != nil {
		http.Error(rw, "bad signature", 400)
		return
	}

	if err := w.s.Unscoped().Exec(r.Context(),
		`INSERT INTO stripe_events (id, type, payload) VALUES ($1,$2,$3) ON CONFLICT (id) DO NOTHING`,
		ev.ID, ev.Type, payload); err != nil {
		http.Error(rw, "record", 500)
		return
	}

	switch ev.Type {
	case "customer.subscription.created", "customer.subscription.updated", "customer.subscription.deleted":
		var sub stripe.Subscription
		if err := json.Unmarshal(ev.Data.Raw, &sub); err != nil {
			http.Error(rw, "decode", 400)
			return
		}
		// Re-fetch so we act on Stripe's current view, not the event snapshot.
		fresh, err := subscription.Get(sub.ID, nil)
		if err != nil {
			log.Printf("billing: fetch %s: %v", sub.ID, err)
			rw.WriteHeader(200) // acknowledge; Stripe would otherwise retry for three days
			return
		}
		acct := fresh.Metadata["account_id"]
		if err := upsertSubscription(r.Context(), w.s.Scoped(acct), fresh); err != nil {
			log.Printf("billing: upsert %s: %v", sub.ID, err)
		}
		if err := w.s.Scoped(acct).Exec(r.Context(),
			`INSERT INTO audit_events (account_id, kind, ref) VALUES ($1,'plan_changed',$2)`, acct, sub.ID); err != nil {
			log.Printf("billing: audit %s: %v", sub.ID, err)
		}
	}
	rw.WriteHeader(200)
}

func upsertSubscription(ctx contextT, s *store.Scoped, sub *stripe.Subscription) error {
	return s.Exec(ctx,
		`INSERT INTO subscriptions (account_id, stripe_id, plan, status, current_period_end)
		 VALUES ($1,$2,$3,$4,to_timestamp($5))
		 ON CONFLICT (account_id) DO UPDATE SET plan=$3, status=$4, current_period_end=to_timestamp($5)`,
		sub.Metadata["account_id"], sub.ID, sub.Items.Data[0].Price.LookupKey, string(sub.Status), sub.CurrentPeriodEnd)
}
