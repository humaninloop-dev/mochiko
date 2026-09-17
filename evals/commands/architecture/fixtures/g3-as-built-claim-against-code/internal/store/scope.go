package store

import (
	"context"

	"github.com/jackc/pgx/v5/pgxpool"
)

// Store is the only door to the database. Scoped(account) returns a repository that
// prepends the account_id predicate to every tenant-table statement; the isolation suite
// fails the build on any pool.Query outside this package (GI-003, NFR-003).
//
// Unscoped() exists for the two tables that carry no account: stripe_events and
// schema_migrations. It refuses any other table name.
type Store struct{ pool *pgxpool.Pool }

func New(pool *pgxpool.Pool) *Store { return &Store{pool: pool} }

type Scoped struct {
	pool    *pgxpool.Pool
	account string
}

func (s *Store) Scoped(account string) *Scoped { return &Scoped{pool: s.pool, account: account} }

func (s *Store) Unscoped() *Unscoped { return &Unscoped{pool: s.pool} }

func (s *Scoped) Exec(ctx context.Context, sql string, args ...any) error {
	_, err := s.pool.Exec(ctx, guard(sql, s.account), args...)
	return err
}

// Tenant tables (12): account, job, technician, customer, events, jobs, session,
// partner_endpoint, webhook_delivery, subscriptions, audit_events, invoice.
func guard(sql, account string) string { return enforceAccountPredicate(sql, account) }

type Unscoped struct{ pool *pgxpool.Pool }

func (u *Unscoped) Exec(ctx context.Context, sql string, args ...any) error {
	if !touchesOnly(sql, "stripe_events", "schema_migrations") {
		panic("store: unscoped statement on a tenant table")
	}
	_, err := u.pool.Exec(ctx, sql, args...)
	return err
}
