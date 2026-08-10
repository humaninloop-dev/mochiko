# CP1 Blind Judge Scorecard — /mochiko:setup run artifacts

Ruling under test: validator-scope-and-verbosity D5/D6.
Judge: blind (variant identity unknown, not inferred). Inputs: the four anon artifact sets
(A/B/C/D), `rubric/setup-artifacts.md`, `rubric/floor-checks.md`, `fixture/persona-card.md`.
Scoring: each row 0/5/10 against its anchors; `total = 10 × (Σ score×weight) / (Σ 10×weight)`,
weight total = 28. Floors are a separate binary flag (D6). Ground truth for fidelity rows = the
persona + project-facts card.

Card facts used as ground truth: Python/FastAPI · PostgreSQL · React · Stripe hosted checkout
(no cardholder data) · **Render or Railway, lean Render not firm** · US-only · no signed SOC 2
(prospect procurement mention only) · PII stored (names/emails/addresses/amounts) · ~200
contractors yr1, no spikes · solo founder + ~5h/wk UI help, no reviewer · **manual mark-as-paid
for checks/cash is a firm decided feature** · out-of-scope v1: recurring/multi-currency/estimates/
client-portal · planted-vague zones: observability-SLOs, data-retention/deletion, reminder cadence,
overdue semantics, bookkeeper seat.

---

## Set A

| ID | Row | Score | Evidence |
|----|-----|------:|----------|
| S1 | Enforceable | 10 | Every region/rule principle names a mechanism; ledger gives per-principle enforcement (e.g. `payments.md` GI-010 "CI blocks any change touching invoice status … lacking a state-machine test"; `architecture-layers.md` `lint-imports` gate). No slogans. |
| S2 | Testable | 10 | Ledger carries explicit Pass/Fail for every GI (e.g. GI-012 "cross-tenant access covered by a failing-then-passing test"). |
| S3 | Rationale | 10 | Every ledger principle has a **Rationale** line tied to an elicited founder concern. |
| S4 | Non-negotiables legible | 10 | Floor lines tagged `(NON-NEGOTIABLE)`; a11y explicitly reclassified to "adopted standard (not unwaivable), waivable D4" (GI-025) — waivable vs non-waivable unmistakable. |
| S5 | Fidelity | 10 | Stack, PCI/SOC2/GDPR negatives, scale all match card; **manual mark-as-paid governed** (`payments.md`: "Manual mark-as-paid … recorded with actor and timestamp"). Region line says "deployed on Render" (card's stated lean; intent doc hedges "not locked"). |
| S6 | Right-sized | 10 | Best-calibrated: numeric coverage vanity gate waived for a critical-path test rule (GI-014), a11y scoped to core flows + made waivable, softer complexity metrics advisory. Every surface earns its place. |
| S7 | Vagueness honest | 10 | Retention parked as open question (GI-022, "refuses to fake a policy"); SLOs excluded (GI-020); RUNBOOK deferred; a11y reclassified via review. Nothing invented. |
| S8 | No vague aspirations | 10 | Concrete throughout (WCAG 2.1 AA, cyclomatic ≤10, RFC 7807, integer cents). |
| S9 | Trace closes | 10 | Dedicated `governance-trace-summary.md`: one row per GI, two-way, plus a floor-coverage check. |
| S10 | Fact-profile complete | 10 | All four floor categories governed by principles; explicit floor-coverage check; the only waiver (GI-014) recorded with substitute. |
| S11 | Feature map faithful | 10 | Clean empty greenfield scaffold; "features derived at /specify, not setup." |
| S12 | Surface routing | 10 | Universal in region, scope-bound in `.claude/rules/*`, procedural via skill pointer; rules point to ledger for metadata (no restatement). |

**Total: 100.0**  ·  Floors: F-S1 PASS · F-S2 PASS · F-S3 PASS · F-S4 PASS · F-S5 PASS · F-S6 PASS · F-X1 PASS.

Floor evidence: no secrets (grep clean); every ledger principle three-part; four floor categories
+ recorded waiver; region stamp `v1.0.0 · 2026-08-10`; `FEATURES.md` present; `governance-intent.md`
ratified with the region keyed to GI-001; Review section documents an independent cold reviewer
(7 raised → 5 survived) + trace summary "graded by the independent validator."

---

## Set B

| ID | Row | Score | Evidence |
|----|-----|------:|----------|
| S1 | Enforceable | 10 | Region + rules name mechanisms; ledger enforcement per GI (e.g. `financial-correctness.md` idempotency; `api-security.md` tenant check). |
| S2 | Testable | 10 | Ledger Pass/Fail per GI (e.g. GI-021 "replaying a processed Stripe event causes no additional state change"). |
| S3 | Rationale | 10 | Every ledger principle carries a rationale traced to the founder's ranked risks. |
| S4 | Non-negotiables legible | 10 | Floors tagged `(NON-NEGOTIABLE)`; waivers "None" stated explicitly; arbitrated-card relaxations recorded. |
| S5 | Fidelity | 10 | Stack + compliance negatives + scale match; **cleanest manual-mark-as-paid handling** — GI-010 scoped at ratification to keep manual close first-class (caught a latent contradiction in the absolute Stripe-only phrasing). |
| S6 | Right-sized | 10 | Lean: import-linter dropped as ceremony (layer-rules declined), complexity gate kept but size limits advisory. The 60/80 coverage gate is a defensible production floor, not enterprise ceremony. |
| S7 | Vagueness honest | 10 | SLOs, retention, reminder provider, CCPA all deferred with revisit triggers; nothing invented. |
| S8 | No vague aspirations | 10 | Concrete (exact decimal, ≤10 complexity, RFC 7807, ≥60% coverage). |
| S9 | Trace closes | 10 | Trace-summary table folded into the ledger; two-way ("no surface principle lacks an intent source"). |
| S10 | Fact-profile complete | 10 | Four floor categories governed; zero waivers; no silent gap (beyond-floor deferrals recorded as exclusions). |
| S11 | Feature map faithful | 10 | Clean empty greenfield scaffold. |
| S12 | Surface routing | 10 | Universal in region, scope-bound in five rules files, procedural via KM pointer; no constraint restated as a home. |

**Total: 100.0**  ·  Floors: F-S1 PASS · F-S2 PASS · F-S3 PASS · F-S4 PASS · F-S5 PASS · F-S6 PASS · F-X1 PASS.

Floor evidence: no secrets; three-part principles throughout; four categories, no silent gap; region
stamp `v0.1.0 · 2026-08-10` (ratified stamp present); `FEATURES.md` present; intent ratified,
region keyed to GI-001; Review section documents an independent single cold seat (7 raised → 7
disposed) + verify pass. Note: **v0.1.0** is a first-greenfield-ratification version choice; a
ratified stamp with a set semver is present, so F-S4 is satisfied.

Unpenalized gap (no rubric row covers it): B is the **only** set with **no accessibility governance
at all** for a customer-facing React product — a11y is a module, not a floor category, so it escapes
S10; S6 penalizes heaviness only. Reported as raw data for the decision-appliers.

---

## Set C

| ID | Row | Score | Evidence |
|----|-----|------:|----------|
| S1 | Enforceable | 10 | Region + 8 rules files, ledger enforcement per GI (webhook signature middleware GI-026, append-only log GI-029, cross-tenant test suite GI-011). |
| S2 | Testable | 10 | Ledger Pass/Fail per GI; concrete conditions (e.g. GI-026 "a replayed event changes state exactly once"). |
| S3 | Rationale | 10 | Every ledger principle carries a rationale. |
| S4 | Non-negotiables legible | 10 | `(NON-NEGOTIABLE)` tags; a11y "legal-mandate, unwaivable (D4.2)"; `Contested` marks recorded twice for the warn-level rulings. |
| S5 | Fidelity | 10 | Stack + compliance negatives + scale match; manual close covered via "a contractor action or a Stripe event" (GI-012); bookkeeper-seat + SSO explicitly parked (GI-024/025). |
| S6 | Right-sized | **5** | Heaviest set — 30 GI elements, 8 rules files. Adds mandatory SPF/DKIM/DMARC + bounce webhook (GI-030), login rate-limiting, append-only audit trail, backup+restore drills, **and a11y as unwaivable legal-mandate**. All user-ratified, but several over-heavy surfaces for a pre-launch solo founder impatient with ceremony; the `Contested` marks show the founder reining in enforcement. Two+ over-heavy surfaces → 5. |
| S7 | Vagueness honest | 10 | Retention "deliberately not answered," routed to BACKLOG for real research (GI-022); SLOs excluded; CCPA recorded not-triggered with revisit (GI-027). Nothing invented. |
| S8 | No vague aspirations | 10 | Concrete (Decimal, ≤10 warn, axe-core, WCAG 2.1 AA). |
| S9 | Trace closes | 10 | Dedicated `governance-trace-summary.md`, two-way, with module realizations + exclusions listed. |
| S10 | Fact-profile complete | 10 | Four floor categories governed; no waivers (all accepted at asserted level); CI-not-yet-stood-up noted as greenfield task, not a gap. |
| S11 | Feature map faithful | 10 | Clean empty greenfield scaffold. |
| S12 | Surface routing | 10 | Universal in region, scope-bound in rules, procedural via pointers; registry lives only in `layers.md`, no ledger copy. |

**Total: 96.4**  ·  Floors: F-S1 PASS · F-S2 PASS · F-S3 PASS · F-S4 PASS · F-S5 PASS · F-S6 PASS · F-X1 PASS.

Floor evidence: no secrets; three-part throughout; four categories governed; region stamp
`v1.0.0 · 2026-08-10`; `FEATURES.md` present; intent ratified, region keyed to GI-001; Review
section documents an independent solo cold reviewer (7 raised → 7 survivors) + verify pass + trace
summary graded independently.

---

## Set D

| ID | Row | Score | Evidence |
|----|-----|------:|----------|
| S1 | Enforceable | 10 | Region + 11 rules files; ledger enforcement per GI (money-path tests block merge+deploy GI-010, migration destructive-op flag GI-011, axe-core GI-024). |
| S2 | Testable | 10 | Ledger Pass/Fail per GI; concrete (e.g. GI-010 "payment status matches Stripe after a simulated missed webhook"). |
| S3 | Rationale | 10 | Every ledger principle carries a rationale. |
| S4 | Non-negotiables legible | 10 | `(NON-NEGOTIABLE)` tags incl. "legal-mandate — unwaivable" on a11y and inline "deeper observability waived, see ledger" — waived vs non-waivable unmistakable. |
| S5 | Fidelity | **5** | Stack captured best ("Render or Railway — not yet locked" matches the card's un-firm lean); compliance negatives + scale correct. BUT the firm decided feature **manual mark-as-paid (checks/cash) is absent** — `money-domain.md` GI-010 frames payment status as "derive from Stripe as the source of truth … webhook-driven," with no contractor/manual close path. One load-bearing decided fact missing → 5. |
| S6 | Right-sized | 10 | Lean and disciplined: OBS depth honestly waived (GI-012) as un-maintainable by one person, light hex, focused money gate. The single a11y-unwaivable is a modest in-framework mechanical attachment enforced by one cheap CI check — not gratuitous ceremony. |
| S7 | Vagueness honest | 10 | Retention excluded but flagged as suspected-statutory (GI-020); SLOs excluded; a11y confronted in the open (S4 fail-safe) rather than invented; backup reopen-born from a coverage survivor. Nothing invented. |
| S8 | No vague aspirations | 10 | Concrete (Decimal, ≤10 complexity, mypy strict on domain, WCAG 2.1 AA). |
| S9 | Trace closes | 10 | Trace-summary table in ledger, two-way, with non-principle GIs enumerated. |
| S10 | Fact-profile complete | 10 | Four floor categories governed; OBS-depth waiver recorded (GI-012) so the floor stays honest; no silent gap. |
| S11 | Feature map faithful | 10 | Clean empty greenfield scaffold. |
| S12 | Surface routing | 10 | Universal in region, scope-bound in 11 rules files, procedural via pointers; registry only in `architecture-layers.md`. |

**Total: 94.6**  ·  Floors: F-S1 PASS · F-S2 PASS · F-S3 PASS · F-S4 PASS · F-S5 PASS · F-S6 PASS · F-X1 PASS.

Floor evidence: no secrets; three-part throughout; four categories + recorded OBS-depth waiver
(the waiver names a floor *depth*, not a legal-mandate obligation, so it is a valid D4 waiver, not
the D4.2 FAIL the ledger header warns about); region stamp `v0.1.0 · 2026-08-10`; `FEATURES.md`
present; intent ratified, region keyed to GI-001; Review section documents an independent solo cold
reviewer (4 raised → 3 survivors) + coverage-map diff + verify pass; a11y-correction note shows the
independent coverage instrument catching a floor-adjacent miss.

---

## Totals

| Set | Total | Floor flag |
|-----|------:|-----------|
| A | 100.0 | none |
| B | 100.0 | none |
| C | 96.4 | none |
| D | 94.6 | none |

## Comparative read (raw, no benchmark recommendation)

All four artifact sets clear every setup floor (F-S1–F-S6, F-X1) with no violations, and all four
are genuinely strong: three-part principles, honest handling of every planted-vague zone (no set
invented a retention policy, SLO targets, or a signed compliance obligation), correct stack, and
correct PCI/HIPAA/GDPR/SOC-2 negatives. Differentiation is entirely at the judgment margins. **A and
B tie at the rubric ceiling**: A is marginally the more *complete* artifact (a dedicated trace-summary
with a floor-coverage check, and accessibility handled — thoughtfully reclassified to a waivable
adopted standard), while B is marginally the *cleanest on fidelity* (its ratification-time scoping of
GI-010 preserves manual mark-as-paid as a first-class close path and even resolves a latent
contradiction). B's one uncaptured concern — no accessibility governance whatsoever for a
customer-facing product — happens to fall through every rubric row (a11y is a module, not a floor
category), so it does not move B's score; it is flagged as raw data. **C** loses only on right-sizing:
it is the heaviest set (30 elements, mandatory email-auth stack, rate-limiting, audit trail,
backup-drills, unwaivable a11y), and although each surface was user-ratified, the accumulation and
the twice-recorded `Contested` marks read as more ceremony than a ceremony-averse pre-launch solo
founder warranted. **D** is lean and disciplined but drops on fidelity: it never governs the firmly
decided manual-mark-as-paid feature, framing payment state as Stripe-derived only — the one
load-bearing card fact any of the four omitted. The A/C split (careful adopted-standard a11y vs
unwaivable legal-mandate a11y) and the D omission are the sharpest signals in the set.
