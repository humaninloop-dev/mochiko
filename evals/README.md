# evals/ — skill-compression eval runner (maintainer-side, never shipped)

Provenance: `.mochiko/brainstorms/skill-compression-tooling/record.md` (D1–D8 as amended) and
`primitive-eval-harness` D1–D5 (this directory is that session's ruled home; the pilot here
fills its D5 slot). Advisory tooling under GI-019's recorded trace — it never gates pipeline
progress, never dispatches pipeline agents, never holds judgment skills own. Nothing in this
directory ships with the plugin (GI-020 untouched).

## Targets and the one vocabulary

The eval layer has three **targets** — skills (this runner), commands and agents (the plan-only
runner, `evals/commands/run.py` + `agents.py`) — under one ruling set and one vocabulary
(`primitive-eval-harness-v2` D1, accepted 2026-09-08). Every runner cites this file for it:

| term | meaning |
|---|---|
| **golden** | one frozen scenario: fixture + prompt/card + `expected_output` + assertions + `tempts` (the rule/claim ids it invites — the band's denominator); changes only as a landing act |
| **rules / claims** | the rubric — a skill's rule inventory, a command's schema rule ids, a persona's body-derived claims with minted ids |
| **partition** | `plan-observable` vs `out-of-instrument` (+ why) — every rule/claim in exactly one list; commands in `observable.yaml`, personas inside `rules.json` |
| **arms** | every target `pre` (at `--old-ref`) · `post` (working tree); `noskill` / `nocmd` / `nopersona` = bare-model control (skills' legacy `baseline · armA · armB` runs stay readable, never runnable) |
| **pre-registration** | `preregistration.md` before any grid: read rule, tolerance band, noise guard, ship bar (personas: + positive control, stopping rule, judge calibration bar, budget bound) |
| **pass^k** | a rule/claim holds only if it holds in every replicate |
| **deterministic-may-block / judge-advisory** | scripted asserts may fail a run; LLM-judge readings never set an exit code (old D2, re-affirmed v2) |
| **baseline/** | a committed frozen result, regenerated only as a landing act (v2 D13(1)); never the name of a live arm |
| **model-native** | a claim the bare model satisfies without the primitive — tagged at mint by the control pass, excluded from the regression read, never deleted |

`evals/contract/` is **not** an eval target: it is the GI-012 release gate.

## What one run is

One isolated `claude -p` session on the host's stored subscription auth (`--setting-sources ""`
so no user-level install loads beside the provisioned tree), loading the **whole `plugins/mochiko`
tree** provisioned OUTSIDE the workspace — the working tree for `post`, a git-archived `--old-ref`
for `pre`, no plugin at all for the bare-model `noskill` control — executing one golden prompt in a
throwaway workspace seeded from the golden's `fixture`. Rules ride the plugin's migration log
rendered by `mochiko-cli` at fire (v0.107.0 end state), so the whole plugin is the unit under test;
the pre-v0.107.0 synthesized single-skill plugin and the `variants/` staging are retired
(converged 2026-09-11, runner review PASS after fixes). The produced artifact — every workspace
file the session wrote or changed — is graded two ways:

- **Scripted assertions** (deterministic, from `evals.json`: `file_exists` · `file_absent` · `contains` · `not_contains` (regex on a workspace file) · `fixture_unchanged` (every handed-in file byte-identical after the run, `except` a listed few)) — may block.
- **Rule-coverage checklist** (Haiku judge; one binary per rule with a quoted evidence span; a
  restated principle is not evidence) — advisory. Aggregation is **pass^k** over valid runs.
- A **pairwise blind A/B** (Sonnet judge, position-swapped, `pre` vs `post`) is opt-in
  `--pairwise` — position-biased in both pilots.

**Load gate per run** (mirrors the persona runner): the provisioned plugin appears in the init
event with its pinned version · a `Skill` tool_use names the skill (invocation is explicit in the
golden prompt) · no `mochiko-cli rules not delivered` halt · session model as pinned · a result
event without `is_error` · the rendered-rules pin succeeded; the control must run bare (no plugin, no skill). A run failing the gate
is recorded `invalid`, excluded from every read, listed in the report, and makes the grid exit 2.
The Skill tool, the rule-delivery binary, and python (for a skill's own checker script) are pre-allowed (`--allowedTools Skill,Bash(mochiko-cli:*),Bash(python3:*),...`)
because headless cannot answer a permission prompt. Pins per run: plugin version · `SKILL.md` sha ·
rendered-rules sha and chars (the skill's `!` lines run with the provisioned tree as root) · judge
prompt sha · session model.

## Layout

```
evals/
  run.py                      # the runner
  <skill>/
    evals.json                # 3 goldens: {id, prompt, fixture?, expected_output?, assertions[], tempts?}
    fixtures/<name>/          # workspace files a golden seeds (the artifact under review + context)
    rules.json                # rule inventory: {id, rule, class, source} (re-keyed onto log ids, rekey.md)
    preregistration.md        # ship bar + delivered-chars arithmetic — REQUIRED before a grid
    runs/<stamp>/             # artifacts, result text, streams, summary.json, report.md
    pass-report.md            # the compression pass report (compressing-skills step 7)
    baseline/                 # committed baseline results; regenerate only as a landing act
```

## Usage

```
python3 evals/run.py probe   <skill> [--arm post|pre] [--old-ref <ref>]   # R5: mechanics (1 cheap run)
python3 evals/run.py grid    <skill> [--replicates 3] [--arms noskill,post] [--old-ref <ref>] [--out <name>] [--pairwise]
python3 evals/run.py rejudge <skill> --out <name>                          # judges only, no sessions
python3 evals/run.py report  <skill> [--out <name>]                        # rebuild report.md
```

A grid persists `summary.json` after every session and resumes a named `--out` by skipping stored
(arm, golden, replicate) sessions — to redo an arm, name a new `--out`. Session model under test:
Sonnet (ruled R7). Judges: Haiku (checklist), Sonnet (pairwise, opt-in). Permission mode is
`acceptEdits`, a recorded divergence from the record's `dontAsk` wording (I2 found `dontAsk` denies
writes; R5 mandates settling flags empirically — the probe verifies it). `--local` keeps the original
`--bare` + `ANTHROPIC_API_KEY` path (metered). Requirements: a logged-in `claude` and `mochiko-cli`
on PATH (the plugin's SessionStart hook prints its version line).

## Discipline (ruled; do not relax in code review)

- The grid refuses to run without `preregistration.md` (R6/R9).
- Floor-class rules are absolute: one lost floor rule kills the arm.
- Judges are advisory — the runner exits 0 on judged degradation and nonzero only on
  mechanical failure (missing prereq, spawn failure, a failed scripted assertion on `post`, or
  an invalid run on any arm).
- Baseline results under `baseline/` are committed and regenerated only as a deliberate,
  reviewed act.
