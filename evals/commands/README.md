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
