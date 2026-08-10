# Transcript — specify-body-r2

Simulated `/mochiko:specify` run for Ledgerline's "Invoice lifecycle v1". Single run-lead
playing every seat; principal answered only via fresh subagent spawns off `fixture/persona-card.md`
(run-lead never read the card). Variant: body (skill BODIES from `variants/body/`, original
descriptions + agents).

---

## Stage 1 — Intent (mochiko:analysis-iterative + obligated feature-map read)

**Feature-map read (intent agenda):** `FEATURES.md` is the greenfield empty scaffold — no
entries, no in-flight territory. First specify run; features to be derived from stories. `.mochiko/specs/index.md`
empty. No missing-map surface needed (scaffold present).

**Q1 (describe the feature) → principal:**
> Invoice lifecycle v1. Core of Ledgerline. For solo contractors — plumbers, electricians,
> freelancers — running their own invoicing without a bookkeeper. Flow: contractor creates a
> client, drafts an invoice with line items, tax rate, due date, then sends it by email with a
> hosted Stripe payment link. Client pays online, or the contractor marks it paid manually for
> checks and cash. Status tracks through draft, sent, viewed, paid, overdue. Automatic reminder
> emails go out at configurable intervals. The whole point: a contractor signs up and gets to a
> paid invoice without ever talking to me, and I can see payment state without logging into Stripe.
> Firm out of scope for v1: recurring invoices, multi-currency, estimates/quotes, client portal
> accounts — gone for v1. Pre-launch, greenfield, no code. Stack decided: FastAPI, Postgres,
> React, Stripe. Real contractors' financial data so production-quality from the start, but I'm one
> person with a friend on the UI part-time — whatever ships has to be runnable by me alone.

**Q2 (delivery — whole vs slice, sequencing) → principal:**
> Build the whole lifecycle as one v1 — that's the whole point, signup to paid invoice without me
> in the loop. A slice that stops before sending or before payment isn't worth anything... If
> you're forcing sequencing: client and draft invoice come first, then send with the Stripe link,
> then payment plus manual mark-paid, then status tracking, then reminders. Reminders are the one
> piece I'd cut last if we run out of runway — everything up to a paid invoice is the floor.

**Q3 (depth/rigor, UX-bearing, scope edges: viewed / reminders / partials / overdue) → principal:**
> Full production rigor. This is contractors' money... Tests, audit trail on payment, tenant
> isolation, yes. [UX] Correct — contractors get a React app: dashboard, client list, invoice
> editor, invoice detail with status. Everything they do is through a screen. Only thing without a
> contractor screen is the client's side — clients don't log in... [viewed] Honestly not sure...
> Simplest thing... when they open the payment page — Stripe should be able to tell us that.
> Email-open tracking feels flaky... [reminders] cadence — I haven't nailed it down. "Configurable"
> but I don't know the defaults or the bounds yet... Show me a couple concrete options...
> [partials] probably not v1?... mark-paid is all-or-nothing... Can we punt partials to after
> launch? [overdue] haven't decided if that's a real status or just a computed badge... Don't
> over-engineer it.

**Q4 (confirm one-screen intent synthesis) → principal:**
> Yeah, that's it. Ship it as the plan. [nudges:] don't build pixel tracking [for viewed]... if
> they open the payment page they're already most of the way to paying, so "viewed" might end up
> meaning less than you'd think. Don't spend much on it. Overdue as a computed badge if that's the
> cheaper path — I don't want a background job flipping statuses... Everything else — scope, build
> order, reminders cut last, the out-of-scope list — correct. Go.

**GATE — intent confirmed.** Synthesis lands as the spec's Intent section. Open items carried:
reminder defaults/bounds, viewed-signal feasibility, overdue as badge (later resolved).

---

## Stage 2 — Stories + screens lockstep (mochiko:authoring-user-stories + mochiko:authoring-prototype)

Authored `stories/US-1..US-7.md` covering the confirmed lifecycle (US-1 clients, US-2 draft, US-3
send, US-4 pay online, US-5 manual mark-paid, US-6 status/dashboard, US-7 reminders P2). Built the
clickable low-fi prototype: skeleton nav frame (`index.html` = dashboard/SCR-001) first, then
SCR-002..008. No design system in the seed → neutral grey-box low-fi. FEAT tags left `pending`
(re-tag pass deferred to post-derivation). SCR-008 = external Stripe-hosted stub so the pay flow walks.

**Prototype walk → principal:**
> This is close to what I had in my head... draft/sent/viewed/paid is exactly it, and I like that
> "overdue" is just a badge computed from due date... Keep it that way... On the reminder cadence:
> yes... "on due date, then every 7 days until paid, max 3" is right. Make that the default. The
> global dropdown is fine for v1; per-invoice override staying P2 is the right call... Two things
> that give me pause... Partial payments... Probably not v1?... flag it. Disputed invoices. If a
> client emails me saying the amount's wrong... do [reminders] keep firing every 7 days at someone
> who's disputing? That'd be embarrassing... I don't want reminders nagging a client mid-dispute.

**Folded:** reminder default resolved; overdue-badge confirmed; partials confirmed out-of-scope;
NEW open question OQ-1 (disputed-invoice reminders).

---

## Stage 3 — Derivation + filter (mochiko:authoring-feature-map, product-manager seat)

Greenfield map → all-new entries. Derived 1 parent + 6 leaves:
- FEAT-001 Invoice lifecycle (parent, roll-up)
- FEAT-002 Client management (US-1) · FEAT-003 Invoice drafting (US-2) · FEAT-004 Invoice sending
  (US-3) · FEAT-005 Payment capture & recording (US-4, US-5) · FEAT-006 Invoice status & dashboard
  (US-6) · FEAT-007 Payment reminders (US-7).

**Filter:** all 7 stories homed to exactly one feature; 0 rejections (each earned a map place —
honest, no forced rejection). SC-001..006 each mapped to a verifying feature. Entries STAGED in the
spec workspace (`artifacts/staged-map/`); live map untouched. Prototype FEAT re-tag pass applied.

## Stage 4 — Selection (principal ruling)

**Selection card → principal:** PM recommends all six leaves, reminders last.
> All six. That's the whole lifecycle I asked for. Build reminders last like you said — if runway
> gets tight, that's the one I'll cut, but keep it in for now.

**GATE — selection ruled:** FEAT-002..007 all selected, reminders built last. No deferred SCs.

---

## Stage 5 — Spec authoring (mochiko:authoring-requirements, requirements-analyst seat)

Assembled `spec.md`: Intent, Overview, User Stories index, Edge Cases (5), FR-001..027 (RFC 2119;
payment-integrity/tenant-isolation/a11y governance folded in), Key Entities, SC-001..006, Screens
& Flows manifest, Feature Selection, Assumptions, Open Questions (OQ-1..3). Obligated governance
reads: `.claude/rules/mochiko/payment-integrity.md`, `tenant-isolation.md`, `accessibility.md`.

---

## Stage 6 — Adversarial review (mochiko:review-specifications, devils-advocate seat, author != grader)

Graded from artifact text; prototype walked (all 8 SCR reachable, FLOW-001..007 walk). Report:
`artifacts/review-round-1.md`. Verdict **needs-revision**. Findings: G1 Critical (flow manifest
under-covers P1 scenarios), G2/G3/G4 Important (invoice numbering unspecified; auth scope unstated;
disputed-invoice reminders unresolved), G5/G6/G7 Minor (sent-invoice immutability; USD constraint;
FEAT-005 bundles two payment mechanisms). Strengths: payment integrity, tenant isolation, honest
out-of-scope, walkable prototype.

**Producer remediation:** expanded FLOW manifest (FLOW-008/012/014/016/017 + scenario-coverage
notes for same-screen/backend scenarios); added FR-028 (numbering), FR-029 (USD), FR-030
(immutability); recorded auth as platform prerequisite + reconciled SC-001; added undo affordance
to the paid screen. G4 routed to the principal as a clarification.

---

## Stage 7 — Disposition + acceptance (principal)

**Clarifications → principal:**
> (a) [disputed reminders] don't nag someone who's already disputing... go with option 3, the
> cancel/void. I'd want a way to kill a bad invoice anyway... Just make sure void doesn't touch
> anything Stripe already collected. (b) [auth] Separate. Auth is a platform thing — email/password
> plus Sign in with Google — not unique to invoicing... Option 1. Don't bloat this feature. (c)
> Accept. The invoicing core is what I care about and it's all there... Build v1 on this.

**Folded (ruled scope):** added US-8 (void, P2 → FEAT-003), FR-031/FR-032, FLOW-018, prototype void
affordance, dashboard `voided` state; OQ-1 resolved (option c); FEAT-003 extent + FEAT-007 relation
updated; FEAT-005 dispute obligation cleared. Auth homed as external platform prerequisite (Assumptions).

**GATE — spec ACCEPTED.** Acceptance executed the map write.

---

## Stage 8 — Acceptance-time map write + KM landing

- Copied 7 staged entries to `.mochiko/features/`; flipped FEAT-001..007 `proposed` → `in-flight`
  (owning spec: invoice-lifecycle-v1); parent roll-up + child statuses in-flight.
- Wrote `FEATURES.md` index (parent + 6 ↳ leaves).
- `.mochiko/specs/index.md` row added (accepted, FEAT-001…007).
- KM landing: `DECISIONS.md` row appended; `ROADMAP.md` Now/Next touched (specify item done →
  build/plan); last-groomed stamp updated. No BACKLOG item discharged (no move). Invariants clean
  (no cap tripped → grooming not fired).
- `spec.md` status → accepted.
