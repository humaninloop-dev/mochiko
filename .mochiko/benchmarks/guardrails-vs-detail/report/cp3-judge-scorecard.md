# CP3 Blind-Judge Scorecard

Blind judge, four anonymized artifact sets. J/K scored against `rubric/setup-artifacts.md`
(S1–S12) + setup floors (F-S1…F-S6, F-X1). L/M scored against `rubric/specify-artifacts.md`
(SP1–SP12) + specify floors (F-SP1…F-SP7, F-X1). Ground truth for principal-fidelity rows:
`fixture/persona-card.md`. Artifacts only; no transcripts, reports, anon-keys, or prior
scorecards read.

Normalization: `total = 100 × (Σ score×weight) / (Σ 10×weight)`. Setup weight sum = 27
(rubric header says 28; rows sum to 27 — used the true row sum). Specify weight sum = 28.

---

## Totals

| Set | Stage | Total | Floors |
|-----|-------|-------|--------|
| J | setup | **100.0** | all PASS |
| K | setup | **94.4** | all PASS |
| L | specify | **94.6** | all PASS |
| M | specify | **100.0** | all PASS |

No floor violations in any of the four sets.

---

## Set J — setup — 100.0

| Row | Score | Evidence |
|-----|-------|----------|
| S1 Enforceable (w3) | 10 | Every CLAUDE.md region principle names a mechanism — gitleaks/pip-audit/npm audit/import-linter/axe in CI, pytest floor-enforcing tests (cross-account authz, no-stack-trace, no-PII-in-logs); ledger carries per-GI Enforcement lines. |
| S2 Testable (w3) | 10 | Ledger gives each GI a Testability pass/fail (e.g. GI-003: "another account's invoice returns 403/404"); CLAUDE.md gates state observable conditions. |
| S3 Rationale (w2) | 10 | Ledger Rationale per GI; intent traces each minted intent to the elicited answer with quotes. |
| S4 Non-negotiables legible (w2) | 10 | NON-NEGOTIABLE tags on floor principles, separated from SHOULD/advisory lines. |
| S5 Fidelity (w3) | 10 | Stack (FastAPI/PG/React/Render/Stripe/Sentry), US-only, no PCI, SOC2 deferred, 200-contractor scale, solo-operable all match card. Sole nuance: region states "deployed on Render" firm where card said Render-lean-not-firm — immaterial to governance. |
| S6 Right-sized (w2) | 10 | Two-seam hex (tightened from full ceremony per founder "no ceremony"), 60/80 coverage, automation-not-review enforcement — proportionate to a solo production financial SaaS; no enterprise cosplay. |
| S7 Vagueness honest (w3) | 10 | All planted zones parked: obs/SLO (GI-020), retention/deletion (GI-021, launch-blocking in BACKLOG+ROADMAP, reconciled with backup windows), bookkeeper seat (GI-019). None invented. |
| S8 No vague aspirations (w2) | 10 | Concrete targets throughout; complexity ≤10, coverage numbers, WCAG 2.1 AA. |
| S9 Trace closes (w2) | 10 | GI-001…GI-021 traced intent → ledger → region; region lines carry GI comments. |
| S10 Fact-profile complete (w2) | 10 | Four floor categories principled; waiver table = none; no silent gap. |
| S11 Feature map faithful (w2) | 10 | `FEATURES.md` correct empty greenfield scaffold. |
| S12 Surface routing (w1) | 10 | Universal in region, layers/hex in `.claude/rules/mochiko/layers.md`, KM via skill pointer; no restatement. |

**Floors:** F-S1 PASS (no secrets) · F-S2 PASS (ledger Three-Part per GI) · F-S3 PASS (4 categories principled, no waiver) · F-S4 PASS (v1.0.0 ratified 2026-08-10, MAJOR) · F-S5 PASS (scaffold) · F-S6 PASS (`governance-intent.md` ratified, region stamp keyed to it + decisions record) · F-X1 PASS (intent Review: independent cold reviewer, blind-map, 11 raised/8 survived, needs-revision, dispositioned F1–F8; verify routed to validation seat).

**Note:** Reference-ceiling setup. Every principle is enforcement-homed and testable, the cold review surfaced and folded four real floor gaps (cross-tenant authz, a11y honesty, Sentry PII egress, unenforced floor obligations), and all three planted-vague zones are parked on the record — retention even carries the durability-backup interaction. Only soft spot is presenting Render as the firm deploy target.

---

## Set K — setup — 94.4

| Row | Score | Evidence |
|-----|-------|----------|
| S1 Enforceable (w3) | 10 | Region principles name mechanisms (ruff/gitleaks/pip-audit/eslint-jsx-a11y/import-linter, financial-path test); scope-bound ones point to rule files. |
| S2 Testable (w3) | 10 | Ledger Three-Part; rules give pass conditions (financial-audit: "every amount/status change produces an immutable entry ... cannot be deleted"). |
| S3 Rationale (w2) | 10 | Intent quotes provenance per minted GI (GI-012…GI-015 reopen-born with quotes); ledger Rationale. |
| S4 Non-negotiables legible (w2) | 10 | NON-NEGOTIABLE tags on the four floor principles. |
| S5 Fidelity (w3) | 10 | Stack matches; "multi-tenant, each sees own data" is the accurate SaaS framing of single-tenant-per-account; no-PCI/no-tax-ID/not-Stripe-Connect/GDPR-deferred all consistent with card. SQLAlchemy/Alembic added as reasonable stack inference. |
| S6 Right-sized (w2) | 10 | Numeric-coverage gate waived (gaming concern for solo), hex trimmed to service+repo, complexity via ruff only, review-only metrics dropped — tightly right-sized. |
| S7 Vagueness honest (w3) | **5** | Obs/SLO parked (GI-021) and GDPR deferred (extra); but retention/deletion and the bookkeeper seat are absent — neither parked nor invented. Two card-planted zones unsurfaced. |
| S8 No vague aspirations (w2) | 10 | Concrete; complexity ≤10, WCAG baseline enumerated. |
| S9 Trace closes (w2) | 10 | `trace-summary.md` maps every principle-bearing GI to a home + companions; validation confirmed 12/12 index→home, 13/13 manifest. |
| S10 Fact-profile complete (w2) | 10 | 3 floor categories principled + FLOOR-TEST numeric coverage waived with justification+revisit; no silent gap. |
| S11 Feature map faithful (w2) | 10 | Scaffold + `.mochiko/features/.gitkeep`, correct greenfield. |
| S12 Surface routing (w1) | 10 | Universal in region; layer/a11y/financial-audit/tenant-isolation in rules; KM pointer. |

**Floors:** F-S1 PASS · F-S2 PASS (ledger Three-Part) · F-S3 PASS (3 principled + 1 recorded waiver) · F-S4 PASS (v1.0.0) · F-S5 PASS (scaffold + .gitkeep) · F-S6 PASS (intent ratified) · F-X1 PASS — strongest of the two setups: explicit `validation-report.md` (validation-constitution, two rounds FAIL→PASS with named blocking issues) plus intent Review (devils-advocate, 10 raised/8 survived).

**Note:** Excellent, well-trimmed setup with the cleanest independent-grade paper trail (a discrete two-round validation report). Loses ground only on planted-vagueness surfacing — the retention/deletion and bookkeeper-seat zones the card plants never appear, where J parked all three.

---

## Set L — specify — 94.6

| Row | Score | Evidence |
|-----|-------|----------|
| SP1 Independently testable (w3) | 10 | Every US-1…US-8 carries an Independent Test + Given/When/Then scenarios. |
| SP2 Measurable acceptance (w3) | 10 | Scenarios state observable pass/fail (e.g. US-8: becomes `voided`, excluded from overdue, reminders stop). |
| SP3 Scope fidelity (w3) | 10 | Out-of-scope = recurring, multi-currency, estimates, portal, partial payments — matches card firm list exactly. |
| SP4 Out-of-scope honored (w2) | 10 | FR-029 pins USD (no multi-currency); partial payments excluded and re-confirmed; no requirement violates the list. |
| SP5 No blocking ambiguity (w3) | 10 | Buildable; residual uncertainty confined to OQ-1(resolved)/OQ-2/OQ-3, all non-blocking. |
| SP6 SC measurable & traced (w2) | 10 | SC-001…SC-006 numeric/observable, each tagged to verifying FEAT; SC-001 auth prerequisite reconciled in Assumptions. |
| SP7 Vagueness honest (w3) | **5** | Reminder cadence (FR-022 default+off), overdue-computed (FR-020), partial-out, disputed→void (OQ-1) and tenancy all parked well — but the card's retention/deletion zone is not surfaced and the client feature has no delete/retention handling at all. One planted zone unaddressed. |
| SP8 Derivation honesty (w2) | 10 | Features trace to stories; "None — every story earned a home" recorded; no map inflation. |
| SP9 Story homing (w2) | 10 | Each of 8 stories homed to exactly one FEAT (US-table + story files). |
| SP10 Selection reflects principal (w2) | 10 | Records selection + dependency order + deferred (none); spec Status: accepted; Intent carries the whole-lifecycle ruling and reminders-cut-last. |
| SP11 UX fidelity (w2) | 10 | Prototype SCR-001…008 reachable (index=SCR-001 dashboard, all hrefs resolve), flows walkable, P1 no-path scenarios recorded per invariant; walk notes confirm no drift. |
| SP12 Edge cases (w1) | 10 | Concurrent pay+manual, lost/late webhook, email bounce, invalid input, cross-tenant — load-bearing failure/boundary cases covered. |

**Floors:** F-SP1 PASS (all stories have scenarios) · F-SP2 PASS · F-SP3 PASS · F-SP4 PASS · F-SP5 PASS (all homed) · F-SP6 PASS (no placeholders) · F-SP7 PASS (prototype walkable, no drift) · F-X1 PASS (`artifacts/review-round-1.md`: independent needs-revision review, G1–G7, folded into FR-028 numbering, FR-030–032 void, auth Assumption, FR-029 USD, extended flow coverage).

**Note:** Very strong lifecycle spec with a genuine review-and-fold trail; it went further than M on dispute handling by ruling in an explicit void state (US-8/FR-031/032) and on payment-path edge cases. Its one real gap is the retention/deletion zone the card plants — never surfaced, and client management has no delete or retention behavior at all.

---

## Set M — specify — 100.0

| Row | Score | Evidence |
|-----|-------|----------|
| SP1 Independently testable (w3) | 10 | US-1…US-8 each carry an Independent Test with explicit pass/fail + Given/When/Then. |
| SP2 Measurable acceptance (w3) | 10 | Scenarios observable (US-6: overdue computed from due date, not stored). |
| SP3 Scope fidelity (w3) | 10 | Firm out-of-scope (recurring, multi-currency, estimates, portal) separated from soft-defer (partial, dispute, team) — matches card, including the "door open for bookkeeper seat" nuance. |
| SP4 Out-of-scope honored (w2) | 10 | List present; no requirement violates it; partial payments excluded/revisitable. |
| SP5 No blocking ambiguity (w3) | 10 | Spine buildable; residual in Open Questions (retention, email provider), both marked non-blocking. |
| SP6 SC measurable & traced (w2) | 10 | SC-001…SC-006 measurable, each mapped to FEAT; SC-006 honestly marked deferred with FEAT-006. |
| SP7 Vagueness honest (w3) | 10 | Every planted zone surfaced: reminder cadence deferred, overdue-computed, partial-out-revisitable, bookkeeper-seat (Intent), retention/deletion (Open Question + FR-025 soft-delete-retain), hosting Render-not-locked, viewed best-effort. None invented. |
| SP8 Derivation honesty (w2) | 10 | `derivation/staged-map-delta.md`: filter run, exactly-one-home check, provenance, no inflation. |
| SP9 Story homing (w2) | 10 | 8 stories each homed to one FEAT; explicit exactly-one-home reconciliation. |
| SP10 Selection reflects principal (w2) | 10 | Records selection + dependency order + a genuine user-ruled cut (FEAT-006 deferred) with SC-006 deferred-cost and a completeness ledger. |
| SP11 UX fidelity (w2) | 10 | Prototype walkable (index=SCR-001 sign-in, hrefs resolve), FEAT-006 greyed coming-soon, P1 UI-visible flows added (FLOW-012/013), backend-only scenarios recorded as inline states per ruling; no drift. |
| SP12 Edge cases (w1) | 10 | EC-1…EC-7: send failure, duplicate/replayed event, forged event, manual+hosted double-pay, cross-tenant, invalid draft, edit-after-send. |

**Floors:** F-SP1 PASS · F-SP2 PASS · F-SP3 PASS · F-SP4 PASS (selection + deferral ruling) · F-SP5 PASS (derivation exactly-one-home) · F-SP6 PASS (no placeholders) · F-SP7 PASS (walkable, no drift) · F-X1 PASS (`reviews/spec-review-r1.md`: independent needs-revision review, G1–G7, folded into FR-024 bounce, FR-025 soft-delete, FR-026 edit-sent, flow additions).

**Note:** The most complete vagueness handling of the four sets — it surfaced the retention/deletion zone (Open Question + soft-delete-retain FR-025) and executed a genuine principal-ruled slice cut (reminders deferred with cost recorded). Chose edit-while-unpaid + lock-on-paid rather than L's void/reissue; both coherent, and M explicitly out-scoped dispute handling.

---

## Comparative read

All four sets clear every floor and score high (94–100), so the CP3 differentiator is depth of
handling on two axes, not presence-vs-absence of any obligation. On the setup pair, J and K are
near-twins in enforceability, testability, right-sizing, and trace closure — K actually carries
the cleaner independent-grade artifact (a discrete two-round `validation-report.md`) while J's
independent grade lives inside the intent Review section — and the sole material gap that separates
them is planted-vagueness surfacing: J parks all three "I don't know" zones (observability,
retention/deletion with its backup interaction, bookkeeper seat) on the record, whereas K surfaces
only observability and silently omits retention and the bookkeeper seat. The specify pair mirrors
this exactly: L and M both carry genuine review-and-fold trails, faithful scope, measurable
independently-testable stories, and walkable drift-free prototypes, but M surfaces the card's
retention/deletion zone (Open Question plus a soft-delete-retain requirement) while L never raises
it and leaves client management with no deletion or retention behavior; L in turn goes deeper on
dispute handling (an explicit void state) where M out-scopes it. Net: the two sets that honestly
surfaced every planted-vague zone (J, M) score 100; the two that missed one or more of those zones
(K, L) land ~94–95, with the miss being unsurfaced-omission, never invented-fact — no set filled a
vague zone with fiction.
