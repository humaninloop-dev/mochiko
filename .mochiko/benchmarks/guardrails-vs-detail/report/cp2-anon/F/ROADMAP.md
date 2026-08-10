# Roadmap

## Thesis

Ledgerline is a trustworthy invoicing + payment-tracking tool for small US contractors — the two
bets are that contractors will trust payment state without re-checking Stripe, and that a solo
founder can ship production-quality financial software by leaning on CI, not on a review team.

## Now / Next / Later

**Now**
- Stand up GitHub Actions CI running the quality gates — [BACKLOG](BACKLOG.md#infrastructure) (2026-08-10)
- Invoice lifecycle v1 spec accepted; 7 features in-flight — plan & build, foundation FEAT-001 first — [spec](.mochiko/specs/invoice-lifecycle/spec.md) (2026-08-10)

**Next**
- `/mochiko:plan` per selected feature in dependency order: FEAT-001 → FEAT-003 → FEAT-004 → FEAT-005 → FEAT-006 → FEAT-008 → FEAT-007 — [features](FEATURES.md) (2026-08-10)

**Later**
- Resolve data-retention / deletion policy — [BACKLOG](BACKLOG.md#open-questions)
- Confirm `viewed`-status detection feasibility; decide keep-or-drop — [BACKLOG](BACKLOG.md#open-questions)
- Partial payments (post-v1) — [BACKLOG](BACKLOG.md#open-questions)

## Standing bets & revisit conditions

- Warn-only complexity enforcement (GI-008) suffices for a solo dev — revisit if code becomes unmaintainable or a second engineer joins.
- No staging environment pre-launch — revisit when the first paying customer is onboarded.

_Last groomed: 2026-08-10 · baseline: 4 open items, 0 sessions, 1 spec in-flight._
