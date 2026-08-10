# CP1 checkpoint report — setup stage, single replicate (PRELIMINARY: n=1, no noise guard)

Judged blind (sets anonymized A–D; key file untouched by the judge). De-anonymized after
scoring against `report/cp1-anon-key.txt`.

## Scores vs baseline (D6 rule, preliminary)

| Arm | Run | Score | Δ vs baseline | 10% rule (prelim) | Floors | Fire/route misses | Skill-bytes loaded |
|---|---|---|---|---|---|---|---|
| **body** | setup-body-r1 | **100.0** (A) | **+3.7%** | within (better) | 0 violations | 0 | 75,038 (−18%) |
| **agents** | setup-agents-r1 | **100.0** (B) | **+3.7%** | within (better) | 0 violations | 0 of 5 seats | ≈ baseline |
| baseline | setup-baseline-r1 | **96.4** (C) | — | — | 0 violations | — | 90,979 |
| **descriptions** | setup-descriptions-r1 | **94.6** (D) | **−1.9%** | within | 0 violations | 0 of 6 moments | 64,413 (−29%) |

## Headline

**Every cut arm scored within the 10% threshold at n=1 — two of three beat baseline.**
Baseline's only rubric loss was S6 right-sizing: the full skills produced the *heaviest*
governance (30 GI elements, mandatory email-auth stack, audit trail, unwaivable a11y) — the
judge read it as over-ceremony for the card's ceremony-averse solo founder. The guardrails
body variant produced a ceiling-score set at 18% fewer skill-bytes.

## Per-arm notes

- **body (100.0):** most complete set (trace summary + floor-coverage check); a11y handled as
  waivable adopted-standard — the judge sided with this reading over legal-mandate.
- **agents (100.0):** cleanest fidelity (manual mark-as-paid kept first-class). Raw-data flag:
  produced NO a11y governance at all — falls through the rubric (a11y is module-class, not a
  floor row), so unpriced by score. Also: only 2 of 6 varied agents staffed setup seats — this
  arm's real routing test is thin at setup; CP2 (specify) exercises more of the roster.
- **baseline (96.4):** all-user-ratified but heaviest; S6 right-sized = 5.
- **descriptions (94.6):** routing perfect (0 misses, incl. correct non-fires); lost only S5
  fidelity — never governed the decided manual mark-as-paid feature (the single load-bearing
  card fact any set omitted). Cheapest arm (−29% skill-bytes). Note: the miss is a *content*
  omission in an authoring seat, not a routing failure — weak evidence against slim
  descriptions specifically; r2 would show whether it repeats.

## Caveats (binding)

- n=1: no replicate spread, noise guard unfired; D6's formal verdict needs ≥2 replicates.
- Scores cluster within 5.4 points — inside plausible single-run noise for an LLM-judged
  rubric; ranking at this margin is signal-bearing but not verdict-bearing.
- Agents arm under-exercised at setup (2/6 varied agents fired).

## Decision asked of the user (checkpointed execution ruling)

Continue to CP2 (4 specify runs off the frozen baseline seed) / adjust / stop.
