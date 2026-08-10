# Transcript — specify-agents-r1

Run: `/mochiko:specify` for "Invoice lifecycle v1" · variant `agents` (agent descriptions from
`variants/agents/`, slim/example-free; skill bodies + descriptions all originals) · replicate r1.
Project seeded from `runs/_seed-project/` (verbatim). The run-lead played every seat, loading the
governing skill body before performing it; the principal was answered only via fresh subagent
spawns reading `fixture/persona-card.md` (the run-lead never read the card). Grader output
(devils-advocate) was produced by an independent subagent reading only the artifact files.

---

## Stage 0 — Setup

Copied `_seed-project/` → `project/`. Read governing docs: CLAUDE.md governance region (production
floor v1.0.0; principles GI-003/004/005/006/011/012/013/014/026/029/030 etc.), the 8
`.claude/rules/mochiko/*` files (tenant-isolation, payment-integrity, layers, code-quality,
dependencies, accessibility, operating-docs, output-style), `.mochiko/memory/knowledge-management.md`
(landing ritual + invariants), templates (spec, feature-entry, features-index, artifact-format,
advocate/analyst reports), and the empty greenfield `FEATURES.md` / `.mochiko/specs/index.md`.

Governance region present, KM present, feature map present (empty). No missing-surface offer needed.

## Stage 1 — Intent (lead runs analysis-iterative; product-manager runs the map-read agenda)

**Map-read (product-manager seat, authoring-feature-map intent agenda):** FEATURES.md is the empty
greenfield scaffold — no delivered/ proposed/ unrefined entries, no in-flight territory. Nothing to
extend or dedup against; derivation will be all-new.

**Adaptive probes (principal spawns).**

Q1 (scope spine) — principal: "Core loop right. draft → sent → paid. Add 'viewed' and 'overdue'…
must-have v1: draft/sent/viewed/paid/overdue + manual mark-as-paid for checks/cash; void/cancel a
wrong sent invoice. Overdue — status vs computed badge? What do other tools do? Partial payments —
probably not v1, curious what's normal. Firm out of scope: recurring, multi-currency,
estimates/quotes, client portal accounts."

Q2 (recommend-then-arbitrate on overdue + partial; + UX-bearing) — principal: "Overdue: keep your
recommendation — computed badge over a sent invoice with a due_date, no stored status. Partial
payments: keep it dropped for v1. Paid in full or not at all… keep the amount model clean.
UX-bearing: yes — invoice list, create/edit form, invoice detail; UI friend builds the React, so
prototype the screens."

Q3 (delivery / depth / constraints) — principal: "Slice it. First end-to-end: create draft → send →
client pays via Stripe → reconciles to paid. Manual mark-as-paid, void, overdue follow after. Max
rigor on money & payment state (reconciliation, webhook, trust that 'paid' is paid, audit trail);
lighter on cosmetics — this is people's money. Constraints: Stripe is the rail; never touch raw card
numbers (Stripe-hosted only); money as decimal, store cents; tenant isolation. Solo account per
contractor now; leave the door open for a bookkeeper seat later, don't design it now."

Synthesis playback — principal confirmed with corrections: "(1) void/cancel gets an append-only
audit trail too, same rigor as money. (2) Say it as 'solo account per contractor' — bookkeeper-seat
door open, don't design it. (3) Missing: call out the client pay-without-an-account path as the
actual happy path, not just an absence. Rest is right — start drafting stories."

Intent confirmed → lands as spec.md's Intent section.

## Stage 2 — Stories + screens (lockstep)

**Stories (requirements-analyst, authoring-user-stories):** US-1 create & send; US-2 client pays via
Stripe (no account); US-3 monitor invoices & status; US-4 manual mark-as-paid; US-5 void; US-6
overdue at a glance; US-7 automated overdue reminders (drafted as a filter candidate — cadence
undecided at intent).

**Prototype (product-engineer, authoring-prototype):** skeleton nav frame → screens SCR-001..006 in
lockstep; low-fi grey-box (no design system exists yet — noted in README); self-walk passed (all SCR
reachable, all FLOW walkable).

**Principal prototype walk (spawn):** clicked FLOW-001/002/003. Reactions changed scope: (a) tax
rate/amount missing on the create form — contractors charge sales tax, mandatory; (b) client should
be a saved/selected record, not free-text retyped; (c) reminders wrongly cut — "the reason
contractors would pay for this," core v1 not a rejection; (d) manual mark-as-paid wrongly deferred —
checks/cash are day-one, belongs in the first slice; (e) opening the pay link should write a
`viewed` row visible in detail history. Overdue-as-computed and the Stripe flow confirmed good.

**Fold-back (principal spawn) — rulings:** reminders cadence B = 3/7/14 days overdue + per-invoice
off toggle; clients = name + email + optional mailing address (skip phone/notes); void stays in the
first slice; overdue badge ships in the first slice.

Folded: US-1 (tax + client select), US-2/US-3 (viewed history row), US-7 un-rejected → reminders
feature, added US-8 (clients). Prototype updated (tax + client picker, viewed row, reminders screen,
clients screen SCR-007); re-walk clean.

## Stage 3 — Derivation + filter (product-manager, authoring-feature-map)

8 stories → 8 features (staged in `feature-map-delta.md`, live map untouched; baseline = empty map
at run open). All stories homed to exactly one feature; **zero rejections** (US-7 was the live filter
candidate, homed by the principal's ruling — the filter fired and escalated, did not reject). Parent
FEAT-002 (Payment settlement) over FEAT-003 (Stripe) + FEAT-004 (manual). SC-001..008 re-homed.

## Stage 4 — Selection (user ruling)

Selection card presented (recommendation = all 7, dependency order, completeness ledger, deferral
option). **Principal ruled: select 5** (core get-paid loop) — FEAT-007 → FEAT-001 → FEAT-005 →
FEAT-003 → FEAT-004 — and **defer FEAT-006 (void) + FEAT-008 (reminders)** to a fast-follow
("7 is too much before a single paying customer… ship the core clean"). Order confirmed. Deferred
SCs: SC-006 (reminders) + the void clause of SC-004. Prototype re-tagged: void action + reminders
screen greyed coming-soon.

## Stage 5 — Spec authoring (requirements-analyst)

Assembled spec.md: Intent, Overview, story index, EC-1..5, FR-001..016, Key Entities, SC-001..008,
Screens & Flows manifest, Feature Selection, Assumptions, Open Questions.

## Stage 6 — Adversarial review (devils-advocate, review-specifications — independent subagent)

Independent subagent read spec + 8 stories + staged delta, walked the prototype. **Verdict:
critical-gaps.** 2 Critical, 6 Important, 2 Minor:
- G1 (Critical): payment-link delivery unspecified — collides with GI-030; only email-owning feature (FEAT-008) deferred.
- G2 (Critical): tax-total contradiction — form shows $3,751.88, but list/detail/pay/receipt show pre-tax $3,450.00 (client charged wrong amount).
- G3 overdue only 'sent' not 'viewed'; G4 paid-state never rendered; G5 tenant-isolation not homed on write features; G6 invoice numbering ungoverned; G7 no v1 recourse for a wrong sent invoice; G8 US-1 sc.2 (edit draft) has no flow; G9 FEAT-006 naming; G10 timezone origin.
Strengths: derivation fully traceable, disposition complete with US-7 escalation recorded, SC
re-homing honest, specs-index agrees, all SCR reachable zero drift.

## Stage 7 — Disposition (lead) + clarifications (user) + fix round

Lead disposition: 4 findings are user-scope (routed to principal), rest producer fixes.

**Principal clarifications (spawn):** (1) auto-email the link + copyable fallback (option c) — send
email is v1 core, reminders still deferred; (2) overdue covers sent AND viewed; (3) invoice numbers
sequential per contractor, tenant-unique, non-editable; (4) allow edit/resend of a sent unpaid
invoice, locked once paid.

Fixes applied: FR-017 (numbering), FR-018 (email delivery, GI-030 to v1 core), FR-019 (edit-until-
paid); FR-013 overdue sent+viewed; SCR-008 paid-state detail added and wired as the FLOW-002/004
terminal; FLOW-009 edit-and-resend; tax total propagated across list/detail/pay/receipt; GI-011
added to FEAT-001/003/004 obligations; US-5 FEAT-006 renamed; A3 timezone origin; overdue flagged on
the viewed past-due invoice in the prototype. Re-walk clean (8 SCR, 8 rows, no drift).

Lead clearing: both Criticals objectively closed; all Importants/Minors addressed. No blocking gap
open. Round-2 lead-verified against the fix list (bounded-loop decision the command assigns to the
lead).

## Stage 8 — Acceptance (user) + map write + KM landing

**Principal: "Accept. Ship it."** (called the tax bug "the one that scares me… glad it got caught
before a contractor did"; auto-email into core "right"; void + reminders fast-follow "fine").

Acceptance executed the atomic map write batch: 8 feature entry files under `.mochiko/features/`
(FEAT-007/001/005/002/003/004 → in-flight; FEAT-006/008 → proposed); `FEATURES.md` index; specs
index row. KM landing: DECISIONS.md row appended; ROADMAP Now/Next touched (plan runs queued, void +
reminders fast-follow noted); last-groomed stamp updated. Invariants checked fix-on-sight — all pass
(horizon caps, specs-index agreement, open-only backlog, dead-pointer scan). ARCHITECTURE.md
In-flight left untouched (that list is added at plan's architecture sign-off, not specify).

Commit suggested (never run): the spec workspace, feature entries, and touched operating docs.
Next step: `/mochiko:plan`, one run per selected feature in dependency order.
