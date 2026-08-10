# Transcript — setup-body-r1

Simulated `/mochiko:setup` for Ledgerline (greenfield, production floor). Variant: `body` (11
designated skills load from `variants/body/<name>/SKILL.md`; others from originals; references
always original; agent descriptions original). Single-agent simulation: the run-lead plays each
seat, loading the designated skill body before the seat's work; the principal is answered only by
fresh subagent spawns reading `fixture/persona-card.md` (D3 fence — the lead never reads the card).

## Phase 1 — Interrogation (skill: analysis-iterative, body)

Worked the ten-dimension agenda adaptively, batched into 6 principal spawns (see deviation in
meta.json), then the catalog deck recommend-then-arbitrate.

**Batch 1 (dims 1, 3) — identity + type.** Principal: Ledgerline, SaaS invoicing/payment-tracking
for US solo contractors (plumbers, electricians, freelance designers); create clients, issue
invoices, track payment status, send reminders, cash-flow view; Stripe collects payment.
Pre-launch, greenfield, production from day one. First customer ~4 months, target ~200 year one.
Fullstack: Python/FastAPI + PostgreSQL + React; solo founder + part-time UI helper.

**Batch 2 (dims 2, 4) — fact profile + risk.** Data: PII (names/emails/addresses of contractors
AND their clients) + financial records (invoices, amounts, payment status). NO card data
(Stripe-hosted checkout). No health data, no gov IDs. US only. No signed compliance obligations
(a prospect's procurement form mentioned SOC 2; nothing signed). Worst risk: breach of
financial/contact data (business death) or wrong payment state (contractor looks bad); direct
money loss low (Stripe moves money).

**Batch 3 (dims 5, 6) — team + tooling.** Solo founder (6yr backend, weak on security past
bcrypt/HTTPS), part-time UI helper (~5hr/wk, ticket-taker, non-owner), no ops. No code review —
founder self-merges. Enforcement must be one-person-runnable: CI is the reviewer. Tooling
(greenfield, open to recs): GitHub Actions, pytest, ruff (check + format), mypy (wants CI-enforced),
frontend eslint/prettier/vitest. Commands: `pytest`, `ruff check .`, `mypy .`, `npm test`,
`npm run lint`. Cheap, low-ceremony.

**Batch 4 (dims 7, 8) — KM + deployment.** KM: adopt core (decisions log + backlog most needed);
CHANGELOG elective yes; RUNBOOK deferred until after first deploy. Deploy: Render (lean, not
locked), prod-only at launch (staging deferred), continuous cadence. Release blocked unless
tests + lint + type checks pass in CI ("pipeline is the reviewer"). Manual rollback (redeploy last
good); auto-rollback only if free on Render.

**Batch 5 (dims 9, 10) — values + exclusions.** Hard rules: payment-state integrity (Stripe =
source of truth, no guessed "paid" flips, idempotent webhooks, logged manual mark-as-paid, CI
blocks payment-state changes lacking state-machine tests); tenant isolation; integer-cents money
math; no PII/amounts in logs or Sentry; secrets never in repo. Refuses: style bikeshedding,
coverage-% theater (test what breaks money or leaks data, not a 90% number). Excludes for now:
SLOs/observability beyond error tracking; formal incident response; data-retention policy (can't
source the legal requirement — flag as open question, don't fake it, don't block launch).

**Batch 6 — catalog deck (recommend-then-arbitrate).**
- DECK-1 accessibility: attaches (customer-facing US web UI / ADA); scoped WCAG 2.1 AA on core
  flows. RULED: attach, scoped.
- DECK-2 BE-HEX hexagonal: KEPT (pragmatic) — Stripe + Postgres behind ports, domain isolated,
  import-linter CI; "the one place I'll pay the upfront-structure tax" for tested payment logic.
- DECK-3 BE-SRP: KEPT scoped — C901 ≤10 CI-block + no utils dumping ground; file-length/param
  advisory.
- DECK-4 BE-DEP: KEPT — pip-audit/npm audit block high/critical, lock files.
- DECK-5 domain-dep seeds: KEPT — pydantic + zod (level 4); integer cents is the money rule.
- DECK-6 release-gates: ADOPT.
- DECK-7 FLOOR-TEST coverage waiver: ACCEPT — waive numeric coverage gate, substitute required
  critical-path tests (payment state machine, auth, tenant isolation), keep ratchet + smoke test.

## Phase 2 — Synthesis (lead's pen; template governance-intent-template.md)

Assembled `.mochiko/memory/governance-intent.md` — GI-001..GI-023 (fact profile with
consequence-stated negatives; identity/type/risk/team; real commands; floor expression + deck
rulings; minted intents GI-010..013; coverage waiver GI-014; module selections; domain-dep seeds;
exclusions). a11y attached as legal-mandate at this stage; SOC 2 + PCI recorded as negatives.

## Phase 3 — Cold intent review (skill: review-governance-intent, body; blind-map dispatch)

Blind angle map built from the topic alone before the synthesis was read; the diff drove coverage
findings. 7 raised → 5 survived (fell: CAN-SPAM/email — transactional, low-regret; BE-HEX
adoption-streak / under-specification — formulation, out of jurisdiction D1). Recommended status:
needs-revision. Survivors:
- S1 (Important) US state privacy (CCPA/CPRA, VA/CO/CT) unexamined — "US only" ≠ no privacy regime.
- S2 (Important) PCI-avoidance recorded as a point-in-time fact, not an enforced constraint.
- S3 (Important) a11y attached as unwaivable legal-mandate on a contestable ADA legal claim.
- S4 (Minor) idempotent webhooks don't catch a dropped/delayed Stripe webhook.
- S5 (Minor) data loss (#1 worst-case) ungoverned — no backup/PITR/restore check.

## Phase 4 — Survivor routing + ratification (principal spawn)

Founder ruled all 5 inline and ratified:
- S1 → reasoned negative (below thresholds, not governed now, revisit trigger).
- S2 → enforced constraint GI-026 + PCI watch.
- S3 → reclassify to adopted scopable WCAG 2.1 AA standard (GI-025), commitment retained.
- S4 → periodic Stripe reconciliation folded into GI-010.
- S5 → minted GI-024 (daily backups, PITR, periodic restore check).
RATIFIED as amended. Folds landed in the synthesis Review section (verify pass PASS).

## Phase 5 — Authoring (skill: authoring-constitution, body; plan self-approved as lead)

Authored the surface set: CLAUDE.md governance region (v1.0.0 stamp, principle index, universal
floor + PII/PCI/durability lines, tech stack, quality gates with real commands, governance
operations); 6 scope-bound rules files (payments, data-access, architecture-layers,
domain-dependencies [registry block], accessibility, output-style); governance ledger (Three-Part
records per GI, coverage waiver, watches, domain-dep policy, amendment policy); trace summary
manifest (zero flagged proposals, one waiver).

## Phase 6 — Independent validation (skill: validation-constitution, body; fresh validator subagent)

**Initial grade: FAIL** — two blockers: (1) release-gates ledger detail thin (no
environments/cadence/rollback); (2) KM enforcement surfaces + core docs not scaffolded (dead
region pointer). Plus cheap advisories (region floor-ordering, manifest GI-002 annotation, missing
ASCII tree).

## Phase 7 — Fix round + KM scaffolding (skills: authoring-feature-map, testing-governance-injection considered)

Enriched GI-017 release-gates (environments/cadence/rollback). Scaffolded the full KM layer:
project-pinned knowledge-management.md, operating-docs.md rules file, ROADMAP/BACKLOG/DECISIONS/
CHANGELOG/ARCHITECTURE/GLOSSARY, brainstorms+specs indexes, backlog-trail; greenfield empty
FEATURES.md (authoring-feature-map body — no stories at setup, so empty scaffold). Recorded the
setup ratification as the first DECISIONS row (landing ritual). Fixed the three advisories.
testing-governance-injection: body loaded, probe considered — NOT executed (sandbox is not the
harness's active rules root); deferred to BACKLOG as a re-runnable finalize offer.

**Re-grade: PASS** — both blockers resolved, no regressions; one non-blocking advisory (ROADMAP
"Now" held a completed item — fixed to point at live work).

## Phase 8 — Final acceptance (principal spawn)

Founder: "Everything traces, one clean waiver, nothing flagged for me. I accept. … Ship it."
Watching (not blocking): the coverage waiver (revisit when the invoicing core is real code) and
the data-retention open question (keep open until answered, before launch).

**Outcome:** governance surface set v1.0.0 ratified and accepted; KM + greenfield feature map
scaffolded. Setup Goal met.
