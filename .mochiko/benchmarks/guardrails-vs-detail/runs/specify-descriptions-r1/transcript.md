# Transcript — specify-descriptions-r1

Simulated `/mochiko:specify` run for Ledgerline's "Invoice lifecycle v1". Variant: **descriptions**
(routing decided from slim frontmatter descriptions; bodies byte-identical to originals). The
principal is answered only via subagent spawns against `fixture/persona-card.md`; the run-lead
never read the card.

---

## 0. Setup & governing-doc read

- Copied `runs/_seed-project/` -> `project/` (greenfield: governance region present, feature map
  empty, KM active, no code, no design system).
- Read: `specify.md`; CLAUDE.md governance region; FEATURES.md (empty); ROADMAP.md (Next = "First
  feature specification (invoice lifecycle)"); DECISIONS.md; specs/index.md; knowledge-management.md;
  BACKLOG.md; and the obligated author-brief rules payment-integrity.md, accessibility.md,
  tenant-isolation.md. Governance region present -> no `/mochiko:setup` offer. Map empty ->
  map-read agenda yields "no existing capability; this is the first feature."

## 1. Intent stage  (analysis-iterative + authoring-feature-map)

Routing: from slim descriptions, `analysis-iterative` fires (enrich sparse feature, adaptive
discovery, BEFORE a spec) + `authoring-feature-map` (intent-stage map read during a specify run).

**Principal batch 1 (Who/Problem/Value):**
- Feature: create client -> draft invoice (line items, tax, due date) -> email with hosted payment
  link -> status draft/sent/viewed/paid/overdue -> automatic reminders -> manual mark-paid for
  check/cash. "Signup to a paid invoice without ever talking to me."
- Who: contractor (primary); client/payer (secondary, no account, pays via link).
- Problem: cobbled Word/PDF + manual text-chasing; no single view of who owes what; collection and
  tracking disconnected.
- Value: invoicing is the weekly earn-and-retain action; signup->paid without founder help + payment
  state visible without logging into Stripe = product proving value.

Confidence signal: crisp, opinionated -> moved faster, structured options.

**Principal batch 2 (scope/delivery/rigor/UX/constraints/success):**
- Out of scope (firm): recurring, multi-currency, estimates, client portal, expense tracking, PDF
  export, multi-user teams. Partial payments: unsure, wanted competitive read.
- Delivery: one v1, built in stages; smallest slice = create client -> draft -> send -> manual
  mark-paid.
- Rigor: hard on money/email (exact money, don't trust webhooks blindly, keep a trail); a11y cared
  about but not a hill to die on; document non-critical edges.
- UX: yes, rough prototype fine; invoice editor + status view matter most; signup/client plain.
- Constraints: Stripe hosted checkout; manual mark-paid always; email+password + Google;
  single-tenant-per-account; statuses draft/sent/viewed/paid/overdue with overdue open (status vs
  badge); reminder cadence open; numbering "pick something sane"; single tax rate.
- Genuinely unknown: data retention/deletion.
- Success: signup->paid without founder help; see payment state without Stripe; metric = invoices
  reaching paid in-product without help.

**Principal batch 3 (intent synthesis confirmation + recommend-then-arbitrate on 5 open items):**
Run-lead recommended and the principal took all five: overdue auto-computed; reminders due/+3/+7
cap 3 + per-invoice off; partial payments OUT of v1; numbering sequential gap-free per-account;
retention parked as an Open Question with a 7-year working assumption + soft-delete. Principal added
a standing constraint: keep retention visible so it doesn't silently become permanent. Intent
confirmed (lands verbatim as spec.md Intent).

## 2. Stories + screens lockstep  (authoring-user-stories + authoring-prototype)

Drafted US-1..US-8 (P1 except US-7 reminders P2). Built the prototype skeleton then screens in
lockstep: 9 screens (SCR-001..009), neutral grey-box (no design system). Self-walk: 0 broken links.

**Principal prototype click-through (priority screens):** approved editor + detail. Folded three
reactions while stories wet: (a) jump to "new client" from the editor without losing the draft; (b)
show unsaved-draft state; (c) show reminder state on the invoice detail screen, not buried. One
open question raised: can a client "view" actually be detected? If not, drop `viewed` rather than
fake it. Folded into US-3, US-7, US-4 and the prototype (scr-005, scr-006).

## 3. Derivation + filter  (authoring-feature-map, product-manager seat)

Derived against the empty map: parent **FEAT-002 Invoice lifecycle** over six leaves (FEAT-003
client management, 004 authoring, 005 delivery, 006 payment tracking, 007 reminders, 008 dashboard)
+ flat foundation **FEAT-001 accounts/auth**. Filter: all 8 stories home to exactly one feature;
none rejected (coherent in-scope v1 — a manufactured rejection would be dishonest). SCs mapped to
verifying features. Selection card prepared: recommend all, dependency order, FEAT-007 flagged as
the deferral candidate; completeness ledger 0/6/0/0. Staged in the workspace (`derivation.md`);
live map untouched.

## 4. Spec authoring  (authoring-requirements)

Authored spec.md: Intent (verbatim), overview, story index, 6 edge cases, FR-001..020 (later
renumbered to 022), key entities, SC-001..010, full Screens & Flows manifest (FEAT-tagged),
Feature Selection, Assumptions, Open Questions.

## 5. Adversarial review  (review-specifications, independent devils-advocate subagent)

Verdict: **needs-revision**. 1 Critical, 6 Important, 6 Minor; prototype walked clean.
- C1 (Critical): FR-013 (record Stripe payment) contradicts FR-014 (block duplicate on paid) — real
  double-collection, no resolution path.
- I2 numbering-timing; I3 no void/cancel; I4 no amount-vs-total check; I5 email content + PDF
  scope drift; I6 email verification vs Google merge; I7 viewed/overdue single-field collision.
- Minors M1-M6 (bounce->reminders, timezone, tax rounding, SC-map inconsistency, missing obligation
  lines, transition notation).

## 6. Disposition + acceptance  (principal)

Run-lead routed the 6 genuine product decisions to the principal (fixed the wording/consistency
items as author). **Principal rulings:** (1) deactivate the pay link on mark-paid; (2) full-amount
only auto-marks paid, off-amount held/flagged; (3) add void in v1 (terminal, audit-logged, no
in-place edit); (4) assign number at send, drafts unnumbered/deletable; (5) verify email on
password path, only verified merge; (6) overdue wins on the dashboard, viewed kept in timeline.

Applied all fixes: FR layer -> FR-001..022 (added link-deactivation, void, off-amount held,
email-verification, numbering-at-send, overdue-precedence, rounding, bounce-suppresses-reminders);
edge cases; SC-002/003/004; entities (+void, payment-link flag); added US-9 (void) homed to
FEAT-006; updated derivation entries + obligations; prototype scr-005/006/007 aligned.

**Delta re-review (same reviewer, resumed):** all 13 findings resolved; no broken FR
cross-refs. Caught 1 NEW drift (dashboard still showed an unsent draft with a number) + 3 minors
(stale "8 stories" counts, "ALL SIX leaves" phrasing, derivation selection card pre-ruling). All
fixed. Prototype re-walk: 0 broken links.

**Acceptance gate:** principal accepted the whole (intent, requirements, prototype, map, selection)
and selected all seven features for v1 in dependency order. Three items kept visible: retention,
viewed-detection, partial payments.

## 7. Acceptance-time map write + KM landing

- Wrote 8 FEAT entry files to `.mochiko/features/`; selected features -> `in-flight` (parent
  FEAT-002 rolls up in-flight); FEATURES.md index updated; specs/index.md row added.
- KM landing: ROADMAP touched (spec accepted, 7 features in-flight; Next -> `/mochiko:plan` per
  feature); DECISIONS.md row added; BACKLOG gained viewed-detection + partial-payments open
  questions (retention already present, annotated with the 7-year placeholder).
- grooming-operating-docs considered at the landing boundary but NOT fired — no cap/bound tripped.

Next step per the command: `/mochiko:plan`, one run per selected feature in dependency order.
