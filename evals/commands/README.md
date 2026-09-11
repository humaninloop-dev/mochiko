# Command plan-only eval

Regression instrument for command edits (`.md` + schema pairs). Ruling:
`.mochiko/brainstorms/command-plan-only-eval/record.md` (D1–D11, accepted 2026-08-27);
probe findings: `brainstorm-probe/probe-report.md`. Maintainer-side advisory tooling —
never shipped; sibling of the skill runner `evals/run.py`.

**The bet:** a command, run headless against a small branch-forcing fixture, generates
its interpreted action plan without executing (no seats, no writes — allow-list fence);
an advisory LLM judge grades which of the command's own schema rules the plan *embodies*
(never recites); regression = the within-grid diff of that coverage between the pre-edit
and post-edit pair.

## Layout

```
commands/
  run.py                 runner (uv run evals/commands/run.py <subcommand> ...)
  wrapper.md             pinned form-only elicitation wrapper (D11; sha in every run's pins)
  brainstorm-probe/      build-item-0 probe report + captured plan
  implement/             pilot command (D5)
    evals.json           goldens: id · args · fixture · control_prompt · expectations
    fixtures/<scenario>/ self-contained minimal workspaces (D4): s1-zero-gap ·
                         s2-two-gaps (planted: missing search contract + store-colliding
                         background worker) · s3-empty-args
    observable.yaml      D8 partition: 58-rule rubric + 46 declared out-of-instrument
    preregistration.md   read rule + tolerance band + F2 noise guard (grid-gating)
    runs/<name>/         plans, summary.json, report.md (gitignored or committed per run)
```

## Workflow

```sh
uv run evals/commands/run.py check-rubric implement      # D8 partition still covers the schema
uv run evals/commands/run.py check-fixtures implement    # every referenced path exists
uv run evals/commands/run.py plan-run implement s1-zero-gap   # one ad-hoc session (~$1)
# Baseline / edit evaluation (metered — ~18 sessions + judges per edit):
uv run evals/commands/run.py grid implement --old-ref <pre-edit-sha> [--control]
uv run evals/commands/run.py judge implement <run-name>
uv run evals/commands/run.py report implement <run-name>
# Rubric bucket diff alone (free, no sessions):
uv run evals/commands/run.py partition implement --old-ref <sha>
```

Editing a command pair? Run the grid with `--old-ref` at the pre-edit commit; the
report's unchanged-bucket regressions / removed-still-surfacing / added-DEAD-TEXT lines
are the instrument's three answers.

## Invariants (probe-settled)

- **No `--bare`** — it skips stored auth; isolation is `--setting-sources ""` + a
  neutral ephemeral cwd (fixture + provisioned `plugins/mochiko`, absolute
  `--plugin-dir`).
- **Blocking:** the init-event load gate (pair name+version visible) and fixture/rubric
  consistency. **Advisory:** every judge verdict (harness D2) — the runner exits 0 on
  judged degradation.
- The wrapper forces *form only* — never the phases, seats, rules, or artifacts a good
  plan would mention (D11), and forbids rule-ID citation (probe finding 7). Sessions
  cite IDs anyway (smoke finding), so the runner scrubs every known rule ID from a plan
  before judging — the coverage judge can only grade embodiment, never lexical match.
- Schema edits move the rubric by ID (mint-once + tombstones): re-run `check-rubric`
  after any implement.yaml edit; new IDs must be added to `observable.yaml` in exactly
  one bucket.

## Persona target (`agents.py`, v2 D2–D13)

The same plan-only substrate seats a **persona** instead of a command
(`primitive-eval-harness-v2`, accepted 2026-09-08; vocabulary: `../README.md`). Data lives
under `evals/agents/<persona>/`: `evals.json` (goldens: `card` + `fixture` + `tempts`),
`fixtures/<name>/`, `rules.json` (the minted rubric), `preregistration.md`, `judge-readings.md` (the kit's
pre-registered readings, carried into every judge call and pinned as `readings_sha256`; optional), `runs/`.

```sh
uv run evals/commands/run.py agent-mint staff-engineer --old-ref <pre-sha>   # rules.json over pre ∪ post
# hand-partition the drafts (plan-observable | out-of-instrument + why), split compounds, drop `draft`
uv run evals/commands/run.py agent-check staff-engineer      # completeness · partition · temptation
uv run evals/commands/run.py agent-plan-run staff-engineer g1-decided-card --arm post
uv run evals/commands/run.py agent-prune staff-engineer      # one-time nopersona pass (untagged ids only)
uv run evals/commands/run.py agent-grid staff-engineer --replicates 3       # pre/post, persona alone
uv run evals/commands/run.py agent-judge staff-engineer <run-name>
uv run evals/commands/run.py agent-report staff-engineer <run-name>
uv run evals/commands/run.py agent-label-sheet staff-engineer <run-name>   # hand-label 24 pairs (I9)
uv run evals/commands/run.py agent-calibrate staff-engineer <run-name> --labels evals/agents/staff-engineer/runs/<run-name>/calibration-sheet.json
```

Invariants specific to the persona target (probe-settled 2026-09-08):

- **Seating:** `--agent mochiko:<persona>` from the fixture cwd — the persona *is* the session;
  no nested lead. `--append-system-prompt` reaches the seat, so the pinned
  `evals/agents/wrapper.md` rides the same channel as the command wrapper. The plugin tree is
  provisioned **outside** the workspace cwd (a sibling temp dir): seated inside it, the persona
  Read its own skills' files through the fence (smoke finding), which broke "persona alone".
  `--permission-mode acceptEdits` (the sibling ruling's probe-settled mode) is passed and pinned.
- **Fence as roster:** `--tools Read,Grep,Glob` — write, shell, spawn, and skill-load tools are
  absent, not merely denied (D3 + fold C1; `--disallowedTools` left `Workflow` and the
  messaging tools in the roster). The run records any roster surplus and any breach.
- **Model:** `--model opus` explicit on every arm, control included — `--model` overrides the
  persona's frontmatter pin under `--agent` (R5), so the pin is never relied on.
- **Rubric:** minted from the persona body over the union of both refs; ids are stable by
  source sha across re-mints; each unit → ≥ 1 claim or `not_claims`; each claim exactly one
  partition value. `model_native` tags carry forward (R3); `untempted` must be explicit (I5).
- **Read-trace:** the stream's `tool_use` Read/Grep/Glob events with paths, saved per run;
  the wrapper also asks for a `FILES-READ:` line as the self-report.
- **Judge:** embodiment only — a plan that restates a standard as a principle reads as
  `absent` (recitation is not embodiment, fold I9); calibration set + agreement bar are the
  pilot pre-registration's.
