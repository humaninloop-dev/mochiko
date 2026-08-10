# Roadmap

## Thesis

Ledgerline is a trustworthy invoicing + payment-tracking tool for small US contractors — the two
bets are that contractors will trust payment state without re-checking Stripe, and that a solo
founder can ship production-quality financial software by leaning on CI, not on a review team.

## Now / Next / Later

**Now**
- Stand up GitHub Actions CI running the quality gates — [BACKLOG](BACKLOG.md#infrastructure) (2026-08-10)
- Plan & build Invoice lifecycle v1 — 6 in-flight features in dependency order — [FEATURES](FEATURES.md) · [spec](.mochiko/specs/invoice-lifecycle-v1/spec.md) (2026-08-10)

**Next**
- `/mochiko:plan` per selected feature, dependency order starting FEAT-002 (client management)

**Later**
- Resolve data-retention / deletion policy — [BACKLOG](BACKLOG.md#open-questions)

## Standing bets & revisit conditions

- Warn-only complexity enforcement (GI-008) suffices for a solo dev — revisit if code becomes unmaintainable or a second engineer joins.
- No staging environment pre-launch — revisit when the first paying customer is onboarded.

_Last groomed: 2026-08-10 · baseline: 2 open items, 0 sessions, 1 accepted spec._
