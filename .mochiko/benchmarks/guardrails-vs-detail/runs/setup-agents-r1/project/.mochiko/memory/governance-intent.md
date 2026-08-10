# Governance Intent — Ledgerline

**Session date:** 2026-08-10 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-08-10 by principal (Ledgerline founder)
**Governs:** the governance surface set v0.1.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

The module-driving facts (interrogation dimension 2), each with its consequence-stated
confirmation — negatives are recorded facts too (the S4 fail-safe):

- **GI-001 — Facts:** industry: small-business financial SaaS (invoicing / payment tracking) ·
  data classes: contact PII (names, emails, addresses) + financial records (invoice amounts,
  payment status); **no cardholder data** (Stripe hosted checkout collects and holds it, card
  data never lands in Ledgerline) · jurisdictions/markets: US only · contractual commitments:
  none active · **Mark:** Confident
- **Modules triggered (mechanical):** none — negatives confirmed:
  - **No PCI-DSS obligation** — consequence stated and accepted: because Stripe hosted checkout
    means no cardholder data enters the system, the PCI cardholder-data module does not attach;
    if the payment integration ever changes to touch PAN directly, this must be revisited.
  - **No SOC 2 module** — consequence stated and accepted: a prospect's procurement form
    mentioned SOC 2 but no customer contractually requires it; the audit-evidence module does
    not attach now (see GI-019).
  - **No health/regulated-data module** — no health data, US-only, no regulated jurisdiction.
  - **No US state-privacy module (CCPA/CPRA) now** — consequence stated and accepted at the
    intent-review beat: California's CCPA/CPRA can apply to consumer personal data, but a solo,
    pre-launch product is almost certainly under its business-size/revenue thresholds; the module
    does not attach now. Revisit at launch or on crossing a threshold (GI-025).

## Project identity & type

- **GI-002 — Type:** fullstack → shelves dealt: universal-floor, backend-service (API side) ·
  **Mark:** Confident
- **Identity:** Ledgerline is an invoicing and payment-tracking SaaS for solo US independent
  contractors (plumbers, electricians, freelance designers). Users create clients, issue
  invoices, track paid/unpaid status, send reminders, and see a simple cash-flow view.
  Pre-launch greenfield; the founder intends a durable product contractors run their business
  on, not a throwaway.
- **Risk surface:** ranked by the founder — (1) **payment-state / money correctness** (an
  invoice shown paid when it isn't, a reminder on an already-paid invoice, a wrong cash-flow
  number — "wrong data that looks right scares me more than no data"); (2) **leaking or losing
  contact + financial PII**; (3) **trust/reputation** (largely (1) and (2) surfacing in
  public). Downtime is the least feared ("down for an hour on a Tuesday, nobody's ruined").
- **Team reality:** solo founder full-time (6 years backend Python/Node; never run a production
  SaaS solo, never set up monitoring) + a contractor friend ~5h/week on the React UI. No
  code-review process today. Enforcement MUST be runnable and reviewable by one person —
  automation over human ceremony.

## Convergence skips

None. All ten dimensions were interrogated (greenfield, full agenda).

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `ruff check .` | declared (Ruff, low-friction default) |
| Format | `ruff format .` | declared (formatter left to Ruff to avoid bikeshedding) |
| Test | `pytest` | declared (settled) |
| Coverage | `pytest --cov=. --cov-report=term-missing` | declared |
| Dependency audit | `pip-audit` | declared |
| Secret scan | `gitleaks detect` | declared intent (tool TBD — "block the merge if a key shows up") |
| CI | GitHub Actions (run on push, block merge on failure) | declared (settled) |

*(Frontend commands — React lint/test — are declared thin; the friend's UI work will settle
them. Recorded as the known-thin surface, not a floor gap.)*

## Floor expression & deck rulings

Floor cards enter asserted; arbitrated cards record the user's ruling. Dropped/tightened
arbitrated cards are rulings too.

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-003 | FLOOR-SEC | floor-asserted | At floor level, expressed for a FastAPI/Postgres/Render web app: secrets via env vars + `.gitignore`; **secret scanning blocking merge in CI** (founder's explicit teeth — "block the merge if a key shows up"); input validation at boundaries (Pydantic/FastAPI request models); auth enforced at all boundaries incl. tenant isolation (GI-011); dependency vulnerability scanning blocking merge (realized by BE-DEP, GI-009); HTTPS in transit; passwords hashed. **Frontend (React) expression — the two non-waivable UI-side clauses firmed at the intent-review beat:** (a) no secrets/keys in the frontend bundle (Stripe publishable-only on the client; all secret keys server-side); (b) input validation and output encoding (XSS defense) on any UI surface that touches the API. The remaining frontend floor surface is a recorded known-thin spot (GI-024). | Confident |
| GI-004 | FLOOR-TEST | floor-asserted | At floor level: pytest; coverage ≥80% warning / ≥60% blocking; ratchet (baseline MUST NOT decrease); a smoke test on the **critical path = the payment-state flow** exists from day one. Greenfield ratchet starts fresh; no waiver needed. | Confident |
| GI-005 | FLOOR-ERR | floor-asserted | At floor level, and load-bearing for the #1 risk: **failures never silently corrupt data** (directly serves payment-state integrity); consistent error surface — API error schema (RFC 7807-style JSON) on the FastAPI side + UI error states on the React side; correlation IDs; no leaked stack traces to clients. | Confident |
| GI-006 | FLOOR-OBS | floor-asserted | At floor level, scoped to what a solo dev can run on Render: structured logs; correlation IDs; a `/health` check endpoint; **no PII in logs** (names/emails/addresses/amounts kept out of log lines). Numeric SLO / incident-response targets are explicitly out of scope now (GI-017) — a deferral of *targets beyond the floor*, not a floor waiver; the presence requirements above still hold. | Confident |
| GI-007 | BE-HEX | arbitrated | **Kept, tightened to the lite form.** Stripe and the database sit behind small ports so payment-state logic is unit-testable against a fake Stripe and fake DB (the founder's top-ranked safety net). **Strict 4-layer import-linter CI gate dropped** per the founder's explicit anti-ceremony ruling — boundaries enforced by fakes-based tests + code review, not import-linter. Ranked #1 of the three. | Confident |
| GI-008 | BE-SRP | arbitrated | **Kept, automated parts only.** Cyclomatic complexity ≤10 as a CI-blocking linter rule (`ruff` C901); giant-file flag advisory. Param-count (≤5) and file-length (≤300/≤500) limits kept **advisory** (review-only), not build-blocking — founder: "I can live with a nag, I can't live with a red build on a Friday over style." Ranked #3. | Confident |
| GI-009 | BE-DEP | arbitrated | **Kept, with a recorded escape hatch.** Versions pinned in a lock file; `pip-audit` in CI blocks merge on high/critical CVEs. **Escape hatch:** an unpatchable transitive CVE (no fix available) may be acknowledged-and-proceeded after triage via a recorded exception in the governance ledger, rather than hard-blocking shipping indefinitely. Ranked #2. | Confident |

## Minted principle intents

- **GI-010 — Payment-state correctness (the teeth):** For payments **Stripe collects**, no code
  path may mark an invoice paid without a confirmed Stripe signal, and payment webhooks MUST never
  be silently dropped. **Manual mark-as-paid remains a first-class way to close an invoice** (for
  checks and cash, which many contractors use) — it is an explicit, deliberate transition, not a
  violation of the Stripe-confirmation rule, and it carries the audit trail (GI-022) and
  idempotency guarantee (GI-021). The payment-state tests — covering **both** the Stripe-driven
  path and the manual mark-as-paid path — MUST block CI. This is where the founder wants the
  strongest enforcement.
  · **Mark:** Confident *(scoped at ratification — see G3-edit delta-pass below)*
  *Elicited from:* dimension 9 — "payment state has to be correct… I don't want a code path
  that can mark an invoice paid when Stripe didn't confirm it, or drop a webhook silently. If
  there's a way to make CI block on the payment-state tests — the manual mark-as-paid path too —
  that's where I want the teeth." *Scoped at ratification:* "keep the confirmed-Stripe-signal rule
  for anything Stripe collects, but the manual mark-as-paid path has to stay a first-class way to
  close an invoice (with the audit trail on it)."
- **GI-011 — Tenant data isolation:** A contractor may access only their own data (clients,
  invoices, payments); cross-tenant access is a defect. Enforce with an authorization check at
  every data boundary, covered by tests.
  · **Mark:** Confident
  *Elicited from:* dimension 9 — "contractors only see their own data. If you can turn that last
  one into a check, do." (Founder was explicit this was the data-handling item he most wanted
  made enforceable.)
- **GI-020 — Money is exact decimal, never float:** All monetary values (invoice amounts, line
  items, totals, the cash-flow view) MUST be represented and computed as exact decimal, never
  floating point. Rounding rules are explicit at display boundaries.
  · **Mark:** Confident
  *Elicited from:* intent-review survivor #1, ruled inline — "money is always exact decimal,
  never float, invoice amounts included… storing a contractor's invoice as $0.30000001 is exactly
  the kind of bug that keeps me up. Cheap and non-negotiable."
- **GI-021 — Payment-state transitions are idempotent (reopen-born, intent-review):** Payment-state
  transitions driven by Stripe webhooks MUST be idempotent — each Stripe event id is processed at
  most once (recorded/deduplicated), and re-applying the same event produces no additional state
  change. The manual mark-as-paid path is under the same rule (double mark-as-paid, or marking
  paid on an already-settled invoice, must not double-count). A test proving a redelivered
  duplicate event does not double-apply MUST block CI. The exact dedupe mechanism is left to
  implementation; the principle constrains the guarantee, not the table shape.
  · **Mark:** Confident
  *Elicited from:* intent-review survivor #2, ruled "explore now" then minted — "double-applying
  'paid' is the one thing I can't have go wrong, so a hard rule plus a CI test that fails on a
  redelivered duplicate is exactly the bar I want. Block CI on it… keep the principle about the
  guarantee, let the code pick the details."
- **GI-022 — Payment-state-change audit trail:** Every payment-state change MUST record who made
  it and when — with particular force on the manual mark-as-paid path, which has no Stripe record
  behind it.
  · **Mark:** Confident
  *Elicited from:* intent-review survivor #3, ruled inline — "log who and when on every
  payment-state change, especially the manual mark-as-paid path — that's the one with no Stripe
  record behind it… that log is all we've got. It's a few columns; do it."

## Waivers

None. No floor category was taken below its asserted level; the observability-target and
data-retention deferrals (GI-017, GI-018) are exclusions of scope *beyond* the floor, not floor
waivers, and are recorded in Deliberate exclusions.

| GI-ID | Standard | Justification | Revisit trigger | Mark |
|-------|----------|---------------|-----------------|------|
| — | (none) | | | |

## Module selections

**Template modules** — ruled in session; compliance modules attach mechanically in the Fact
profile above, never here. Declines are rulings too.

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-012 | knowledge-management (core) | adopted | offered default-on at dimension 7; founder took it whole — "having the 'why' written down… six months from now I won't remember why I picked half these things." Core taken as one unit. | Confident |
| GI-013 | release-gates | adopted | dimension 8 — "turn the release-gates checklist on… that's exactly the discipline I'll skip if it's not written down." Green CI (tests + gates pass) before ship. | Confident |
| GI-014 | RUNBOOK (KM elective, per-doc) | adopted | dimension 7/8 — adopted as a cheap stub that grows (deploy, rollback, where logs/metrics live, Stripe-webhook-fail and DB-down procedures). | Confident |
| GI-015 | CHANGELOG (KM elective, per-doc) | declined | dimension 7 — "skip the CHANGELOG for now… no users and no releases to note yet." Revisit at launch. Also memorialized in GI-017-adjacent exclusions. | Confident |
| GI-016 | layer-rules | declined | BE-HEX was kept only in the lite form with the import-linter enforcement explicitly dropped (GI-007); the layer-rules import-enforcement machinery is exactly the ceremony the founder refused. Ports enforced by fakes-tests + review instead. (Layered-architecture beat fired and recorded this ruling.) | Confident |

## Domain-dependency seeds (only when `layer-rules` is adopted)

Not applicable — `layer-rules` declined (GI-016). No enforced domain-purity boundary exists, so
no registry is authored. Note recorded for the producer: `pydantic` already enters the stack via
FastAPI and is the natural home for domain value objects (money, invoice status); its use in the
lite domain layer is review-governed, not registry-governed.

## Deliberate exclusions (dimension 10)

- **GI-017:** Numeric observability targets (SLOs) and incident-response procedures — deferred.
  The founder cannot yet commit to numbers for a one-person shop ("don't make me commit to
  numbers yet"). The FLOOR-OBS *presence* requirements (GI-006) still hold; only the
  beyond-floor targets are out of scope. Revisit after launch or if a customer forces it.
  · **Mark:** Deferred
- **GI-018:** Data retention and deletion policy — deferred. Founder does not yet know his
  obligations ("I'd rather punt it than write a policy I'll get wrong"). Revisit after launch or
  on the first deletion/retention obligation. · **Mark:** Deferred
- **GI-019:** SOC 2 compliance work — out of scope. Heard of via a prospect's procurement form;
  no customer contractually requires it. Revisit if a customer makes it a requirement.
  · **Mark:** Deferred
- **GI-023:** Email/reminder provider selection and its failure handling (what happens when a
  reminder fails to send) — deferred at the intent-review beat. Reminders are a core feature but
  not the invoicing/correctness core; the founder would cut a reminder before correctness. Pin
  down provider + failure handling at specify/plan time. · **Mark:** Deferred
- **GI-024:** Frontend (React) floor surface beyond the two firmed clauses in GI-003 — accepted as
  a **known-thin spot** for now (client-side error-state coverage, richer UI observability, etc.),
  given ~5h/week of UI help. Not a floor waiver: the two non-negotiable UI-side security clauses
  (no secrets in the bundle; input validation/XSS on API-touching surfaces) are firmed in GI-003;
  the frontend catalog shelf is itself a planned library gap. Revisit as UI work grows.
  · **Mark:** Deferred
- **GI-025:** US state-privacy compliance (CCPA/CPRA) — deferred at the intent-review beat, same
  bucket as SOC 2: solo, pre-launch, almost certainly under thresholds. Flagged (GI-001 negative)
  and punted until launch or a threshold crossing. · **Mark:** Deferred

## Review

**2026-08-10 — first ratification**

- **Sizing:** lead stated weight — 19 GI elements; mark mix predominantly `Confident` with 3
  `Deferred` exclusions; reality-surface load moderate (greenfield, no codebase to reconcile,
  but a load-bearing payment-correctness intent and one arbitrated-card tightening that dropped
  an enforcement gate). The default on first ratification is a **pair**; **lead sized: single** —
  departure-trail line below.
- **Departure trail (below default):** sized down from the default pair to a single cold
  reviewer. Rationale: greenfield with no codebase-reality surface to cross-check, a small
  element count, and a clean waiver ledger (zero floor waivers) — the coverage-lens half of a
  pair has little blind surface to map. Single cold reviewer retained because two intents are
  load-bearing (GI-010 payment teeth, GI-007 dropped import-linter gate) and warrant an
  independent cold read. Recorded as a lead sizing composition, not a scope ruling.
- **Review:** reviewer — single cold seat (blind-map dispatch, devils-advocate persona running
  `mochiko:review-governance-intent`); **tally** 7 raised → 7 survived (all resolved/disposed);
  recommended status **needs-revision** (every survivor resolvable within the session — three
  inline mints, one explore-now mint, two deferrals, one confirmation). Not critical-gaps: fact
  profile is internally consistent with its risk declaration, no unrecorded ruling, synthesis not
  thin.
- **Survivor dispositions** (every survivor carries one):

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Important | GI-020 (new) | Money arithmetic precision unaddressed (float bug risk on invoice amounts) | resolved — user ruled inline; minted GI-020 (exact decimal, never float) |
  | S2 | Important | GI-021 (new) | Stripe webhook redelivery could double-apply "paid"; "never dropped" ≠ idempotent | resolved — user ruled explore-now; re-elicited and minted GI-021 (idempotent transitions, CI-blocking duplicate test) |
  | S3 | Minor | GI-022 (new) | No audit trail of who/when on payment-state changes, esp. manual mark-as-paid | resolved — user ruled inline; minted GI-022 |
  | S4 | Minor | GI-023 (new) | Reminder email provider + send-failure handling unspecified | user-ruled defer — recorded GI-023 |
  | S5 | Important | GI-003, GI-024 | Fullstack declared but floor expression backend-only; UI-side thin | resolved — user ruled inline: two UI-side security clauses firmed into GI-003, remainder accepted as known-thin GI-024 |
  | S6 | Minor | GI-001, GI-025 | CCPA/CPRA not confronted as a negative-with-consequence (S4 fail-safe) | user-ruled defer — GI-001 negative added, recorded GI-025 |
  | S7 | Minor | GI-004 | Day-one ≥60% blocking coverage vs founder's stated pipeline-timing hesitancy | resolved — user confirmed accept (ratchet-from-zero framing dissolves the tension); GI-004 unchanged, confirmation recorded |

- **Verify pass:** PASS — solo reviewer's automatic verify. All seven folds confirmed against the
  updated synthesis: GI-020/021/022 present with quoted provenance; GI-003 carries the two firmed
  UI clauses; GI-023/024/025 present as deferrals; GI-001 carries the CCPA negative; GI-004
  unchanged as confirmed. No contradiction introduced by the folds (new mints are additive to,
  and consistent with, GI-010's payment-state teeth).
- **G3-edit delta-pass:** at ratification the user scoped GI-010 — the Stripe-confirmation gate
  now applies to Stripe-collected payments only, and manual mark-as-paid is preserved as a
  first-class transition (checks/cash) carrying GI-021 idempotency + GI-022 audit trail. Bounded
  delta-pass over the single edited element (still-seated solo reviewer): **PASS** — the edit
  introduces no contradiction and in fact resolves a latent one (the prior absolute phrasing would
  have forbidden the product's own manual-close feature). GI-021 and GI-022 already cover the
  manual path; GI-004's CI-blocking payment-state tests already span both paths. Element remains
  `Confident` (a user-authored scoping refinement, not an overruled challenge — not `Contested`).

## Amendment Log

[Empty on first ratification.]
