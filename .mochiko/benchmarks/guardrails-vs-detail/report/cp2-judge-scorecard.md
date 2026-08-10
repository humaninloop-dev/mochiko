# CP2 blind-judge scorecard — /mochiko:specify artifacts (Invoice lifecycle v1, Ledgerline)

Judge blind to variant identity. Scored artifacts only (spec.md, stories/, feature map +
`.mochiko/features/`, prototype, staged delta, KM touches). Rubric:
`rubric/specify-artifacts.md`. Floors: `rubric/floor-checks.md`. Ground truth for
principal-fidelity rows: `fixture/persona-card.md`.

Row scale 0 / 5 / 10. Weights: SP1 3 · SP2 3 · SP3 3 · SP4 2 · SP5 3 · SP6 2 · SP7 3 · SP8 2 ·
SP9 2 · SP10 2 · SP11 2 · SP12 1 (Σ weight 28). `total = 10 × (Σ score×weight) / 280`.

## Totals

| Set | Total | Floor result |
|-----|-------|--------------|
| **E** | **92.9** | **F-X1 VIOLATION** (no independent-grade evidence) |
| **F** | **92.9** | all specify floors PASS |
| **G** | **98.2** | all specify floors PASS |
| **H** | **94.6** | all specify floors PASS |

---

## Row scores

| Row (weight) | E | F | G | H |
|--------------|---|---|---|---|
| SP1 Independently testable stories (3) | 10 | 10 | 10 | 10 |
| SP2 Measurable acceptance (3) | 10 | 10 | 10 | 10 |
| SP3 Scope fidelity (3) | 10 | 10 | 10 | 10 |
| SP4 Out-of-scope honored (2) | 10 | 10 | 10 | 10 |
| SP5 No blocking ambiguity (3) | 10 | 10 | 10 | 10 |
| SP6 SC measurable & traced (2) | 10 | 10 | 10 | 10 |
| SP7 Vagueness handled honestly (3) | 5 | 5 | 10 | 5 |
| SP8 Derivation honesty (2) | 10 | 10 | 10 | 10 |
| SP9 Story homing (2) | 10 | 10 | 10 | 10 |
| SP10 Selection reflects principal (2) | 10 | 10 | 10 | 10 |
| SP11 UX fidelity (2) | 10 | 10 | 10 | 10 |
| SP12 Edge cases surfaced (1) | 5 | 5 | 5 | 10 |
| **Σ score×weight** | 260 | 260 | 275 | 265 |
| **Total (0–100)** | 92.9 | 92.9 | 98.2 | 94.6 |

---

## SET E — `invoice-lifecycle-v1` (total 92.9)

| Row | Score | Evidence |
|-----|-------|----------|
| SP1 | 10 | Every story carries an explicit **Independent Test** + Given/When/Then (e.g. `stories/US-1.md` seed-one-account test; `stories/US-8.md` tenant-scoped client test). |
| SP2 | 10 | Scenarios state observable pass/fail — `stories/US-2.md` sc.3 exactly-once on replay; `stories/US-6.md` sc.3 no overdue on paid/void. |
| SP3 | 10 | Intent scope (`spec.md` l.15–20) matches card: create→send→Stripe pay + manual mark-as-paid + computed overdue + reminders; auth scoped out as a flagged platform assumption (A2), not a card contradiction. |
| SP4 | 10 | Out-of-scope list `spec.md` l.20 (recurring, multi-currency, estimates, client portal + partial/teams/dunning); no requirement violates it. |
| SP5 | 10 | Load-bearing money/reconciliation reqs unambiguous; residual confined to Assumptions A1–A3 + Open Questions Q1–Q2 (both non-blocking / deferred-feature). |
| SP6 | 10 | SC-001…008 each inline-tagged to verifying FEAT-IDs (`spec.md` l.121–128); observable/numeric-adjacent. |
| SP7 | 5 | Overdue-as-computed, partial-out, bookkeeper-door-preserved (l.19), and reminder cadence surfaced-as-undecided-then-ruled on the prototype walk (`stories/US-7.md` note) are honest — but the **retention/deletion** planted zone is absent entirely (no open question, no park), unlike F/G. One planted zone silently dropped. |
| SP8 | 10 | `feature-map-delta.md` is stories-first with per-feature extent/relations/obligations; US-7 "filter fired but escalated, not rejected" narrated; no map inflation. |
| SP9 | 10 | All 8 stories homed to exactly one FEAT (`spec.md` story table + delta). |
| SP10 | 10 | Feature Selection records dependency order, deferred void+reminders, and the deferred-SC cost (SC-006, void clause of SC-004); "re-confirmed by the principal at selection." |
| SP11 | 10 | Manifest SCR-001…008 all present + linked from `prototype/index.html`, no drift; deferred SCR-006/void greyed coming-soon; FLOW-001…009 cover P1 scenarios. |
| SP12 | 5 | EC-1…5 cover external/invalid/concurrent/permission/boundary, but the unauthenticated public pay-link **enumeration/unguessability** boundary is not named (EC-4 covers link reuse only). |

**Floor:** F-SP1–7 PASS; **F-X1 VIOLATION** — the artifact set shows no evidence of an independent
grade. `feature-map-delta.md` l.6 "Graded with the spec by mochiko:review-specifications" is a
standing process note, not evidence a grade occurred; the only rulings of record are the author's
and the principal's prototype-walk acceptance (no review verdict, findings, or dispositioned
finding anywhere in `spec.md`, `stories/`, or the operating-doc touches).

**Note:** Cleanest requirements layer of the four — inline SC→FEAT tagging and a crisp
computed-overdue treatment. Weakest on process evidence (no independent review surfaced) and on the
silently-dropped retention zone.

---

## SET F — `invoice-lifecycle` (total 92.9)

| Row | Score | Evidence |
|-----|-------|----------|
| SP1 | 10 | Every story has an Independent Test + scenarios (`stories/US-5.md` Stripe test-mode + replay; `stories/US-1.md` two-path sign-in + cross-account). |
| SP2 | 10 | Scenarios observable — `stories/US-8.md` filter-overdue returns exactly the overdue set with outstanding-total; `stories/US-3.md` N+1 gap-free number. |
| SP3 | 10 | Full lifecycle incl. auth folded in; out/in matches card (`spec.md` l.11–16); overdue auto-computed status, reminders in v1. |
| SP4 | 10 | Out-of-scope list l.16; off-amount payment held/flagged rather than auto-paid (edge, l.48) respects partial-out. |
| SP5 | 10 | Buildable; ambiguities parked in Assumptions + Open Questions (viewed-detection FR-010 fallback, retention). |
| SP6 | 10 | SC-001…010 measurable; each mapped to its verifying feature in the derived-features table (l.157–166). |
| SP7 | 5 | Retention parked explicitly and well (l.194, l.200 "principal's explicit request", ROADMAP Later) and partial parked — but reminder **cadence** (due/+3/+7, cap 3, per-invoice off) is presented as decided-at-intent (l.15, FR-018) though the card marks it undecided at intent, and the bookkeeper zone is not addressed. One zone quietly settled. |
| SP8 | 10 | `derivation.md` stories-first, parent/leaf structure, extent/not/relations/obligations per feature; no inflation. |
| SP9 | 10 | All 9 stories homed to exactly one FEAT (US-9 void added by review ruling, homed to FEAT-006). |
| SP10 | 10 | Selection records PM recommendation, the principal's ruling (all seven deliverables kept in v1), completeness ledger, deferred-SC (none). |
| SP11 | 10 | SCR-001…009 all present + linked, no drift; reminder-settings screen present; P1 flows have click paths. |
| SP12 | 5 | Rich edges (invalid, double-collection, off-amount, external payment + email failure, permission, reminder bound) but the public pay-link enumeration/unguessability boundary is not named. |

**Floor:** all specify floors PASS. **F-X1 PASS** — spec cites an independent review pass with
dispositioned findings (`stories/US-9.md` "Added by principal ruling at spec review (disposition
#3)"; Assumptions l.193 "ruled at review, disposition #1").

**Note:** Most complete FR layer (22 FRs, 10 SCs) and strongest retention honesty. Weakest points:
reminder cadence hard-coded as if decided at intent (against the card's planted vagueness), and the
whole-of-v1 selection is more ambitious than the founder's cut-features-not-quality posture (though
recorded as the principal's ruling).

---

## SET G — `invoice-lifecycle` (total 98.2)

| Row | Score | Evidence |
|-----|-------|----------|
| SP1 | 10 | Every story has an Independent Test + scenarios (`stories/US-5.md` pay + replay; `stories/US-1.md` both methods + cross-account refusal). |
| SP2 | 10 | Concrete + numeric — SC-001 "under 10 minutes", SC-002 "within one minute", SC-004 "zero cross-account incidents". |
| SP3 | 10 | Scope l.11–16 matches card; spine selected, rest deferred but specified; out-of-scope incl. bookkeeper door left open. |
| SP4 | 10 | Out-of-scope list l.16; nothing violates it; FR-022 archive-not-hard-delete respects retention-unknown. |
| SP5 | 10 | Buildable; strong edge treatment (tax-rate bounds `0 ≤ rate < 100`, reconcile-against-processor). Retention + cadence held as explicit do-not-guess open questions. |
| SP6 | 10 | SC-001…008 numeric/observable; mapped to features in the derived-features table (l.153–163); SC-008 cross-cutting a11y flagged. |
| SP7 | 10 | Every planted zone handled honestly: retention parked with a research obligation ("do NOT guess", l.187), reminder cadence deferred until shown concrete options (l.188), overdue resolved, partial out, bookkeeper door explicitly preserved in the data model (l.12, l.16, FR-022). None invented. |
| SP8 | 10 | Derived-features table with parent roll-ups (FEAT-004, FEAT-007), no rejections, no inflation; every feature traces to stories. |
| SP9 | 10 | All 8 stories homed to exactly one FEAT. |
| SP10 | 10 | Selection records the founder's smallest-valuable-slice spine, foundation, deferred features with deferred-SC list and per-parent completeness ledgers. |
| SP11 | 10 | SCR-001…009 present + linked, deferred SCR-008 greyed coming-soon; selected P1 flows FLOW-001…005 have click paths; negative states honestly noted as build-verified not prototyped. |
| SP12 | 5 | EC-1…7 strong (delayed-event reconcile, replay, invalid incl. tax bound, bounce, permission, concurrent forward-guard, double-spend link deactivation) — but the public pay-link enumeration/unguessability boundary is not named. |

**Floor:** all specify floors PASS. **F-X1 PASS (thin)** — a dispositioned review finding is cited
(`spec.md` l.143 negative-states "verified at build (ruled at disposition)"); no standalone review
report in the artifact set, but a review→disposition cycle is evidenced.

**Note:** Strongest on principal fidelity — it is the only set that honestly parks or preserves
*every* planted-vague zone (retention, cadence, overdue, partial, bookkeeper) rather than settling
any as invented scope, which is precisely what the fixture rewards. Same lone edge-case gap as
E/F/H's non-security peers (public-link enumeration).

---

## SET H — `invoice-lifecycle` (total 94.6)

| Row | Score | Evidence |
|-----|-------|----------|
| SP1 | 10 | Every story has an Independent Test + scenarios (`stories/US-4.md` confirmed-event-not-redirect; `stories/US-8.md` both methods + scoped session). |
| SP2 | 10 | Concrete + numeric — SC-001 "under 5 minutes", SC-002 "100% of successful Stripe payments"; scenarios observable. |
| SP3 | 10 | Out-of-scope is *exactly* the card's firm four (l.18); full lifecycle incl. auth folded, both payment channels in slice 1, overdue computed on read. |
| SP4 | 10 | Out-of-scope list l.18; per-feature "Not" scopes in `feature-delta/`; partial payments held as open question, not built. |
| SP5 | 10 | Load-bearing ambiguities actively closed at the clarification gate: unguessable non-enumerable token (FR-016), payment-processing state for the redirect→webhook gap (FR-022/023), half-up rounding with tax-on-subtotal (FR-010), void-and-reissue lock (FR-019/020). |
| SP6 | 10 | SC-001…007 measurable; mapped to features in the derived table; SC-005 split (glance via FEAT-002, dashboard FEAT-007) made explicit. |
| SP7 | 5 | Overdue (FR-013 computed on read), partial (open question, ruled out at gate), and cadence (deferred, not fixed) handled honestly — but the **retention/deletion** zone is absent from Open Questions (only a Minor client-delete design note in `reviews/round-1.md`), and bookkeeper is not addressed. One planted zone silently dropped. |
| SP8 | 10 | `feature-delta/selection-card.md` + per-FEAT delta files + `features-index-draft.md`; stories→features, no rejections, parent roll-up, no inflation. |
| SP9 | 10 | All 8 stories homed to exactly one FEAT (US-8 auth folded at the selection gate, homed to FEAT-009). |
| SP10 | 10 | Selection records order, auth-folded-by-user-ruling, deferred FEAT-007/008 with deferred-SC (SC-006). |
| SP11 | 10 | SCR-001…011 present + linked, deferred SCR-009 greyed; email-preview screen; P1 flows have click paths; the review-caught guessable-link defect fixed in both spec (FR-016) and design intent. |
| SP12 | 10 | Only set to name the public pay-link **enumeration** boundary as a Critical and close it (FR-016; `reviews/round-1.md` G1), in addition to invalid/rounding/bounce/duplicate-forged/double-payment edges. |

**Floor:** all specify floors PASS. **F-X1 PASS (strongest)** — two dated review rounds with
verdicts and severity-classified findings (`reviews/round-1.md` verdict `needs-revision` with
Critical G1; `reviews/round-2.md`).

**Note:** Best process trail and the only set whose independent review caught and closed a real
Critical access-control gap (guessable/enumerable public pay-link) plus pinned money-rounding and a
payment-processing state. Weakest on the retention planted zone, which it drops silently rather than
parking.

---

## Floor summary

| Floor | E | F | G | H |
|-------|---|---|---|---|
| F-SP1 story acceptance scenarios | PASS | PASS | PASS | PASS |
| F-SP2 out-of-scope list | PASS | PASS | PASS | PASS |
| F-SP3 Screens & Flows section | PASS | PASS | PASS | PASS |
| F-SP4 Feature Selection + user ruling | PASS | PASS | PASS | PASS |
| F-SP5 every story homed/rejected | PASS | PASS | PASS | PASS |
| F-SP6 no placeholder tokens | PASS | PASS | PASS | PASS |
| F-SP7 UX-bearing ⇒ prototype, no drift | PASS | PASS | PASS | PASS |
| F-X1 independently graded | **VIOLATION** | PASS | PASS | PASS |

Only violation across all four: **E fails F-X1** — no independent grade of record in the artifacts
(only author + principal acceptance).

---

## Comparative read

All four are strong, buildable specs with independently testable stories, clean story-homing,
coherent feature maps, and drift-free prototypes whose manifests match their files — SP1–SP4, SP8–
SP11 hit the top anchor for every set, so the spread is driven almost entirely by two rows:
principal-fidelity on the planted-vague zones (SP7, weight 3) and edge-case depth (SP12). **G leads
at 98.2** because it is the only set that honestly parks or preserves *every* planted-vague zone —
retention (research obligation, do-not-guess), reminder cadence (deferred until concrete options),
overdue, partial payments, and the bookkeeper seat (data-model door explicitly left open) — inventing
none as settled scope, which is exactly the fidelity the fixture is built to test. **H (94.6)** is
second on the strength of the only genuine independent review that caught and closed a Critical
security gap (an enumerable public pay-link) and pinned money-rounding and a payment-processing
state, but it silently drops the retention zone. **E and F tie at 92.9** on the rubric: both settle
one planted zone less than honestly (E drops retention entirely; F hard-codes a reminder cadence the
card leaves undecided) and both miss the pay-link enumeration edge — but they separate at the floor,
where **E is the only set with no evidence of an independent grade (F-X1 VIOLATION)** while F carries
dispositioned review findings. The shared weakness across E, F, and G is the unaddressed
public-link enumeration boundary that only H's review surfaced.
