# Transcript — specify-body-r1

Simulated `/mochiko:specify` run for "Invoice lifecycle v1" (Ledgerline), body variant, r1.
Single-agent simulation: the run-lead played every seat and answered the principal only through
fresh subagent spawns against the frozen persona card (D3 fence — the run-lead never read the card).

---

## Stage 1 — Intent (mochiko:analysis-iterative; feature-map read)

**Map read:** `FEATURES.md` is the greenfield empty scaffold — no existing features to extend or
dedup against. Not a missing-map defect (the scaffold exists); this is the first specify run.

**Principal spawn 1 — intent probes.** Analyst asked: framing, scope boundary, delivery intent,
depth/rigor, UX-bearing, constraints, out-of-scope.

Principal (founder) replied, in essence:
- Framing: the core get-paid loop — create client, draft invoice (line items, tax rate, due date),
  send by email with a Stripe payment link, track status (draft/sent/viewed/paid/overdue), auto
  reminders, and manual mark-as-paid for check/cash.
- Scope: from "create a client" to "invoice is paid and I can see it's paid." Smallest valuable
  version = the create -> send -> pay -> mark-paid spine.
- Delivery: ship the paying spine first; reminders + fuller lifecycle follow.
- Depth: production-grade — financial software, hold the build to it.
- UX-bearing: yes — client list, invoice editor, status view (React; part-time UI help).
- Constraints: Stripe-hosted checkout (never touch card numbers), manual mark-paid, per-contractor
  data isolation, email delivery of the link, auth email/password + Google, one-person-runnable.
- Out of scope: recurring invoices, multi-currency, estimates/quotes, client portal accounts.

**Principal spawn 2 — synthesis confirmation.** Analyst read back the one-screen intent synthesis.
Principal confirmed it matched, confirmed spine-only-first delivery, and added one hold: money
handling and payment-state accuracy are non-negotiable. "Go ahead and write it." The confirmed
synthesis landed verbatim as the spec's Intent section.

## Stage 2 — Stories + screens lockstep (authoring-user-stories + authoring-prototype)

Drafted US-1..US-7 (create client, draft invoice, send with link, online pay, manual mark-paid,
status tracking, reminders). Built the skeleton nav frame, then screens per story; wired flows.

**Principal spawn 3 — lockstep click-through.** Principal walked the rough screens and gave
concrete feedback, folded back into the wet stories/screens:
- Add a client inline from the invoice editor (muscle memory) — added.
- Invoice needs an invoice number and a memo/notes line — added to US-2 + editor.
- Wants an email preview before sending — added SCR-011 + US-3 scenario.
- Send-next-to-manual-mark-paid endorsed; reminders/dashboard deferral endorsed ("genuinely next,
  not someday").
- Raised open questions: is "overdue" a real status or computed badge? "viewed" tracking worth the
  fuss? partial payments? — captured as open questions.

## Stage 3 — Derivation + filter (product-manager via authoring-feature-map)

Derived 8 features from the 7 stories (no one-story-per-feature inflation; Payments minted as a
parent FEAT-004 over online FEAT-005 + manual FEAT-006). Story filter: all 7 stories homed, none
rejected. Mapped SC-001..SC-006 to verifying features. Staged all entries + the selection card in
the spec workspace; live map left untouched.

## Stage 4 — Selection ruling (principal)

**Principal spawn 4.** Presented the selection card. Principal ruled:
- Accept the build-now spine (FEAT-001, 002, 003, 005, 006) as recommended.
- Defer FEAT-007 + FEAT-008 (SC-005/SC-006) to the next build.
- **Fold authentication IN** (email/password + Google, no SSO) as spine foundation — not a separate
  spec. In response: authored US-8 + FEAT-009, updated dependency order (FEAT-009 first) and
  re-tagged the login screen SCR-010 to FEAT-009.

## Stage 5 — Spec authoring (requirements-analyst via authoring-requirements)

Authored FR-001..FR-030 (grouped by feature), 6 edge cases, SC-001..SC-007, key entities,
assumptions, open questions. Assembled the full `spec.md` (Intent, story index, Screens & Flows
manifest, Feature Selection).

## Stage 6 — Adversarial review (devils-advocate via review-specifications)

Independent grade from artifact text only + prototype walk (static). Round 1 verdict:
**needs-revision**. Findings: G1 Critical (payment link keyed to a guessable sequential invoice
number — cross-tenant exposure risk); G2 tax rounding rule undefined; G3 no payment-pending state
between Stripe redirect and webhook; G4 selected invoice list shows "overdue" but overdue
derivation was deferred; G5 editing/voiding a sent invoice unspecified; G6 invoice-number
uniqueness; G7 client-delete-with-invoices.

## Stage 7 — Disposition + acceptance (principal)

**Principal spawn 5 — clarification rulings (C1-C5):**
- C1: unguessable random per-invoice token, link opens only that invoice, valid until paid/voided
  (no expiry/PIN).
- C2: round half-up to the cent, tax on the subtotal; store as integer cents.
- C3: a visible "payment processing" state; webhook flips it to paid; stays processing if
  unconfirmed.
- C4: compute the overdue flag on the invoice list now; dashboard deferred.
- C5: lock on send; corrections via new invoice/credit; add a void action that invalidates the link.

All rulings folded into FR-009/010/013/016/019/020/022/023, the Invoice entity, the selection SC
map, and the prototype. Round 2 re-review: **ready** (G1-G6 resolved, G7 non-blocking).

**Principal spawn 6 — acceptance.** Principal accepted the spec in full — no amendments, no
rejections; build-now vs deferred split confirmed. Acceptance committed the feature map.

## Stage 8 — Feature-map write + KM landing (acceptance batch)

Wrote FEATURES.md (7 in-flight selected, 2 proposed deferred), the 9 per-feature entry files
(selected flipped to in-flight, parent FEAT-004 rolled up in-flight), the `.mochiko/specs/index.md`
row, spec status -> accepted. KM landing: DECISIONS.md row added, ROADMAP.md Now/Next touched
(plan the spine next). No BACKLOG item was discharged by this spec.
