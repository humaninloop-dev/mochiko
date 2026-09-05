# P3 — the end state: contract suite (wave 6, `cli-schema-delivery`)

**Unit:** `evals/contract/**`. **Bar:** the wave-5 V3 audit. **Depends on:** P1's `doc`, migration
`0003` and the views layout, then P2's deletions.

## 1. Run-wide no-Read (§4.1)

`assert_no_schema_read` runs in every session-bearing case already; what is not run-wide is its
**reach**. It reads stream events only, so a fallback read in the preload subagent is invisible.
- **Keying.** `schema_reads_in(raw)` parses JSONL rows and collects `tool_use` blocks: structured,
  not a grep over prose, which would fire on a model that merely names a path. A hit is `Read` or
  `NotebookRead` whose `file_path` satisfies `is_schema_path`, `Bash` whose `command` carries a
  `[\w./-]+\.yaml` token satisfying it, or `Grep` in content mode whose `path` or `glob` satisfies
  it. `is_schema_path` is unchanged: any `.yaml` under `/schemas/`, or any path ending
  `schema.yaml`. A `Glob` hit is recorded, not gated.
- **Where.** `assert_no_schema_read_evidence(staged.root)` globs `*.jsonl` in the case's own
  evidence directory — `stream*`, `transcript*`, `sidechain*-N`, every channel a session case
  captures, the preload subagent included. It replaces each case's per-events check; `preload`'s two
  become one sweep naming both states. The event-level form stays for the two fixture cases.
- **Host limb.** `render-ceiling` already renders every section of all 36 primitives, so two string
  tests fold in at zero extra render: no section render may contain `when the binary is absent` or
  `plugins/mochiko/schemas/`. The subject is the **render**, never the log — `0001-genesis.yaml`
  keeps the old wording by construction.

## 2. The `deliverables` host case (§4.2)

- **Shape.** A fourth host case; no sandbox, no session. It stages the plugin, runs `views emit
  --out <staged tmp>`, and discovers its subjects from the emitted layout — `templates/*` →
  `template <name>` and `--check`; `shelves/*` and `labels/*` → `doc <name>` — then cross-checks
  that set both ways against a constant: templates `architecture-store` · `codebase-analysis` ·
  `feature-entry` · `features-index` · `governance-intent` · `governance-surfaces` · `spec` ·
  `tasks`; shelf `architecture-shelf-backend`; registries `command-labels` · `skill-labels`.
- **Assertions.** Nineteen invocations (8×2 + 3), each exit 0 with empty stderr and non-empty
  stdout. Per `doc` name: a head line opening `mochiko-cli doc <name> · ` and carrying `· binary `,
  `· grammar `, `· plugin ` (reusing `TRIPLE_MARKERS`), then exactly `mochiko-cli doc end · <name>`.
- **Conflict with §4.2.** `template` emits neither line today — first line the document's own
  `# Title`, last `schemas: replayed from <log dir>`, verified against the built binary — and
  wrapping it is a render-shape change in P1's unit the plan does not carry. So I assert what it
  emits: exit 0 plus that footer as the closing line, head/end required of `doc` alone. Uniformity
  instead is a P1 change I will key to; say so before P1 closes.

## 3. The full run (§4.3)

- **Cases.** Four host (`hook-input`, `converted-shape`, `render-ceiling`, `deliverables`) and 78
  sandbox (2 fixture, 6 + 6 command, 3 mechanism, 30 + 30 skill, `preload`) — host first.
- **Sessions: 151, not ~80.** §4.3's figure counts cases — a delivery case is three replicates and
  `preload` is two: 2 + 18 + 6 + 3 + 90 + 30 + 2. Unchanged from wave 5; wave 6 adds a host case.
- **The transition-clause row is reworded, not removed.** `case_hook_input` asks
  `unconverted_primitive()` for a skill the hook must leave alone, and the eight prose skills
  (`analysis-iterative`, `grooming-operating-docs`, the router, four `patterns-*`,
  `testing-governance-injection`) keep it a real subject. Its justification becomes "a primitive
  with no rules is never gated"; the row and its stub stay.
- **No re-freeze of `expected-skills.json`.** Nothing in `run.py` reads a schema file at run time:
  floor sets come from the binary, `baseline_bytes` and `body_bytes_pre` are constants in the frozen
  JSON, `body_bytes_new` is `stat()` on the staged `SKILL.md`, delivered bytes from the transcript —
  so `0003` and the deletions leave every case runnable. Re-freezing is refused by design too, and a
  bar re-read after the thing it grades has landed is not a bar.
- **Read-cost columns** read against the frozen wave-5 baselines, never re-measured. The baseline
  column is a historical constant frozen at plugin 0.103.0 and 0.105.0, its sources surviving only
  in git history after v0.107.0. Delivered moves a few bytes because `0003` rewords two-arm
  sentences — measured, not predicted. `freeze_expectations.py` stays unrun as the reproducibility
  record; its `--verify` now needs a pre-v0.107.0 plugin root, and the README carries that recipe.

## 4. README

The case table gains `deliverables` (0 sessions); the totals become 82 cases / 151 sessions. The
no-Read rows say run-wide, with a paragraph on the keying that names the preload subagent. The
transition-clause paragraph takes the "no rules to deliver" justification. Criterion (2) states that
the baselines are frozen constants whose sources no longer ship, with the `--verify` recipe.
"Measured figures" takes this run's numbers — V3 item 10 failed on stale ones. "What is not here"
gains a line: the suite reads no schema file either, because none ships.

## 5. Order and assumptions

- **Order.** After P1 closes: rebuild the host binary, run `--host-only` — it validates `doc`, the
  new case, the reworded renders and all 36 floor sets free. After P2 closes: the full run, once.
- **Assumptions.** `doc`'s two lines exactly as §2.1 states; the views layout is `templates/` ·
  `shelves/` · `labels/` under `--out`; `template` keeps its output shape; I do not sweep P2's `.md`
  bodies for `plugins/mochiko/schemas/` — that is V2's audit, available as a host check on request.
