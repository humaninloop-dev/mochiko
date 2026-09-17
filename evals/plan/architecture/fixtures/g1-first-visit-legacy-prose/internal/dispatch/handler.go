package dispatch

import (
	"encoding/json"
	"net/http"

	"github.com/go-chi/chi/v5"

	"fieldnote/internal/store"
	"fieldnote/internal/web"
)

// AssignJob sets the technician on a job and enqueues the customer's ETA SMS in the same
// transaction. The SMS itself is sent by the worker process (see internal/worker); this
// handler never calls Twilio or Mapbox, so the dispatcher's click returns in one round trip.
func AssignJob(s *store.Store) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		acct := web.AccountFrom(r.Context())
		var in struct {
			TechnicianID string `json:"technician_id"`
		}
		if err := json.NewDecoder(r.Body).Decode(&in); err != nil {
			http.Error(w, "bad request", 400)
			return
		}
		jobID := chi.URLParam(r, "id")
		err := s.Scoped(acct).Tx(r.Context(), func(tx store.Tx) error {
			if err := tx.Exec(`UPDATE job SET technician_id=$1, assigned_at=now(), state='assigned' WHERE id=$2`, in.TechnicianID, jobID); err != nil {
				return err
			}
			return tx.Exec(`INSERT INTO jobs (kind, account_id, job_id, run_after) VALUES ('send_eta_sms', $1, $2, now())`, acct, jobID)
		})
		if err != nil {
			http.Error(w, "assign failed", 500)
			return
		}
		w.WriteHeader(200)
	}
}

// PublicBooking is the one unauthenticated write: it creates a job in state `requested`
// under the account the slug names and emails the dispatcher. No rate limit sits in front
// of it today; the router mounts it directly.
func PublicBooking(s *store.Store) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		slug := chi.URLParam(r, "slug")
		acct, err := s.AccountBySlug(r.Context(), slug)
		if err != nil {
			http.NotFound(w, r)
			return
		}
		var in struct {
			Name, Phone, Address, Problem string
		}
		if err := json.NewDecoder(r.Body).Decode(&in); err != nil {
			http.Error(w, "bad request", 400)
			return
		}
		if err := s.Scoped(acct).Exec(r.Context(), `INSERT INTO job (account_id, customer_name, customer_phone, address, problem, state) VALUES ($1,$2,$3,$4,$5,'requested')`, acct, in.Name, in.Phone, in.Address, in.Problem); err != nil {
			http.Error(w, "booking failed", 500)
			return
		}
		w.WriteHeader(201)
	}
}

func CreateJob(s *store.Store) http.HandlerFunc { return createJob(s) }
func MyJobs(s *store.Store) http.HandlerFunc    { return myJobs(s) }
