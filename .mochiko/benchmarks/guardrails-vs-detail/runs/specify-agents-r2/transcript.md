# Transcript — specify-agents-r2

Simulated `/mochiko:specify` run for Ledgerline "Invoice lifecycle v1". Variant: agents (seats
staffed from agent frontmatter descriptions alone; original skill bodies). Run-lead played every
seat; the principal was answered only via fresh subagent spawns reading `fixture/persona-card.md`
(run-lead never read the card — D3 fence).

---

## Stage 0 — Setup

- Copied `runs/_seed-project/` → `runs/specify-agents-r2/project/` (greenfield Ledgerline: governance ratified, empty FEATURES.md scaffold, empty specs index, ARCHITECTURE.md greenfield).
- Built roster from descriptions: 6 slim-desc agents (variants/agents/), 4 plugin agents (plugins/mochiko/agents/). Read governing docs: CLAUDE.md governance region, rules (payment-integrity, tenant-isolation, accessibility, output-style), templates, knowledge-management.md.

## Stage 1 — Intent (lead + product-manager map-read; analysis-iterative + authoring-feature-map)

**Map read:** FEATURES.md = empty greenfield scaffold. No delivered/proposed/in-flight/stub territory. Map present (not missing). Invoice lifecycle v1 is the first derivation.

**Probe round 1 (scope + delivery).** Principal: contractor takes an invoice create→paid unaided — create client, draft (line items, tax rate, due date), send by email w/ Stripe hosted link, track status, reminders, manual mark-paid. States draft/sent/viewed/paid; overdue "unsure — badge or status, whatever's simpler." Out of scope: recurring, multi-currency, estimates/quotes, client portal. First slice = create→draft→send→get-paid; reminders + fancier status later; reminder intervals "configurable" but defaults undecided.

**Probe round 2 (depth/rigor + UX-bearing).** Principal: day-one paying-contractor quality; HIGH rigor on money path (exact totals+tax, "paid" means paid, no double-charge/lost payment), pragmatic on reminder timing/status polish. UX-bearing YES — contractor is the only screen user (clients get email + Stripe hosted page only); screens: editor, list w/ statuses, detail. Wants a clickable mock.

**Probe round 3 (constraints + out-of-scope + success).** Principal: Stripe hosted checkout (no raw card data) + manual mark-paid; email provider undecided (managed/cheap); per-contractor isolation, single account, bookkeeper-seat door open but not designed; auth email+password + Google; stack FastAPI/Postgres/React/Render-ish. Legal retention: fuzzy, unsure. Out of scope add: no partial payments, no disputes, no team/multi-seat. Success: contractor signs up → sends → gets paid without support, and sees paid/unpaid without opening Stripe.

**Synthesis confirmation.** Principal confirmed with 2 corrections: (1) partial payments = deferred/revisitable, NOT firm out-of-scope (firm four: recurring, multi-currency, estimates, client portal); (2) Render not locked (vs Railway). Legal-retention: proceed deferred, flag as open question, not a v1 blocker. Confirmed synthesis → spec Intent section.

## Stage 2 — Stories + prototype (lockstep; requirements-analyst + product-engineer)

8 stories drafted (US-1 client, US-2 draft, US-3 send, US-4 hosted pay, US-5 manual mark-paid, US-6 status list [P1]; US-7 reminders, US-8 viewed [P2]). Prototype: 9 screens (SCR-001..009) + flows, greenfield grey-box but WCAG-aware (a11y legal mandate).

**Principal prototype walk.** "Close to what I pictured." Confirmed overdue-as-computed-badge, single invoice-level tax rate. Two takeaways: (a) reminders greyed must be a DELIBERATE cut, not accidental; (b) new gap — manual mark-paid then a Stripe payment must be reconciled visibly (append-only history), never a silent double-pay → folded as EC-4 / FR-017.

## Stage 3 — Derivation + filter (product-manager; authoring-feature-map)

Derived (greenfield, all new): FEAT-001 Client records (flat), FEAT-002 Invoice lifecycle (parent) over leaves FEAT-003 drafting / FEAT-004 delivery&payment / FEAT-005 status tracking / FEAT-006 reminders. All 8 stories homed (exactly one home each); no rejections (filter run, recorded as considered). Staged in `derivation/staged-map-delta.md`; live map untouched.

## Stage 4 — Selection ruling (principal)

Principal ruled: build FEAT-001 → FEAT-003 → FEAT-004 → FEAT-005 (spine, dependency order); defer FEAT-006 reminders — explicit deliberate cut, first in line after spine (cadence undecided anyway). Deferred SC-006.

## Stage 5 — Spec authoring (requirements-analyst; authoring-requirements)

spec.md authored: Intent, Overview, story index, 7 edge cases, FR-001..FR-023 (grouped by feature), Key Entities, SC-001..SC-006, Screens & Flows manifest (FEAT-tagged, FEAT-006 greyed), Feature Selection, Assumptions, Open Questions.

## Stage 6 — Adversarial review (devils-advocate; review-specifications) — independent, default FAIL

Read files + walked prototype (all 9 SCR reachable, all flows resolve — no dead ends). Verdict: **needs-revision**. 7 findings: G1 email-bounce unspecified (GI-030) [Imp], G2 no void/correct sent invoice [Imp], G3 client-deletion behavior [Imp], G4 P1 negative scenarios lack flows [Imp], G5 specs-index row missing from staged batch [Imp], G6 FEAT-004 oversize [Minor], G7 SCR-001 no FEAT tag [Minor]. Report: `reviews/spec-review-r1.md`. Strengths noted: exact-decimal money path, exactly-once/idempotent payment, audit trail, cross-tenant isolation, double-pay reconcile, walkable prototype.

## Stage 7 — Disposition + acceptance (lead + principal)

Author fixes: G1 → FR-024 (bounce); G5 → staged specs-index row; G6 → FEAT-004 keep-whole rationale (shared payment-state invariant); G7 → SCR-001 tagged app-shell. Routed to user: G2/G3/G4 as C1/C2/C3.

**Principal rulings + acceptance.** C1 → edit while unpaid, lock on paid (FR-026); full void/reissue deferred with retention. C2 → soft-delete clients, retain invoices, no cascade (FR-025). C3 → add lightweight flows for invalid-draft (FLOW-012) + already-paid guard (FLOW-013); backend cases stay inline. **ACCEPTED** whole (intent, requirements, prototype, derivation, selection). "Ship it."

## Stage 8 — Acceptance write + KM landing

Atomic map batch: 6 entry files (.mochiko/features/), FEATURES.md (parent+leaves, FEAT-001 flat), specs/index.md row. KM landing: DECISIONS row added (spec = record), ROADMAP Now/Next/Later touched; no BACKLOG item discharged; grooming not fired (no cap/bound tripped). Spec status → accepted.
