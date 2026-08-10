# Roadmap

## Thesis

Ledgerline is a trustworthy invoicing + payment-tracking tool for small US contractors — the two
bets are that contractors will trust payment state without re-checking Stripe, and that a solo
founder can ship production-quality financial software by leaning on CI, not on a review team.

## Now / Next / Later

**Now**
- Stand up GitHub Actions CI running the quality gates — [BACKLOG](BACKLOG.md#infrastructure) (2026-08-10)
- Plan the v1 invoice-lifecycle features via `/mochiko:plan`, dependency order FEAT-007 → FEAT-001 → FEAT-005 → FEAT-003 → FEAT-004 — [spec](.mochiko/specs/invoice-lifecycle-v1/spec.md) (2026-08-10)

**Next**
- Build the v1 invoice-lifecycle features (`/mochiko:plan` → `/mochiko:implement`, one run per selected feature)
- Fast-follow: invoice void (FEAT-006) and overdue reminder emails (FEAT-008), both `proposed` on the map

**Later**
- Resolve data-retention / deletion policy — [BACKLOG](BACKLOG.md#open-questions)

## Standing bets & revisit conditions

- Warn-only complexity enforcement (GI-008) suffices for a solo dev — revisit if code becomes unmaintainable or a second engineer joins.
- No staging environment pre-launch — revisit when the first paying customer is onboarded.

_Last groomed: 2026-08-10 · baseline: 2 open items, 0 sessions, 1 spec (invoice-lifecycle-v1)._
