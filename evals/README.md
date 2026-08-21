# evals/ — skill-compression eval runner (maintainer-side, never shipped)

Provenance: `.mochiko/brainstorms/skill-compression-tooling/record.md` (D1–D8 as amended) and
`primitive-eval-harness` D1–D5 (this directory is that session's ruled home; the pilot here
fills its D5 slot). Advisory tooling under GI-019's recorded trace — it never gates pipeline
progress, never dispatches pipeline agents, never holds judgment skills own. Nothing in this
directory ships with the plugin (GI-020 untouched).

## What one run is

One isolated `claude -p --bare` session, loading a **synthesized minimal plugin** that carries
only the skill variant under test, executing one golden prompt in a throwaway workspace. The
produced artifact is graded two ways:

- **Scripted assertions** (deterministic, from `evals.json`) — may block.
- **Rule-coverage checklist** (Haiku judge; one binary per baseline rule with a quoted
  evidence span) — advisory. Aggregation is **pass^k**: a rule holds only if it holds in all
  replicates.
- A **pairwise blind A/B** (Sonnet judge, position-swapped) runs as a secondary sanity read.

Arms: `noskill` (control — rules that pass here measure the model, not the skill, and are
pruned) · `baseline` (current `plugins/mochiko/skills/<skill>/`) · `armA` · `armB`
(variants staged by the `compressing-skills` repo skill).

## Layout

```
evals/
  run.py                      # the runner
  <skill>/
    evals.json                # 3 goldens: {id, prompt, expected_output?, assertions[]}
    rules.json                # rule inventory: {id, rule, class, source}
    preregistration.md        # ship bar + delivered-chars arithmetic — REQUIRED before a grid
    variants/armA/ armB/      # full skill-dir copies (staged by compressing-skills)
    runs/<stamp>/             # transcripts, artifacts, summary.json, report.md
    pass-report.md            # the compression pass report (compressing-skills step 7)
    baseline/                 # committed baseline results; regenerate only as a landing act
```

## Usage

```
python3 evals/run.py probe   <skill>              # R5: settle flags empirically (1 cheap run)
python3 evals/run.py grid    <skill> [--replicates 3] [--arms noskill,baseline,armA,armB]
python3 evals/run.py report  <skill>              # rebuild report.md from the latest run
```

Requirements — **sandbox mode (default)**: Docker AI sandbox `claude-mochiko` (`sbx` CLI)
with a logged-in claude agent; sessions run via `sbx exec` on the sandbox's stored
subscription auth — no API key. Isolation (probe-settled 2026-08-22, R5): neutral cwd
`/tmp/eval-*` inside the sandbox + `--setting-sources ""` (the sandbox carries a user-level
mochiko plugin install that otherwise loads the real skill beside the variant; the probe
caught it). `--bare` is dropped in sandbox mode — it skips stored credentials by design.
`plugin_errors` was FIELD-ABSENT on CLI 2.1.221; the load gate is "synthesized skill visible
in the init event" instead. **`--local` mode**: the original `claude -p --bare` path;
requires `ANTHROPIC_API_KEY` (metered spend; `total_cost_usd` is a client-side estimate). Session model under test: Sonnet (ruled R7). Judges: Haiku (checklist), Sonnet (pairwise —
baseline vs each variant, first replicate per golden, position-swapped). Permission mode is
`acceptEdits`, a recorded divergence from the record's `dontAsk` wording: I2 found `dontAsk`
denies writes absent allow rules; R5 mandates settling the flags empirically, and
`acceptEdits` is the build's call under that mandate — the probe verifies it.

## Discipline (ruled; do not relax in code review)

- The grid refuses to run without `preregistration.md` (R6/R9).
- Floor-class rules are absolute: one lost floor rule kills the arm.
- Judges are advisory — the runner exits 0 on judged degradation and nonzero only on
  mechanical failure (missing prereq, spawn failure, failed scripted assertion in `baseline`).
- Baseline results under `baseline/` are committed and regenerated only as a deliberate,
  reviewed act.
