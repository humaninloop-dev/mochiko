# P3 — contract suite + measurements plan (wave 3)

## 0. Host probes already run (read-only, 2026-09-04)

P1's move and legend are on disk. Against `--plugin-root plugins/mochiko` (plugin 0.103.0, binary 0.1.0 · grammar 1..1) the seven blocks render **10,513 chars** — preamble 2,055 post-legend · roles 1,582 · reserved 1,435 · tools 1,669 · ways-of-working 1,924 · boundaries 725 · fail-conditions 1,123 — against the pre-conversion baseline **12,819**: a **−18 %** delta with 2,306 chars of headroom under abort criterion (2). The seven `class: floor` ids are exactly the seven the bar names, and the fail-conditions end line reports `4 rules`, matching the preamble pin. `wc -m` counts characters, not bytes (2,055 vs 2,102 on the preamble), which is the measure `evals/run.py` already treats as canonical.

## 1. Cases

Nine. `[fixture]` = the wave-1 probe plugin, unchanged; `[staged]` = a full copy of `plugins/mochiko/` (log and hooks included) at the case root, loaded with `--plugin-dir`, command `/mochiko:brainstorm`. Sessions keep the wave-1 argv shape (`sonnet`, `--setting-sources ''`, stream-json) with `--max-turns 2`; two append `--settings '<json>'`, which the CLI applies even under an empty `--setting-sources`. The binary's directory is prepended to `PATH` except where the row says off.

| case | sessions | staging | gating assertions | reported |
|---|---|---|---|---|
| `hook-input` | 0 | committed captures, host-side | §3 | — |
| `render-ceiling` | 0 | direct binary | every section render of every converted primitive < 30,000 chars (converted = the `.md` carries a `` !`mochiko-cli rules `` line, the hook's own truth source) | largest render; optionally the corpus-wide max |
| `absence` `[fixture]` | 1 | binary off `PATH` | wave-1 set, minus the resolved `pending` | — |
| `skew` `[fixture]` | 1 | skew log via `MOCHIKO_MIGRATIONS` | wave-1 set, unchanged | — |
| `brainstorm-delivery` | 3 | `[staged]`, binary on | seven head lines and seven end lines present in the first user message · each end-line count matching its preamble `sections` row, `fail-conditions` = 4 · no `` !`mochiko-cli `` literal · no `Permission to use Bash has been denied` · no Read of `plugins/mochiko/schemas/` or `*schema.yaml` · the `SessionStart` presence line present · the dependency hook's presence line present · the init event's `slash_commands` carries all six `mochiko:*` commands | read-back metric (§2); delivered chars (§4) |
| `brainstorm-absence` | 1 | `[staged]`, binary off | no model turn · `cargo install mochiko-cli` in the session output · no schema Read | which channel carried it; hook block or `!` line first |
| `brainstorm-skew` | 1 | `[staged]`, **the staged plugin's own `migrations/`** swapped for the `grammar: 99` file — the `!` lines pass `--plugin-root`, which beats `MOCHIKO_MIGRATIONS`, so the wave-1 env lever cannot reach them | no model turn · `grammar 99` and `cargo install mochiko-cli` in the session output · no schema Read | which limb halted first |
| `brainstorm-hooks-off` | 1 | `[staged]`, binary off, `--settings '{"disableAllHooks":true}'` | no model turn · `<local-command-stderr>` carries `mochiko-cli` and `command not found` | — |
| `brainstorm-policy` | 1 | `[staged]`, binary on, `--settings '{"disableSkillShellExecution":true}'` | none — recorded only, per D8 | halted on the placeholder or proceeded; what the hook delivered |

The wave-1 `absence` case's `pending` entry ("the install line reaches the model") is deleted and its subject asserted in `brainstorm-absence`; its docstring says where it went, so the resolution is readable rather than silently dropped.

**Halt channel, keyed only after measurement.** Under branch-B hooks the dependency hook's exit-2 block fires before expansion, so wave-1's `<local-command-stderr>` channel may never appear in `brainstorm-absence` or `brainstorm-skew`, and the stream shape of a `UserPromptExpansion` exit-2 block is measured nowhere. I run each of those two once as a shape-discovery probe in Phase 3, record the shape, then key the assertion to it — the wave-1 discipline. Until then they search the union of transcript, process stdout and process stderr and report the channel; a tolerant assertion is not the shipped form.

## 2. The read-back metric — exact scoring

The probe topic reads: *CONTRACT PROBE — before your first question reply with exactly one line `FLOOR: <every class: floor rule id you were delivered, comma-separated>` then stop.*

Per replicate: take the final assistant text, find the first line matching `^\s*\**FLOOR:`, take everything after the colon, split on commas, strip whitespace, backticks and surrounding quotes, drop empties. The replicate **passes iff the token set equals exactly** `{brainstorm.user-record-acceptance, brainstorm.author-grader-default-fail, brainstorm.transport-floor, brainstorm.fail.record-unaccepted, brainstorm.fail.unreviewed-no-waiver, brainstorm.fail.survivor-undispositioned, brainstorm.fail.index-mismatch}` — all present, nothing else, no near-miss credit. A missing `FLOOR:` line is a failed replicate, not a harness error.

**Pre-registered bar: 3/3**, fixed here before the first run and unchanged after it. **Reported, never gating** (D8): it lands in `verdict.json` as `read_back` with the per-replicate token sets and the score, prints in the summary, and never sets a non-zero exit code. The lead evaluates abort criterion (1) from it.

## 3. The hook-input case (no sandbox, cheapest gate)

Each committed capture is fed on stdin to `plugins/mochiko/hooks/scripts/dependency-halt.sh` and `session-start.sh` on the host, with `CLAUDE_PLUGIN_ROOT` pointed at a staged copy and `PATH` controlled per row.

| row | stdin | binary | expected |
|---|---|---|---|
| unconverted command | UPE, `command_name: mochiko:specify` | absent | exit 0, silent |
| unconverted skill | PreToolUse, `skill: mochiko:review-brainstorm` | absent | exit 0, silent — the `Skill` limb is a tested no-op at wave 3 |
| foreign namespace | UPE, `command_name: other:thing` | absent | exit 0, silent |
| converted, absent | UPE, `command_name: mochiko:brainstorm` | absent | exit 2, stderr carries `cargo install mochiko-cli` and `/mochiko:brainstorm` |
| converted, absent | PreToolUse, a converted stub skill | absent | exit 0, stdout parses as JSON with `permissionDecision: "deny"` |
| converted, present | UPE, `command_name: mochiko:brainstorm` | present | exit 0, stdout parses as JSON with a non-empty `additionalContext` |
| SessionStart | SessionStart capture | present | exit 0, stdout matches `mochiko-cli <semver> · grammar <a>..<b>` and carries `in range` |
| SessionStart | SessionStart capture | absent | exit 0, stdout carries `cargo install mochiko-cli` |

The captures are the **shape** source; the case substitutes only the `command_name` / `skill` value per row, so P2 owns the field set and I own the matrix. The skill rows need a converted skill to exist, so the case writes a one-line stub `SKILL.md` into its own staged copy rather than converting anything real. If P2's set carries no `SessionStart` sample I synthesize one from F14's documented field set and label it synthesized in the fixture README; `session-start.sh` reads only `cwd` from stdin, so the two SessionStart rows would otherwise go untested.

## 4. Measurements

- **Delivered read cost.** Sum the seven blocks as they appear in `brainstorm-delivery`'s first user message, head line through end line inclusive, plus the `SessionStart` and dependency-hook lines. Cross-check against a direct render sum in the sandbox; on a disagreement the transcript is truth and the gap is reported. Compare to **12,819** (14,349 with labels). Host expectation: 10,513 plus roughly 150 of hook lines.
- **Store latency.** Timed inside the sandbox in one `sh -c`, not across `sbx exec`, so the figure is the binary and not the transport: `date +%s%N` around ten runs of each section; mean and max per section, plus the seven-render whole-fire figure.
- **Log cost in the plugin.** `du -sb plugins/mochiko` and `du -sb plugins/mochiko/migrations` after the move; before = the difference. Host today: 2.3 M total, 616 K log.
- **Abort criteria.** Both evaluated with their numbers stated plainly in my report — (1) read-back below 3/3, (2) delivered read cost above 12,819. The ruling is the lead's.

## 5. Staging, binary placement, and scope

`stage()` gains a `source` argument so one function stages either the fixture or `plugins/mochiko/`; the copy is `shutil.copytree` into the case root, which is also the evidence directory — about 2.3 MB per staged case, seven per full run. The binary is the sandbox build at `/home/agent/mochiko-target/release/mochiko-cli`, on `PATH` by prepending its directory (the D4 install shape in miniature, replaced by `cargo install mochiko-cli` after the publish). `sandbox_path()` already verifies the binary's absence from the sandbox `PATH` and skips rather than lying if it is present.

I touch `evals/contract/run.py`, `evals/contract/README.md`, and additive files under `evals/contract/fixture/`. Nothing under `plugins/mochiko/`, no schema. The suite dispatches no agent and grades no content (GI-019). Exit codes stay 0 / 1 / 3, the case list still prints on every path, and every case still writes `stream.jsonl`, `argv.txt`, `script.sh`, `stderr.txt` and `verdict.json` to disk.

## 6. README changes

`evals/contract/README.md` gains the nine-case list in place of the two-case section; the staged-versus-fixture distinction and why the brainstorm skew case swaps the plugin's own log rather than setting `MOCHIKO_MIGRATIONS`; the read-back metric with its bar and its never-gating status; the hook-input case and the sanitization caveat on its captures; the two `--settings` levers; and the measured read-cost and latency figures. The wave-1 "two cases" and "pending assertion" sections are rewritten, not appended to.

## 7. Run order

1. **Phase 2, on "approved":** case scaffolding, the hook-input case, `render-ceiling`, the README. Run the two session-free cases on the host — they need no sandbox and no P2 close.
2. **Phase 3, on "P1 and P2 closed":** shape-discovery probes for `brainstorm-absence` and `brainstorm-skew`; record the halt channel; key those two assertions to it.
3. **Phase 4:** full run — session-free cases, then the two fixture cases, then the five brainstorm cases; measurements from the same run; evidence on disk per case.
4. **Report:** the tally, the two abort-criteria numbers, the measurements, deviations with reasons.

## 8. Lead rulings folded (approval, 2026-09-04) — these override §0, §2 and §7 above

1. **Units.** The 12,819 baseline was measured with `wc -c`, so it is **bytes**. Abort criterion (2)
   compares bytes to bytes. Post-legend the seven blocks are **10,700 bytes / 10,513 chars**, a
   **−16.5 %** byte delta with 2,119 bytes of headroom. Every table reports bytes and chars in
   separate columns; §0's "−18 %" is the char figure and is not the criterion.
2. **Replicate rule.** The first delivery replicate is scored like any other. It is discarded and
   replaced only where its failure is demonstrably turn-cap exhaustion — no assistant text at all
   and turns exhausted — in which case `--max-turns` rises to 3 and three fresh replicates are run
   and scored, with the discarded probe disclosed. A replicate that produced any `FLOOR:` line is
   scored and never re-run.
3. **Read-back tokens.** `brainstorm.<id>` bare or wrapped in backticks is accepted; every other
   decoration is a miss. This narrows §2's strip list — quotes are no longer stripped.
4. Shape-discovery probes: approved, Phase 3, one run each, the final assertion keyed to what they
   measure. A synthesized `SessionStart` capture is an approved fallback, but a real one from P2 is
   preferred if it lands before `hook-input` runs.
5. The report goes to `wave3-reports/p3-suite.md`. Nothing is committed.

## 9. Deviations from §1, disclosed at build (Phase 2)

- **A tenth case, `converted-shape`** (host, no session). A `.md` that enumerates six sections when
  the schema declares seven delivers six well-formed blocks, and no session assertion in §1 calls
  that a failure — they grade what arrived, not what was asked for. The case compares each
  converted `.md`'s `!` lines against the section list its own preamble render declares, plus the
  presence of the `Bash(mochiko-cli *)` grant. Additive, free, and it moves a class of failure off
  the metered path. Host cases are now three, sessions still nine.
- **`hook-input` gained two rows beyond §3's eight**: the dependency hook against an out-of-range
  log (its only gate other than absence, and it needed no session to test), and `SessionStart`
  against a settings file carrying `disableSkillShellExecution` (the unsupported-environment notice
  GI-020 obliges). Thirteen assertions, all green.
- **The no-Read assertion was too narrow to survive staging.** It matched
  `plugins/mochiko/schemas/` and a `schema.yaml` suffix, and the staged copy's own
  `schemas/brainstorm.yaml` matched neither — the assertion would have passed a run that did the
  one thing no-fallback exists to rule out. It now matches any `.yaml` under a `schemas/`
  directory, and a Bash command naming such a path counts as the same failure.
- **No synthesized `SessionStart` capture was needed.** P2 committed a real one, which is the
  preferred path under ruling 4.
