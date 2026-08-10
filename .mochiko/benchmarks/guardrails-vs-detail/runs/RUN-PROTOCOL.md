# Run protocol — guardrails-vs-detail benchmark (FROZEN)

Ruled: validator-scope-and-verbosity D2/D3/D6 (as amended). One document governs every stage
run so runs differ ONLY in variant arm and replicate seed.

## Run matrix

4 variants × 2 commands × 2 replicates = 16 runs — executed in **checkpointed stages**
(user-ruled 2026-08-10, staged execution, not a protocol deviation):

- **CP1:** setup × 4 variants × r1 → judge the 4 → preliminary report → user rules continue/stop.
- **CP2:** specify × 4 variants × r1 (seeded per Seeding) → judge → preliminary two-command report.
- **CP3:** r2 replicates only where the preliminary gap is within noise of the 10% line (or
  a floor/fire/route result needs confirmation). The D6 decision rule fires formally only
  where ≥2 replicates exist; CP1/CP2 verdicts are explicitly preliminary (n=1, no noise guard).
  An arm losing catastrophically at n=1 may be ruled without r2 by the user (recorded).

| Variant | Skill bodies | Skill descriptions | Agent descriptions |
|---|---|---|---|
| `baseline` | `plugins/mochiko/skills/` | original | original |
| `body` | `variants/body/` (fallback to original for non-cluster skills) | original | original |
| `descriptions` | original | `variants/descriptions/` | original |
| `agents` | original | original | `variants/agents/` |

Run ID: `<command>-<variant>-r<n>` (e.g. `setup-body-r1`). Output dir: `runs/<run-id>/`.

## Simulation shape

Each run is executed by one **run-lead agent** simulating the command end-to-end in an
isolated output sandbox:

- The run-lead receives: the command file text (`setup.md` or `specify.md`), a **skill
  roster** (name + description per the variant column — descriptions are how it decides what
  to invoke), an **agent roster** (name + description per the variant column), and the paths
  to load skill bodies from when it invokes one.
- The run-lead plays every seat itself after "dispatching" it (single-agent simulation) but
  MUST load the governing skill body from the run's designated path before performing that
  seat's work, and must honor author≠grader independence by producing grader outputs strictly
  from the artifact text.
- **The principal is answered from the frozen card** (`fixture/persona-card.md`). The
  run-lead spawns a fresh principal subagent per question batch: the subagent gets the card +
  the dialogue transcript so far + the new question(s), and returns only the principal's
  reply per the card's answering policy. The run-lead NEVER reads the card itself — the card
  path is passed only into principal spawns. This is the D3 contamination fence.
- All artifacts a real run would write to the project land under `runs/<run-id>/project/`
  (treat it as the fake Ledgerline repo root; it starts empty — greenfield).
- Setup runs execute `/mochiko:setup` for the Ledgerline project (greenfield, production
  floor). Specify runs execute `/mochiko:specify` for the card's "Invoice lifecycle v1"
  feature; the specify run's project dir is seeded by copying the SAME frozen setup output
  (see Seeding) so specify runs never inherit variance from their own arm's setup run.

## Seeding (specify runs)

All 8 specify runs start from an identical frozen project state: `runs/_seed-project/` —
produced once by the FIRST completed baseline setup run (setup-baseline-r1's `project/`,
copied verbatim). This isolates the specify comparison from setup output variance.

## Capture obligations (every run)

Write into `runs/<run-id>/`:
- `project/` — the artifacts (the judged object)
- `transcript.md` — the full simulated dialogue (questions, principal replies, gate rulings)
- `invocations.json` — every skill invocation decision: `{"moment": "...", "skill_chosen":
  "...", "skills_considered": [...], "from_description_only": true}` — this feeds the
  fire-rate check (descriptions arm) and route check (agents arm)
- `costs.json` — `{"principal_spawns": n, "approx_tokens_in_skills_loaded": bytes_loaded,
  "wall_notes": "..."}` — bytes of every skill body loaded, summed, as the token proxy
- `meta.json` — run id, variant, replicate, start/end, deviations from this protocol (any
  deviation must be recorded, not silently absorbed)

## Fire-rate / route checks

- **descriptions arm:** after the run, compare `invocations.json` against the baseline
  replicates' invocation sets: any cluster skill the baseline invoked at a moment where the
  slim-description run did not (or invoked the wrong sibling) = a fire miss.
- **agents arm:** same comparison over seat routing: wrong persona chosen for a seat, or a
  seat never staffed = a route miss.
- Misses are floor-adjacent evidence, reported alongside scores (D6).

## Judging (after all runs)

Judge agent reads `runs/<run-id>/project/` ONLY (never transcripts), scores against
`rubric/setup-artifacts.md` / `rubric/specify-artifacts.md` + `rubric/floor-checks.md`,
blind to variant (run dirs are presented to the judge under anonymized labels A/B/C/...).
Persona card is the judge's ground truth for principal-fidelity rows. Decision rule: D6 —
10% threshold vs baseline mean, floors absolute, noise guard on replicate spread.
