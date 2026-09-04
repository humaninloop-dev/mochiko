# Wave 3 — pilot build plan (lead-drafted referent)

**Ruling home:** `record.md` D3 as amended (per-section `!` lines, head + end confirmation,
`allowed-tools` grant, positive-confirmation halt), D4 (user-installed binary), D5 (version
contract), D7 (hooks: `SessionStart` presence; dependency-halt on the plugin's own commands
and skills; absence-only gating; 5-second timeout; fail-open when the hook cannot run), D8 as
amended (contract suite in the sandbox; deterministic set gates, read-back metric reported,
per-primitive no-Read scope), D9 (wave 3 scope and the pilot abort criteria), the wave-2 run's
additions (the log moves into the plugin at `plugins/mochiko/migrations/`; `primitive-edits.md`
globs; `README.md` re-authored), and governance v3.0.0 (GI-020 as superseded — transition
clause; GI-012 gate 6; GI-004 audit before the bump). **Wave open:** user-confirmed
2026-09-04 with the four §9 rulings as recommended — Q-A branch A if the platform allows it,
else B · Q-B follow D3, no hand pin, criterion 3 gains its converted branch under PATCH v3.0.1
· Q-C wave 3 now, the publish later on the user's word · Q-D the read-back bar 3/3 as stated. **Floor:** sound loop tripped (judgment-authored
plugin primitives, product code, and a test suite) — seats produce on lead-approved plans,
fresh validators review, the user accepts; transport floor topology lane fired → **sequential
single pen-holder**, one producer seat writing at a time; message legs held.

**Done condition (fixed):** `brainstorm` fires from the migration log the plugin carries, with
no schema file read: the seven `!` lines deliver, the halt clause and the hooks are in place, the
contract suite's brainstorm cases are green on their deterministic set with the read-back metric
reported against its pre-registered bar, the read-cost delta and the store latency are measured
and recorded, the pilot abort criteria are evaluated and stated, every audit is PASS, the strip
entry exists, and the wave lands as `plugin.json` 0.104.0 with the landing ritual complete
(`CHANGELOG.md` · `marketplace.json` · `DECISIONS.md` · `BACKLOG.md` · `ROADMAP.md` · record
wave-3 section · brainstorms index). The four D8 layers green before the bump; a SKIPPED contract
suite blocks it (GI-012 as amended).

---

## 0. Measured before the wave (lead, host, 2026-09-04)

| figure | value |
|---|---|
| rendered `brainstorm`, all seven blocks (preamble + six sections) | **10,088 chars**; largest block 1,952 (`ways-of-working`), smallest 743 |
| raw read the `.md` obligates today | `brainstorm.yaml` 10,323 + `common.yaml` 2,496 = **12,819 chars** (+ `command-labels.yaml` 1,530 if counted: 14,349) |
| read-cost delta, pre-pilot | **−21 %** (−30 % against the three-file figure) |
| store latency, one section render, host (Apple silicon, release build) | **≈ 30 ms** wall-clock; seven renders per fire ≈ 0.2 s |
| migration log | `0001-genesis.yaml` 618,122 bytes (the plugin gains this file) |
| Bash inline ceiling (F12e) | ≈ 30,000 chars; the largest block sits at 6.5 % of it |
| `class: floor` rules in `brainstorm` | 7 — `user-record-acceptance` · `author-grader-default-fail` · `transport-floor` · the four `fail.*` |
| sandbox `claude-mochiko` | reachable; Claude Code 2.1.259; `cargo` present; `mochiko-cli` absent from `PATH`; prior sandbox build at `/home/agent/mochiko-target/release/mochiko-cli` |

The sandbox figures for the same rows are re-measured by P3 and land in its report.

## 1. Scope and ownership (three sequential producer seats)

| seat | owns (exclusively while it holds the pen) | delivers |
|---|---|---|
| **P1 — store move + crate** | `migrations/` → `plugins/mochiko/migrations/` (the log and its README, by `git mv`), `crates/mochiko-cli/src/render.rs`, `crates/mochiko-cli/tests/*.rs` (re-path only, plus the legend test), `.github/workflows/ci.yml` (path filter), `.claude/rules/mochiko/rust-cli.md` (the two "until wave 3" clauses) | the log inside the plugin; every crate test green against the new path; the preamble render's **legend block**; CI filter re-pathed |
| **P2 — plugin side** | `plugins/mochiko/commands/brainstorm.md`, `plugins/mochiko/hooks/hooks.json`, `plugins/mochiko/hooks/scripts/*.sh`, `.mochiko/strips/brainstorm.md` (append), `.claude/rules/mochiko/primitive-edits.md` (`paths` globs + the ruled criterion-3 branch), `README.md` (install section) | the re-pointed command; the three hooks; the strip entry; the rules-file edits; the README |
| **P3 — contract suite + measurements** | `evals/contract/run.py`, `evals/contract/README.md`, `evals/contract/fixture/**` (additive) | the brainstorm cases (§4), the pending assertion resolved, the measurements (§5), the abort-criteria evaluation, the suite run with evidence |

Sequence P1 → P2 → P3. P2 needs the log at its plugin path (P1); P3 needs both. A later seat
touches an earlier seat's file only as the pen-holder of its turn, additively, disclosed in its
report. **No file under `plugins/mochiko/` other than those named for P2 changes this wave**;
`plugins/mochiko/schemas/*.yaml` and every `skills/*/schema.yaml` stay byte-identical (the
transition clause: unconverted primitives keep reading them).

## 2. P1 — the store move and the render legend

- **Move:** `git mv migrations plugins/mochiko/migrations`. The README moves with the log (it
  documents the file a user can now find in their plugin cache; nothing loads it). No repo-root
  `migrations/` remains. The genesis file is byte-identical after the move (`git mv`, no
  regeneration).
- **Resolution unchanged:** `--log-dir` › `--plugin-root <root>/migrations` › `MOCHIKO_MIGRATIONS`
  › `./migrations`. Every command `!` line and every hook passes `--plugin-root
  "${CLAUDE_PLUGIN_ROOT}"`, so the plugin version resolves too. Maintainer invocations become
  `--plugin-root plugins/mochiko` alone.
- **Tests re-pathed:** `tests/fidelity.rs` (`repo_root().join("plugins/mochiko/migrations")`,
  the regenerate hint in its failure message), `tests/matrix_similar.rs` (the allowlist walk
  still resolves `scripts/similar-rules-allowlist.yaml` from the new log dir's ancestors — assert
  it), `tests/views.rs` and any other test that names the repo path. The `tests/cli.rs` fixtures
  build their own roots and need no change beyond what the seat finds.
- **Legend block:** the `preamble` render gains one fixed block after `pins`, headed `legend`,
  carrying the reading grammar the `.md` no longer restates — six lines, verbatim:
  ```
  legend
  - class: floor is always delivered whatever its when:; when: gates when the obligation applies, never whether it reaches you.
  - kind: names what a rule is — constraint (the default) · duty · gate · reservation · binding · bound · routing · fail · latitude.
  - when: binds a rule only where its terms hold against the conditions block above.
  - enforces: on a kind: fail rule names the rules it is the end-state contrapositive of.
  - pointer: binds you to that skill's procedure — referenced, never restated.
  - extends: is already resolved in this render; the rule's own id stays the citable id.
  ```
  Golden-tested in `tests/render.rs`; the preamble's end line still reports `0 rules`.
- **CI:** `.github/workflows/ci.yml` path filters swap `migrations/**` for
  `plugins/mochiko/migrations/**` (both `push` and `pull_request`). No other workflow change.
- **Rules file:** `.claude/rules/mochiko/rust-cli.md` — the `paths` list drops the repo-root
  `migrations/**` glob; the "at the repo root `migrations/` until then" clause is struck.
- **Check:** `cargo test --all` green, `fmt --check` and `clippy -D warnings` clean;
  `mochiko-cli migrate validate --plugin-root plugins/mochiko` reports 0 rejecting;
  `mochiko-cli migrate status --plugin-root plugins/mochiko` prints the same state hash as before
  the move (record it in the report, before and after).

## 3. P2 — the re-pointed command and the hooks

### 3.1 `commands/brainstorm.md` — the exact shape

Frontmatter gains one key; nothing else in the frontmatter changes:

```yaml
allowed-tools: Bash(mochiko-cli *)
```

`## Identity & Mission` is unchanged, byte for byte. The `## Rules — load the schema first`
section is replaced whole by a section headed **`## Rules — delivered by mochiko-cli`** whose
body is: one short halt-clause paragraph, then seven `!` lines, then nothing else. The halt
clause, verbatim:

> Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
> carries — one block per section. Every block opens with a version-triple line
> (`mochiko-cli rules brainstorm · section <id> · binary <v> · grammar <g> · plugin <p>`) and
> closes with an end line (`mochiko-cli rules end · brainstorm · <id> · <N> rules`). **Proceed
> only when every block carries both lines in that exact shape, from whichever channel delivered
> it — this slot or the plugin's dependency hook.** Anything else — an error, an empty block, the
> placeholder `[shell command execution disabled by policy]`, a file-path-plus-preview stub — is
> a failure to deliver: surface `mochiko-cli rules not delivered: <what was seen>` and halt. Never
> Read a schema file instead; there is no fallback. The `legend` in the preamble block is the
> reading grammar; a `pointer:` binds you to that skill's procedure, referenced never restated.

The seven lines, one per block, in this order — `preamble`, then the six sections in schema
order:

```
!`mochiko-cli rules brainstorm --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules brainstorm --section brainstorm.sec.roles --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules brainstorm --section brainstorm.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules brainstorm --section brainstorm.sec.tools --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules brainstorm --section brainstorm.sec.ways-of-working --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules brainstorm --section brainstorm.sec.boundaries --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules brainstorm --section brainstorm.sec.fail-conditions --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
```

The `2>&1` is the fixture's proven shape (wave 1: the harness carries a failing line's stderr
either way; the binary writes nothing to stderr on success). `${CLAUDE_PLUGIN_ROOT}` substitutes
in command bodies (F12b). The grant names the bare binary (F12a); the path argument sits under
the wildcard.

`## Adaptive Goal Protocol` — steps 1 and 2 unchanged. Step 3 per D3 (counts printed by the CLI,
never hand-pinned) becomes:

> 3. **Not done — default FAIL:** the `kind: fail` rules of `brainstorm.sec.fail-conditions` —
>    their count is the `kind: fail` line under `pins` in the preamble block — any one standing
>    fails the run. A fail-conditions block whose end-line count disagrees with that pin is the
>    delivery out of sync: halt and surface it before closing.

*(If the user rules §9 Q-B the other way, the line keeps "the 4 rules of `kind: fail`" beside the
pin reference.)*

**Strip entry** (`.mochiko/strips/brainstorm.md`, newest-first, stamp `[v0.104.0]`): one
supersession-by-ruling entry for the retired Rules block — disposition *superseded → the seven
`!` lines + the halt clause (record D3 as amended, `DECISIONS.md` 2026-09-03; the reading grammar
now printed by the CLI as the preamble `legend`)*; verbatim content of the retired block; kept
deliberately: Identity & Mission, Entry, Goal; consumers assessed: none shared (the block was the
command's own). A second entry if Q-B removes the hand-pinned count.

### 3.2 `hooks/hooks.json` and the three scripts

Shipped under `plugins/mochiko/hooks/`. Every hook: `type: command`, a 5-second `timeout`, the
command quoted as `"${CLAUDE_PLUGIN_ROOT}/hooks/scripts/<name>.sh"`. Scripts are POSIX `sh`,
executable bit set, no dependency beyond `sh`, `grep`, `command -v`, and `mochiko-cli` itself —
**never `jq`** (not on every machine). Stdin JSON is parsed with a `sed`/`grep` extraction of the
one or two fields each script needs; the extraction is tested by P3 against captured real hook
input, committed under `evals/contract/fixture/hook-input/`.

- **(a) `SessionStart`** → `session-start.sh`, no matcher (fires on startup, resume, clear,
  compact). Prints to stdout, one to three lines, as context:
  - present + in range: `mochiko-cli <binary version> · grammar <a..b> · plugin <p> · log grammar <g> · in range` (from `mochiko-cli --version` and `mochiko-cli migrate status --plugin-root "${CLAUDE_PLUGIN_ROOT}"`).
  - absent: `mochiko-cli is not installed — the mochiko plugin depends on it; every converted command halts until it is. Install: cargo install mochiko-cli`.
  - out of range: the binary's own D5 stderr line, verbatim.
  - best-effort policy detection: if `disableSkillShellExecution` appears in `~/.claude/settings.json` or `./.claude/settings.json`, one line naming the environment as unsupported (GI-020). Detection failing silently is fine; a false negative costs nothing.
  Always exit 0. Never blocks.
- **(b) dependency-halt** → `dependency-halt.sh`, registered twice: `UserPromptExpansion`
  matched on the namespaced command name (`mochiko:*`) and `PreToolUse` matched on `Skill`.
  The script:
  1. Reads the primitive name from stdin — `command_name` (`mochiko:<cmd>`) or
     `tool_input.skill` (`mochiko:<skill>`). A name outside the `mochiko:` namespace → exit 0.
  2. **Converted check:** the primitive's own file carries a `!`mochiko-cli rules` line
     (`${CLAUDE_PLUGIN_ROOT}/commands/<cmd>.md` or `skills/<skill>/SKILL.md`). Not converted →
     exit 0, no output — the transition clause: an unconverted primitive reads its snapshot and
     is never gated. No list to maintain; the `.md` is the truth.
  3. **Presence:** `command -v mochiko-cli` fails → stderr `mochiko-cli is not installed — /mochiko:<cmd> cannot run without it. Install: cargo install mochiko-cli`, exit 2.
  4. **Range:** `mochiko-cli migrate status --plugin-root "${CLAUDE_PLUGIN_ROOT}"` exits 3 → its
     stderr forwarded, exit 2.
  5. **Delivery:** per §9 Q-A. Branch A (the hook can see the expanded prompt): if the prompt
     already carries the preamble's head line, print one line `mochiko-cli present · rules
     delivered by the command's own render`; otherwise print every block, all seven, to stdout
     (the D3 second channel). Branch B (it cannot): print the one presence line only; the
     policy-placeholder environment halts on the prose clause, as GI-020 declares it unsupported.
  For `PreToolUse` the same outcomes are emitted in the tool's JSON shape (`permissionDecision:
  "deny"` with the reason, or `additionalContext`). At wave 3 no skill is converted, so the
  `Skill` limb is a tested no-op: a mochiko skill call proceeds with the binary absent.
- **(c) maintainer hook** — `.claude/settings.json` in this repo, never shipped: `PostToolUse`
  on `Edit|Write` whose `file_path` is under `plugins/mochiko/migrations/`, running
  `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` and printing the report.
  Advisory; exit 0 always.

Wave-0 facts the scripts rely on (F12c): `UserPromptExpansion` receives `command_name`
namespaced and its stdout reaches the model; exit 2 blocks before any model turn with the stderr
text as the result; `PreToolUse` on `Skill` receives `tool_input.skill` namespaced, its
`additionalContext` reaches the model, its `deny` errors the call; `SessionStart` stdout reaches
the model; `${CLAUDE_PLUGIN_ROOT}` expands in hook commands.

**F14 — platform-doc verification (2026-09-04, `claude-code-guide` dispatch; pages
`code.claude.com/docs/en/hooks.md`, `hooks-guide.md`, `settings-reference.md`, `skills.md`,
`plugins.md`, `cli-reference.md`):**
- `hooks/hooks.json` is `{"hooks": {<Event>: [{matcher?, hooks: [{type: "command", command,
  timeout}]}]}}`; `timeout` is **seconds** (default 600 for command hooks); no per-hook
  `description` field; `${CLAUDE_PLUGIN_ROOT}` substitutes in `command`.
- `SessionStart` stdin: `session_id` · `hook_event_name` · `cwd` · `permission_mode` ·
  `agent_id` · `agent_type` · `source` (`startup|resume|clear|compact|fork`). Context = plain
  stdout. Exit 2 shows stderr to the user and **execution continues** — it cannot block.
  `CLAUDE_ENV_FILE` is a file path the hook may append `KEY=value` lines to.
- `UserPromptExpansion` stdin: `hook_event_name` · `command_name` (namespaced) · `prompt` ·
  `session_id` · `cwd` · `permission_mode`. Matcher matches the command name. Block: exit 2 +
  stderr, or JSON `{"hookSpecificOutput": {"decision": "block", ...}}`. Context on exit 0:
  JSON `{"hookSpecificOutput": {"additionalContext": "..."}}` (wave 0 saw plain stdout reach the
  model too; the JSON form is the documented one — use it). **Whether `prompt` is the raw user
  line or the expanded text is not stated** — the event blocks *the expansion*, which reads as
  pre-expansion. P2 measures it: capture the real stdin on a fire of the converted `brainstorm`
  (a sandbox or host run with a capture line in the script) and commit the capture under
  `evals/contract/fixture/hook-input/`. **Branch rule (Q-A as ruled):** the rendered preamble
  head line present in `prompt` → branch A is possible; absent → branch B, and the script never
  injects rules (an always-inject default would double-deliver on every fire).
- `PreToolUse` with `matcher: "Skill"` stdin: `tool_name: "Skill"` · `tool_input.skill`
  (namespaced) · `tool_input.arguments` · `tool_input.input_context`. Deny: JSON
  `{"hookSpecificOutput": {"permissionDecision": "deny", "permissionDecisionReason": "..."}}` on
  exit 0, or exit 2 + stderr. Context: `additionalContext` in the same object.
- Failure semantics: command not found, timeout, or a non-zero exit other than 2 → non-blocking
  error, the action proceeds (fail-open, D7's floor confirmed); logged in the transcript.
- `disableSkillShellExecution` blocks inline `!` lines only and **does not disable hooks**;
  `disableAllHooks: true` is the separate global switch (the `brainstorm-hooks-off` case uses
  it; the `brainstorm-policy` case uses the former with hooks on).
- `${CLAUDE_PLUGIN_ROOT}` substitutes in skill/command content and `allowed-tools` before the
  model sees it; `Bash(mochiko-cli *)` matches the bare command with any arguments.
- Headless: `--plugin-dir <dir>` loads the plugin's `hooks/hooks.json` for the session;
  `claude plugin marketplace add <owner/repo>` + `claude plugin install <name>@<marketplace>
  --scope user` is the persistent form.
- Not documented: a per-hook `description`; the complete stdin schema per event.

### 3.3 Rules-file and README edits

- `.claude/rules/mochiko/primitive-edits.md`: `paths` gains `plugins/mochiko/migrations/**` and
  `plugins/mochiko/hooks/**` (the wave-3 obligation, ledger); criterion 3 gains the converted
  branch per §9 Q-B; the "Schema data files" paragraph gains one sentence: a converted command's
  rules come from the migration log, and a log edit is a migration file (grammar in the log's
  README), never an in-place edit.
- `README.md`: the Install section becomes two steps — the plugin (unchanged lines) and the
  binary — and states plainly that the plugin depends on the binary from this version, that
  converted commands halt without it, and that until the first crates.io release the install
  line is `cargo install --git https://github.com/humaninloop-dev/mochiko mochiko-cli` (the
  ruled `cargo install mochiko-cli` follows the wave-2 tail publish). The "template-schema CLI
  (optional)" section is rewritten under that: what the binary serves (rules and templates from
  the migration log), the `rules`/`template`/`migrate` usage, and no "raw YAML when absent"
  sentence anywhere.

## 4. P3 — contract cases (brainstorm) and the pre-registered bar

The plugin under test is a **staged copy of `plugins/mochiko/`** (the whole plugin, log and hooks
included) under `evals/.work/contract-<case>-<id>/mochiko/`, loaded with `--plugin-dir` (D8 as
amended); the binary is the sandbox build at the sandbox-local target dir, placed on `PATH` by
the case (D4 in miniature; `cargo install mochiko-cli` replaces the build step after the
publish). The fixture `probe-plugin` and its two wave-1 cases stay.

| case | runs | asserts (deterministic, gating) | reports (never gating) |
|---|---|---|---|
| `brainstorm-delivery` | 3 replicates, sonnet, `--max-turns 2`, prompt `/mochiko:brainstorm <probe topic>` where the topic text instructs: *"CONTRACT PROBE — before your first question reply with exactly one line `FLOOR: <every class: floor rule id you were delivered, comma-separated>` then stop."* | all seven head lines and all seven end lines present in what the model saw, each end line's count matching the pin (`4` for fail-conditions) · no `!`mochiko-cli` literal passed through · no `Permission to use Bash has been denied` · **no Read of any schema file** (`plugins/mochiko/schemas/`, any `schema.yaml`) · the `SessionStart` presence line present · the dependency hook's line present · the init event's slash-command list carries all six `mochiko:*` commands (the manifest quirk re-verified) | **read-back metric:** the `FLOOR:` line names all seven floor ids exactly and nothing else; **pre-registered bar: 3/3 replicates** (D8 working shape, stated here before the run) |
| `brainstorm-absence` | 1, binary off `PATH` | no model turn (`num_turns` 0) · the install line `cargo install mochiko-cli` present in the session output — from the hook's exit-2 block or the harness's `<local-command-stderr>`, whichever fires first (recorded which) · no schema Read | the `SessionStart` install line present (which channel carried it) |
| `brainstorm-skew` | 1, the staged plugin's `migrations/` replaced by a `grammar: 99` file | no model turn · the D5 message (`grammar 99`, `cargo install mochiko-cli`) present · no schema Read | which limb halted first (hook range check vs the `!` line) |
| `brainstorm-hooks-off` | 1, binary off `PATH`, hooks disabled by setting | no model turn · the harness `<local-command-stderr>` carries `mochiko-cli: command not found` (D7 fail-open → the prose/harness path still halts) | — |
| `brainstorm-policy` | 1, `disableSkillShellExecution` set, hooks on | — (recorded only, D8) | did the run halt on the placeholder, or proceed on the hook's rules (Q-A branch A)? |
| `render-ceiling` | 0 sessions (direct binary) | every section render of every **converted** primitive < 30,000 chars; `preamble` < 30,000 | the largest, in chars |

The **pending assertion** from wave 1 ("the install line reaches the model") resolves into
`brainstorm-absence` as stated. Evidence per case as at wave 1 (`stream.jsonl`, `verdict.json`,
the staged plugin, the hook stdin captures). Exit codes unchanged (0 / 1 / 3). The case list
prints on every path.

## 5. P3 — measurements and the abort criteria

Measured in the sandbox and recorded in P3's report and the record's wave-3 section:

- **Read-cost delta:** rendered chars actually delivered to the model in `brainstorm-delivery`
  (sum of the seven blocks as they appear in the transcript, plus the hook lines) against the
  pre-conversion baseline **12,819 chars** (the `.md`-obligated raw read; 14,349 with labels).
- **Store latency:** `mochiko-cli rules brainstorm --section <each>` wall-clock in the sandbox,
  ten runs each, mean and max; and the whole-fire figure (seven renders).
- **Log cost in the plugin:** the plugin directory's byte size before and after the move.

**Pilot abort criteria (D9, evaluated by the lead from P3's evidence, stated to the user):**
(1) read-back below 3/3; (2) delivered read cost above 12,819 chars. Either trips → waves 4–5
halt and the posture returns to the user (the ledger's GI-020 revisit trigger). Neither trips →
the wave lands and waves 4–5 open on the user's word.

## 6. Checklist (the done condition, itemized)

- [ ] P1: log at `plugins/mochiko/migrations/` (+ README); no repo-root `migrations/`; state hash unchanged; legend block; tests re-pathed; CI filter; `rust-cli.md` clauses
- [ ] P2: `brainstorm.md` per §3.1; `hooks/hooks.json` + three scripts per §3.2; `.claude/settings.json` maintainer hook; strip entry; `primitive-edits.md` globs (+ Q-B branch); `README.md`
- [ ] P3: six cases per §4; pending assertion resolved; measurements per §5; `evals/contract/README.md` updated; suite run green with evidence
- [ ] Audits: V1 crate review (rust-cli.md, non-author) · V2 `mochiko:validator` on the `brainstorm` pair (`.md` + `schemas/brainstorm.yaml`, canonical-scaffold criteria as read under D3 and Q-B) and on the hooks (D7 conformance, fail-open, absence-only) · V3 suite review (assertions keyed to measured shapes; no schema edits; no dispatch)
- [ ] Landing (lead): `plugin.json` 0.104.0 · `marketplace.json` · `CHANGELOG.md` · `DECISIONS.md` row · `BACKLOG.md` item · `ROADMAP.md` · record wave-3 section (+ F14) · index · governance PATCH v3.0.1 row (the glob obligation discharged; GI-011's ledger home; the CLAUDE.md "today the log lives at the repo root" clauses struck) · four layers green · abort criteria stated

## 7. Seat protocol

1. **Plan first.** The seat reads this plan, the record sections named in its scope, the rule
   file for every path it touches (`rust-cli.md`, `primitive-edits.md` — path-scoped rules
   inject on Read), and returns a short plan; the lead approves before any write.
2. **Test-first where there is a test layer** (crate, suite); the `.md` and hooks are graded
   by V2 and exercised by P3's cases.
3. **Reports** land at `.mochiko/brainstorms/cli-schema-delivery/wave3-reports/<seat>.md`: what
   was built, the tally, deviations from this plan with reasons, anything left undone.
4. **No git mutations by seats**; the lead commits at the user's word.
5. **Attempt bound:** 3 verification attempts per seat unit; rework is test-first.

## 8. Review criteria (validators, fresh seats, author never grader)

- **V1 (crate):** `rust-cli.md` in full; the move is a pure `git mv` (genesis byte-identical);
  every path reference updated; the legend golden test; no behavior change beyond the legend.
- **V2 (plugin):** the `brainstorm` pair against `primitive-edits.md` criteria 1–11 as read
  under D3 (criterion 2's enumeration is the seven `--section` arguments; criterion 3 per Q-B;
  criterion 8 preserved responsibilities — Identity, Entry, Goal byte-preserved); the halt
  clause verbatim per §3.1; the grant present; the hooks against D7 (absence-only gating, 5 s
  timeouts, no judgment, no dispatch, converted-check by the `.md` itself, no `jq`); the strip
  entry against `strips/README.md`; README claims true today (no `cargo install mochiko-cli`
  promised before the publish).
- **V3 (suite):** every assertion keyed to a measured shape, the bar pre-registered in this
  plan and unchanged, `pending` never reads as pass, evidence complete, the suite touches no
  plugin file and dispatches nothing (GI-019).

## 9. Open decisions at the wave-open gate (the user's)

- **Q-A — what the dependency hook delivers when the binary is present.** Branch A: the full
  rules only when the expanded prompt lacks them (needs the hook to see the expansion — F14);
  branch B: a one-line presence confirmation only, the policy-placeholder environment halting as
  GI-020 declares it unsupported. Never both channels in the normal case: double delivery
  (≈ 20 k chars) would trip abort criterion (2) by construction. **Lead recommends A if F14
  allows it, else B.**
- **Q-B — the Not-done count.** D3 says the counts are printed by the CLI, never hand-pinned;
  `primitive-edits.md` criterion 3 still demands the hand-pinned count. Option 1: follow D3 —
  no hand pin, criterion 3 gains a converted-command branch now (a governance PATCH row,
  v3.0.1, beside the glob obligation). Option 2: keep the hand pin beside the CLI pin until the
  wave-6 re-key. **Lead recommends 1.**
- **Q-C — sequencing against the wave-2 tail.** The pilot runs on the sandbox build; the ruled
  install line `cargo install mochiko-cli` does not work for anyone until the publish, which
  needs your GitHub-side actions (signed tags, the `crates-io` environment approval rule, the
  registry token). **Lead recommends: run wave 3 now; the README carries the git-install line
  until the publish; the publish is your call when the controls exist.**
- **Q-D — the pre-registered read-back bar** as stated in §4 (3/3, the `FLOOR:` line naming all
  seven ids exactly). Confirm or restate before the run.
