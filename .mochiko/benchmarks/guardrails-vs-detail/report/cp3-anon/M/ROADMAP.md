# Roadmap

## Thesis

Ledgerline is a trustworthy invoicing + payment-tracking tool for small US contractors — the two
bets are that contractors will trust payment state without re-checking Stripe, and that a solo
founder can ship production-quality financial software by leaning on CI, not on a review team.

## Now / Next / Later

**Now**
- Stand up GitHub Actions CI running the quality gates — [BACKLOG](BACKLOG.md#infrastructure) (2026-08-10)
- Plan the invoice-lifecycle spine — FEAT-001 → FEAT-003 → FEAT-004 → FEAT-005 via `/mochiko:plan` in dependency order — [spec](.mochiko/specs/invoice-lifecycle-v1/spec.md) (2026-08-10)

**Next**
- Build the invoice-lifecycle spine (client records · drafting · delivery & payment · status tracking) — [spec](.mochiko/specs/invoice-lifecycle-v1/spec.md) (2026-08-10)

**Later**
- Payment reminders (FEAT-006) — deferred, first after the spine — [feature](.mochiko/features/FEAT-006-payment-reminders.md)
- Resolve data-retention / deletion policy; then revisit void/reissue + partial payments — [BACKLOG](BACKLOG.md#open-questions)

## Standing bets & revisit conditions

- Warn-only complexity enforcement (GI-008) suffices for a solo dev — revisit if code becomes unmaintainable or a second engineer joins.
- No staging environment pre-launch — revisit when the first paying customer is onboarded.

_Last groomed: 2026-08-10 · baseline: 2 open items, 0 sessions, 1 spec (invoice-lifecycle-v1 in-flight)._
