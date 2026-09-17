package store

import (
	"context"

	"github.com/jackc/pgx/v5/pgxpool"
)

// Store is the only door to the database. Scoped(account) returns a repository that
// prepends the account_id predicate to every tenant-table statement; the isolation suite
// fails the build on any pool.Query outside this package (GI-003, NFR-003).
type Store struct{ pool *pgxpool.Pool }

func New(pool *pgxpool.Pool) *Store { return &Store{pool: pool} }

type Scoped struct {
	pool    *pgxpool.Pool
	account string
}

func (s *Store) Scoped(account string) *Scoped { return &Scoped{pool: s.pool, account: account} }

func (s *Scoped) Exec(ctx context.Context, sql string, args ...any) error {
	_, err := s.pool.Exec(ctx, guard(sql, s.account), args...)
	return err
}

func (s *Scoped) Tx(ctx context.Context, fn func(Tx) error) error {
	tx, err := s.pool.Begin(ctx)
	if err != nil {
		return err
	}
	if err := fn(scopedTx{tx: tx, account: s.account}); err != nil {
		_ = tx.Rollback(ctx)
		return err
	}
	return tx.Commit(ctx)
}

// guard rejects a statement on a tenant table that carries no account_id predicate.
// Tenant tables: account, job, technician, customer, events, jobs, session.
func guard(sql, account string) string { return enforceAccountPredicate(sql, account) }
