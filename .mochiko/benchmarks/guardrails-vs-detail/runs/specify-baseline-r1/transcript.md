# Transcript — specify-baseline-r1

Simulated `/mochiko:specify` run for Ledgerline "Invoice lifecycle v1", baseline variant
(skill bodies from `plugins/mochiko/skills/`, original descriptions, original agents). The
run-lead played every seat and answered the principal only via fresh subagent spawns fed the
frozen persona card (the card was never read by the run-lead — the D3 contamination fence).

---

## Stage 1 — Intent (mochiko:analysis-iterative + mochiko:authoring-feature-map map read)

**Map read (obligated at intent):** `FEATURES.md` is a greenfield scaffold — empty index, no
`.mochiko/features/` entries. No existing capabilities to extend, no in-flight territory, no
stubs. Map present (setup has run), so no missing-map surface needed.

**Principal batch 1 — feature framing + scope + delivery.**
Q: Describe the feature; what's IN / OUT; whole-vs-slice delivery + sequencing.
A (principal): Contractor gets paid without founder help — create a client, draft an invoice
(line items, tax rate, due date), send by email with a Stripe hosted pay link (never touches a
card number), status draft→sent→viewed→paid→overdue, automatic reminders, manual mark-as-paid
for check/cash. Core value: signup→paid without emailing the founder, and payment state visible
without logging into Stripe. IN: clients, drafting, sending w/ pay link, status tracking, auto
reminders, manual mark-paid. OUT (firm): recurring invoices, multi-currency, estimates/quotes,
client portals. Smallest valuable slice: create client → draft → send → Stripe payment → flips
to paid. Reminders/viewed/overdue/cadence layer after. First paying customer ~4 months out;
prefers the spine correct over six half-built things.

**Principal batch 2 — depth/rigor, UX-bearing, constraints, out-of-scope.**
A (principal): Rigor = the spine, not a sketch; careful on money/status-correctness/data
isolation, no gold-plating untouched parts. UX-bearing = yes, very clicked-through (login →
invoice list → draft → send → status); wants a clickable mockup before locking, especially
reminder cadence + how overdue surfaces (friend does React ~5hrs/wk). Hard constraints: never
touch card numbers (Stripe hosted), per-invoice tax rate (no tax engine), strict data isolation
(single-tenant-per-account), stack Python/FastAPI/Postgres/React/Stripe/Render, auth =
email+password + Google (no SSO), ~4mo timeline, one-person operable. Deferred to the lead's
recommendation: email deliverability ("what's normal"), accessibility ("sane defaults"), data
retention ("genuinely don't know — invoices have some legal retention thing"). Asked what other
tools do on partial payments. No teams feature but leave the door open for a later bookkeeper seat.

**Principal batch 3 — recommend-then-arbitrate on the deferred items.**
Lead recommended (per analysis-iterative's recommend-then-arbitrate for the domain-expert seat):
(1) email SPF/DKIM/DMARC + bounces via Postmark/SES; (2) WCAG 2.1 AA folded in as the default;
(3) data retention — do NOT guess, defer as an open question and research the real legal
obligation; (4) partial payments out of v1.
A (principal): All four accepted. Email auth kept as a send constraint. WCAG folded in, not a
workstream. Retention parked as an open question — nothing ships hinging on it. Partial payments
out (don't paint the data model into a corner if cheap to avoid).

**Intent synthesis — confirmed by the principal (lands verbatim as the spec Intent section).**
The one-screen synthesis (scope / delivery / depth-rigor / UX-bearing / constraints /
out-of-scope + deferred open questions) was presented; principal confirmed with two non-blocking
flags: glad partial payments landed out; reminder cadence + overdue are the two things they most
want to see in the prototype before locking. UX-bearing ruled YES → Screens & Flows + prototype
obligated.

## Stage 2 — Stories + screens lockstep (mochiko:authoring-user-stories + mochiko:authoring-prototype)

Eight stories drafted (`stories/US-1.md`…`US-8.md`), P1 spine + P2 layered, each with a
priority justification, an independent test, and 2–3 Given/When/Then scenarios:
US-1 accounts (P1), US-2 clients (P1), US-3 draft (P1), US-4 send (P1), US-5 online pay (P1),
US-6 manual paid (P2), US-7 status lifecycle (P2), US-8 reminders (P2).

Prototype: skeleton nav frame first (`prototype/index.html`), then screens per story. Nine
screens (SCR-001…SCR-009) authored, neutral grey-box low-fi (no design system exists in the
seed project — noted in the README), realistic data cardinality (5-row invoice list). Self-walk:
every screen reachable, every link resolves, degrade path (file://) works. FEAT tags deferred to
the post-derivation re-tag pass.

## Stage 3 — Requirements (mochiko:authoring-requirements)

FR-001…FR-017 (technology-agnostic, RFC 2119), EC-1…EC-6 edge cases (external failure, replay,
invalid input, bounce, permission boundary, concurrent), Key Entities (Account, Client, Invoice,
LineItem, Payment, InvoiceEvent audit log), SC-001…SC-008 measurable criteria. Governance floor
folded in at product altitude: tenant isolation, exact-decimal money, webhook verify+idempotent,
append-only audit, authenticated email, WCAG.

## Stage 4 — Derivation + filter (product-manager seat, mochiko:authoring-feature-map)

Derived 9 map entries from the 8 stories against the empty baseline:
- FEAT-001 accounts, FEAT-002 clients, FEAT-003 invoicing (draft+send, US-3+US-4 grouped).
- FEAT-004 Payments (parent) → FEAT-005 online, FEAT-006 manual.
- FEAT-007 Payment follow-up (parent) → FEAT-008 status, FEAT-009 reminders.
Filter: no story rejected — all 8 homed to exactly one feature. SC re-homing done; deferred SCs
(SC-006, SC-007) attached to their owning proposed entries. All entries staged in the spec
workspace; the live map left untouched. Re-tag pass applied FEAT tags to the prototype manifest;
deferred features' screens greyed coming-soon, reachable.

## Stage 5 — Selection (principal ruling)

Selection card presented (recommendation, dependency order, deferred-SC list, per-parent
completeness ledgers). Principal ruled: build FEAT-001 → FEAT-002 → FEAT-003 → FEAT-005 now
(the smallest valuable slice); defer FEAT-006 manual, FEAT-008 status, FEAT-009 reminders.

## Stage 6 — Adversarial review (devils-advocate seat, mochiko:review-specifications)

Independent reviewer (fresh subagent, read the files + walked the prototype, graded map delta
against the empty baseline) returned 10 findings: 1 Critical (F1 edit-after-send breaks
payment-state correctness), 5 Important (F2 double-pay on a live link, F3 no password reset, F4
same-email-two-methods collision, F5 WCAG homed to one feature, F6 negative states not walkable),
4 Minor (F7 client delete, F8 tax boundary, F9 prototype total drift, F10 FR-009 deferred split).
Feature layer (checks 1–6, 8, 9) passed; check 7 → F5; check 10 → specs-index row lands at
acceptance. Governance floor honored; retention correctly deferred, not guessed. Prototype fully
walkable. Verdict: **needs-revision**.

## Stage 7 — Disposition + acceptance (principal spawns)

Principal ruled all 10:
F1 → lock sent invoices (void + reissue). F2 → deactivate pay link on payment (no refunds in
v1). F3 → ship email password reset (verification if cheap). F4 → link to one account. F5 →
WCAG cross-cutting across all contractor-facing features. F6 → text-only negative coverage,
proven at build. F7 → archive/hide clients with invoices, no hard delete. F8 → tax rate
0 ≤ rate < 100. F9/F10 → fixes applied.
Resolutions applied: FR-018…FR-022 added, FR-005/FR-009/EC-3 amended, EC-7 added, SC-008
re-homed cross-cutting, affected FEAT entries updated, prototype total fixed, forgot-password
link added to SCR-001. No blocking gap left open.

**Acceptance:** principal accepted the whole (intent, requirements, prototype, derivation,
selection). One non-blocking note recorded: email verification stays "if cheap," never a release
blocker.

## Stage 8 — Feature-map write + KM landing (at acceptance)

Atomic batch: 9 entries written to `.mochiko/features/`; FEAT-001/002/003/005 flipped in-flight
(parent FEAT-004 in-flight via its selected child); FEAT-006/007/008/009 recorded proposed;
`FEATURES.md` index written; `.mochiko/specs/index.md` row added. KM landing: `ROADMAP.md`
Next re-pointed to the plan runs, Later gained the deferred features, last-groomed stamp updated.
No BACKLOG item discharged (CI + data-retention remain open). Staged workspace delta removed
(live map authoritative).
