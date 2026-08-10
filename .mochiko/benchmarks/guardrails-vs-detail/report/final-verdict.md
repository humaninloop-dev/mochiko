# Guardrails-vs-detail benchmark — verdict package (after CP1+CP2+CP3 targeted)

12 runs executed and blind-judged: 4 arms × 2 commands × r1, plus body/agents × 2 commands × r2
(descriptions arm r2 skipped by user ruling — its fire-rate question closed at r1×2 commands;
baseline r2 skipped by the same targeted ruling, so baseline means are n=1: caveat C-1).

## Score matrix (blind LLM-judge, neutral rubric, 0–100)

| Arm | setup r1 | setup r2 | specify r1 | specify r2 | Floor violations |
|---|---|---|---|---|---|
| baseline | 96.4 | — | 98.2 | — | 0 |
| body | 100.0 | 100.0 | 94.6 | 94.6 | 0 |
| agents | 100.0 | 94.4 | 92.9 | 100.0 | 1 at r1 (F-X1), 0 at r2 |
| descriptions | 94.6 | — | 92.9 | — | 0 |

## D6 rule, applied formally

**Body arm (≥2 replicates both commands):**
- setup: mean 100.0 vs 96.4 → **+3.7%**; replicate spread 0.0
- specify: mean 94.6 vs 98.2 → **−3.7%**; replicate spread 0.0
- Floors: 0 violations in 4 runs. Noise guard: spread (0.0) < gap — does not fire.
- **VERDICT (formal, subject to C-1): detail dies for the body cut.** Guardrails bodies ship
  for the setup+specify cluster. Token side: −18% to −46% skill-bytes per run.

**Agents arm (≥2 replicates both commands):**
- setup: mean 97.2 vs 96.4 → +0.8%; spread 5.6
- specify: mean 96.5 vs 98.2 → −1.8%; spread 7.1
- Noise guard: spread EXCEEDS gap at both commands → formally fires (one more replicate pair
  before a mean-based verdict). Bounded observation: every individual replicate is within
  10% of baseline (worst −5.4%), so no possible mean crosses the threshold — the guard can
  change the number, not the verdict class.
- F-X1: violated once (specify r1), not reproduced at r2 — but **confound C-2**: the r2
  briefs spelled out the review-evidence obligation explicitly, so non-reproduction does not
  cleanly exonerate the variant. Strict D6 reading: a violation in any replicate = "detail
  pays" for that floor's section.
- **VERDICT: (b) ruled by the user, 2026-08-10** (lead-recommended; options (a) strict-D6 and
  (c) further replicates presented and declined): **all six agents ship prose-only.** Basis:
  the arm's mechanism (routing) never failed across 20+ staffing decisions in 4 runs; the
  F-X1 violation's causal chain to the variant is weak (a run-lead paperwork behavior, not a
  staffing failure); restoring ~2,400 chars of example blocks guards nothing measured.
  Recorded with the C-2 confound acknowledged. **Watch item:** F-X1-class review-evidence
  omission at first live runs on the cut descriptions; recurrence re-opens this ruling.
  Mitigation preferred over examples: the cross-cutting finding 1 floor line ("independent
  review leaves verdict + dispositions in the artifacts") — guardrail, not prose.

**Descriptions arm (r1 only, both commands — mechanism verdict):**
- Fire-rate: 0 misses over 12+ invocation moments across 2 commands, incl. both deliberate
  sibling traps and all correct non-fires. Scores 94.6/92.9, within 10% (n=1).
- **VERDICT (mechanism-based): slim descriptions ship.** The arm's question was routing
  reliability; it is answered by counts, not judge scores. Score-side formality is n=1
  (caveat C-1 class), accepted by the checkpointed-execution ruling.

## Cross-cutting findings

1. **The judgment-margin pattern held at every checkpoint:** score spread never came from
   floors, structure, or competence — only from planted-vague-zone honesty (SP7/S7) and
   edge-case depth. Sets that surfaced every "I don't know" zone scored 100; sets that
   silently omitted one landed ~94–95. No set invented facts. This is variant-INDEPENDENT:
   baseline lost CP1 on over-ceremony, body lost specify-r1/r2 on a dropped retention zone,
   agents lost setup-r2 the same way. Vague-zone surfacing discipline looks like a run-lead
   behavior, not a skill-detail effect — candidate for a one-line floor obligation instead of
   returning any playbook prose.
2. **Full-detail's failure mode is over-production** (CP1: heaviest governance, right-sizing
   loss) — detail did not buy honesty; it bought weight.
3. **Cost:** body arm 40,459–57,135 skill-bytes/run vs baseline 75,038–90,979. Descriptions
   68% smaller on the router surface. Agent descriptions 69–81% smaller without example blocks.

## Caveats

- **C-1:** baseline unreplicated (n=1 means) — the CP1↔CP2 rank flip shows single-run
  variance is real; body's verdict margin (±3.7%) is inside observed cross-run wobble. The
  10% threshold verdict is robust to this; the sign of the delta is not.
- **C-2:** r2 briefs hardened the review-evidence obligation → F-X1 non-reproduction is
  confounded.
- **C-3:** all runs are single-agent simulations with a model-played principal.
- **C-4:** setup+specify only; plan/implement/brainstorm untested (record open thread 1).

## What the cost gate (D7) inherits

Winning-variant weights (body arm, per-class budget baseline = winner + 25%):
- Skill bodies (cluster): budget per skill = its guardrails variant size + 25%.
- Skill descriptions: slim sizes (≤500 chars) + 25% headroom; hard cap stays 1,536.
- Agent descriptions: prose-only sizes (268–737 chars) + 25% — live per the (b) ruling.

## Status

**All three arms resolved (2026-08-10):** body — guardrails bodies ship (formal D6 PASS,
C-1 noted) · descriptions — slim descriptions ship (mechanism verdict) · agents — prose-only
ships (user ruling (b), F-X1 watch item). Benchmark complete; next phase is the build: ship
the winning variants through the primitive-edit ceremony (strips + author≠grader audits) and
land the D7 cost gate with the budgets above.
