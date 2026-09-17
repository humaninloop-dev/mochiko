package billing

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"fieldnote/internal/store/storetest"
)

// TestSubscriptionUpdatedUpsertsInline: a signed customer.subscription.updated event
// results in a subscriptions row for the account before the handler returns 200.
func TestSubscriptionUpdatedUpsertsInline(t *testing.T) {
	db := storetest.Fresh(t)
	stripeStub := storetest.StripeStub(t, "sub_123", `{"status":"active","metadata":{"account_id":"acct_harlow"}}`)
	defer stripeStub.Close()

	w := NewWebhook(db.Store(), "whsec_test")
	req := httptest.NewRequest("POST", "/webhooks/stripe", strings.NewReader(signedEvent(t, "customer.subscription.updated", "sub_123")))
	req.Header.Set("Stripe-Signature", sign(t, "whsec_test"))
	rec := httptest.NewRecorder()
	w.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("status %d", rec.Code)
	}
	if got := db.Count(t, "subscriptions", "account_id='acct_harlow'"); got != 1 {
		t.Fatalf("subscriptions rows = %d, want 1 (inline upsert)", got)
	}
	if got := db.Count(t, "jobs", "kind='stripe_event'"); got != 0 {
		t.Fatalf("jobs rows = %d, want 0 (nothing enqueued at launch)", got)
	}
}

// TestBadSignatureRejected: an unsigned body never reaches the store.
func TestBadSignatureRejected(t *testing.T) {
	db := storetest.Fresh(t)
	w := NewWebhook(db.Store(), "whsec_test")
	req := httptest.NewRequest("POST", "/webhooks/stripe", strings.NewReader(`{}`))
	rec := httptest.NewRecorder()
	w.ServeHTTP(rec, req)
	if rec.Code != 400 {
		t.Fatalf("status %d, want 400", rec.Code)
	}
	if got := db.Count(t, "stripe_events", "true"); got != 0 {
		t.Fatalf("stripe_events rows = %d, want 0", got)
	}
}
