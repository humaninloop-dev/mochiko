# Command plan-only eval — probe report (build item 0)

**Date:** 2026-08-27 · **Probe target:** `/mochiko:brainstorm` (smallest pair, 29 rules) ·
**Ruling home:** `.mochiko/brainstorms/command-plan-only-eval/record.md` (accepted, D1–D11)

Two runs. Run 1 failed free and taught the first finding; run 2 succeeded end-to-end.
Captured plan: [`plan.md`](plan.md) (12,724 chars). Raw stream + workdir stayed in the
session scratchpad (not committed).

## Invocation that works

```
cd <ephemeral-workdir>   # fixture files + copy of working-tree plugins/
claude -p "/mochiko:brainstorm <topic>" \
  --plugin-dir <workdir>/plugins/mochiko \
  --setting-sources "" \
  --allowedTools "Read,Grep,Glob" \
  --permission-mode dontAsk \
  --max-turns 25 --model sonnet \
  --output-format stream-json --verbose \
  --append-system-prompt "<plan-only form wrapper>"
```

## Findings

1. **`--bare` and stored auth are mutually exclusive** (run 1: `Not logged in`, exit 0,
   $0.00). `--bare` skips stored credentials by design — the skill harness's own
   `evals/run.py` documents this and drops `--bare` outside API-key `--local` mode.
   **Amends the record's build item 1: drop `--bare`; isolation = `--setting-sources ""`
   (drops user/project config and installed plugins, keeps stored auth) + neutral cwd.**
2. **Invocability confirmed.** A `disable-model-invocation: true` slash command executes
   headless when typed as the `-p` prompt. The whole build is unblocked.
3. **Schema-read path confirmed under workdir provisioning (C4 resolved).** First actions
   were Read of `plugins/mochiko/schemas/brainstorm.yaml` (raw, in full) and
   `command-labels.yaml` from the provisioned tree; the run even verified the .md's
   pinned fail-condition count (4) against the schema.
4. **No shadowing (I1 resolved).** Init event lists exactly one plugin: `mochiko` v0.97.0
   from the workdir path. `plugin_errors` FIELD-ABSENT (as the skill harness found) — the
   load gate must assert plugin name+version present in the init event, which works.
5. **Fence nuance (D7).** `--allowedTools` is a permission allow-list, not a roster strip:
   the init event still lists Task/Bash/Edit/Write/Agent-class tools; unlisted tools are
   denied at call time under `dontAsk`. In this run the model attempted only
   Read/Grep/Glob — no dispatch, no write. Structural denial holds; record language
   "tool strip" should read "permission fence" at build.
6. **Absent-user contract works (D9 confirmed).** Every user gate is described with its
   onward branches (waiver gate, per-survivor-type disposition gates, acceptance gate,
   synthesis/next-step offers). No stall, no bare confirmation request.
7. **Plan shape is scenario-concrete (D3 largely confirmed) with one defect: the plan
   volunteers schema rule-ID citations** (`brainstorm.index-bookkeeping`,
   `brainstorm.fail.survivor-undispositioned`, …) even though the wrapper never asked.
   Citations would let the coverage judge lexically match instead of inferring
   embodiment — undermining exactly what D3 protects. **Build fold: add a register line
   to the wrapper — "express actions in plain terms; do not cite schema rule IDs" —
   register/form, not content, so D11-compatible.**
8. **Interpretation quality was real, not recited:** proposed a concrete slug
   (`repo-health-digest`), a topic-specific 8-question sequence, detected the missing
   `.mochiko/memory/knowledge-management.md` and planned the KM ritual as a conditional
   no-op, and noticed the workdir is not a git repository.
9. **Fixture-consistency lesson:** the hand-made index referenced a
   `sample-prior-session/record.md` that did not exist → mid-run tool error (the model
   recovered). Fixture authoring needs an internal-consistency check (every referenced
   path exists).
10. **Cost/turn envelope:** $0.6492, 107 s, and `num_turns` = 25 — exactly the
    `--max-turns` cap. The plan appears complete (phases through the done-condition
    check), but running at the cap is a flag: **build should raise headroom (~40) and
    treat cap-hit as a per-run warning.** F4's skill anchor (~$0.55) held for the
    smallest command; expect `implement` higher (M5 stands).

## Probe questions vs record

| Probe question (build 0) | Outcome |
|---|---|
| Invocability under `disable-model-invocation: true` | PASS (finding 2) |
| Ephemeral-workdir provisioning / schema path (C4) | PASS (finding 3) |
| Plugin shadowing / load gate (I1) | PASS via init-event version assert (finding 4) |
| Allow-list fence (D7) | Holds as permission fence; roster not stripped (finding 5) |
| Absent-user contract (D9) | PASS (finding 6) |
| Plan shape under form contract | PASS with rule-ID-citation defect → wrapper register line (finding 7) |
| Real cost (`total_cost_usd`) | $0.6492 / 107 s / at turn cap (finding 10) |

**Net: substrate bet stands; no design change beyond build-level amendments (drop
`--bare` · wrapper register line · max-turns headroom · fixture consistency check).**
