# Rubric — /mochiko:specify run artifacts

Outcome-keyed scoring rubric for the spec workspace a `/mochiko:specify` run produces.
Authored fresh for the guardrails-vs-detail benchmark by a seat that authored neither
variant (record D5). Blind to both skills' internals by construction — every row is an
outcome a *buyer of the spec* (a builder, or the principal signing off) would ask about,
never a check against how the spec was made.

## Artifact set in scope

Score the run's produced workspace only:

- `spec.md` — the **Intent** section, FR-XXX functional requirements, measurable SC-XXX
  criteria, edge cases, the **Screens & Flows** section (SCR-XXX/FLOW-XXX manifest, or the
  "No UX surface — prototype waived at intent." line), the **Feature Selection** section
  (derived features, filter verdicts with reasons, the user's selection + deferred-SC list)
- the story files `stories/US-*.md` (text, acceptance scenarios, FEAT-ID mapping, or
  `rejected` with the why)
- the staged feature-map delta (`FEATURES.md` / `.mochiko/features/` entries as they stand
  in the workspace)
- the clickable prototype under `prototype/`, if the spec is UX-bearing

## Scoring protocol (read before scoring)

1. **Artifacts only.** Read the run's produced workspace. Never read a transcript of how it
   was made, an author's report, or the skill/variant that produced it.
2. **Blind to variant.** You do not know and must not infer which variant (baseline,
   body-guardrails, slim-descriptions, slim-agent-descriptions) produced this workspace.
3. **Ground truth for the principal.** Where a row asks whether scope matches what the
   principal ruled in or out, the fixture persona + project-facts card is the source of
   truth for what the principal actually said — cross-check against the card.
4. **Score each row 0 / 5 / 10** against its concrete anchors, and write a one-line evidence
   note (cite the artifact + what you saw). A row with no citable evidence scores 0.
5. **Total.** Weighted mean of the row scores, normalized to 0–100:
   `total = 10 × (Σ score_i × weight_i) / (Σ 10 × weight_i)`.
6. **Floors are separate.** This rubric measures quality on a gradient. Floor violations
   (see `floor-checks.md`) are a binary automatic flag regardless of the score here (D6).

## Rows

| ID | Outcome question | 0 | 5 | 10 | Weight |
|----|------------------|---|---|----|--------|
| SP1 | **Independently testable stories** — can each story be verified on its own? | Stories are entangled; none carries an independent test or standalone acceptance. | Most stories are independently testable; a few depend on others to verify. | Every story carries an acceptance scenario (or named independent test) that verifies it alone. | 3 |
| SP2 | **Measurable acceptance** — are the Given/When/Then scenarios concrete enough to pass or fail unambiguously? | Acceptance conditions are vague ("works well", "user is happy"). | Some scenarios are concrete, others leave the pass condition to interpretation. | Every acceptance scenario states an observable, unambiguous pass/fail condition. | 3 |
| SP3 | **Scope fidelity** — does the spec's scope match what the principal ruled in and out? | Spec scope contradicts the card — builds something the principal excluded, or omits something ruled in. | Mostly faithful, with one boundary drifting from the card. | The Intent section's scope matches the principal's in/out rulings on the card exactly. | 3 |
| SP4 | **Out-of-scope honored** — is the out-of-scope list present, and does nothing in the spec violate it? | No out-of-scope list, or requirements contradict it. | List present; a requirement brushes against an excluded item. | Out-of-scope list present and every requirement respects it. | 2 |
| SP5 | **No blocking ambiguity** — could a builder start without being blocked by an unresolved question? | Load-bearing requirements are ambiguous; a builder would stall immediately. | Buildable in the main, with an ambiguity or two that would need a round-trip. | Nothing load-bearing is ambiguous; residual uncertainty is confined to a marked Assumptions/Open-Questions section. | 3 |
| SP6 | **Success criteria measurable & traced** — are SC-XXX numeric/observable and each tied to a verifying feature? | SC entries are absent or unmeasurable. | SC present and measurable, but traceability to features is partial. | Every SC-XXX is numeric or observable and maps to the feature that verifies it. | 2 |
| SP7 | **Vagueness handled honestly** — were the card's planted-vague zones surfaced and resolved with the principal, not silently invented as scope? | A planted-vague zone was resolved by inventing scope presented as settled. | One vague zone handled honestly, another quietly filled in. | Every planted-vague zone is resolved with the principal on record or parked as an open question — none silently invented. | 3 |
| SP8 | **Derivation honesty** — do derived features trace to stories, with filter rejections reasoned and no map inflation? | Features invented with no story behind them, or rejections dropped silently. | Derivation mostly honest; a rejection reason is thin or a feature is loosely justified. | Every derived feature traces to stories; every filter rejection carries a recorded reason; no capability inflated onto the map. | 2 |
| SP9 | **Story homing** — is every story homed to exactly one feature or explicitly rejected? | Stories float unhomed, or one story spans several features ambiguously. | Most stories homed cleanly; one is ambiguous or unhomed. | Every story maps to exactly one FEAT-ID or is marked `rejected` with a recorded why. | 2 |
| SP10 | **Selection reflects the principal** — does the Feature Selection section record the principal's ruling, not an auto-pick? | Selection was made for the principal with no record of their ruling. | Selection recorded, but the deferred-SC list or reasoning is thin. | The section records the principal's selection, the dependency order, and the deferred-SC cost of what was cut. | 2 |
| SP11 | **UX fidelity** — if UX-bearing, does the prototype match the manifest and cover the P1 flows; if not, is the waiver honest? | UX-bearing but no prototype, or manifest↔prototype drift; or a false waiver on a UX-bearing feature. | Prototype present with minor drift, or a P1 scenario missing a click path. | Every SCR-XXX reachable, every FLOW-XXX walkable, every P1 acceptance scenario has a click path, no drift — or an honest waiver line on a genuinely non-UX feature. | 2 |
| SP12 | **Edge cases surfaced** — does the spec name the failure and boundary cases, not just the happy path? | Only the happy path is specified. | Some edge cases named; obvious ones (empty, error, concurrent, limit) missing. | Edge cases are identified for the load-bearing requirements, covering failure and boundary conditions. | 1 |

Total weight = 28. Row count = 12.
