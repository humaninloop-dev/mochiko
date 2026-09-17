package jobs

import (
	"context"
	"encoding/json"

	"fieldnote/internal/store"
)

// Enqueue inserts one row into the jobs table for the worker to claim. Callers today:
// dispatch.AssignJob (send_eta_sms) and dispatch.MarkDone (partner_webhook).
func Enqueue(ctx context.Context, s *store.Scoped, kind string, ref string, payload any) error {
	body, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	return s.Exec(ctx,
		`INSERT INTO jobs (kind, account_id, ref, payload, run_after) VALUES ($1, current_setting('app.account_id'), $2, $3, now())`,
		kind, ref, body)
}
