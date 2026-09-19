# Wave 4 — plugin contract cases for the two new hooks (plan; awaiting lead approval)

**Author:** QA seat · **Date:** 2026-09-13 · **Graded against:** record D3 (as amended today), D4,
D9, D10 · `wave1-plan.md` §2a–§4 · `evals/contract/README.md` and `run.py`'s `case_hook_input` ·
user ruling R5 (per-seat `SubagentStart` reminder replaces the Read-time form).
**Nothing is written under `evals/` until the lead opens it after wave 1 lands.**

Shape follows the existing `hook-input` case: host rows, no sandbox, no metered session, a staged copy
of `plugins/mochiko/`, each row one `host_sh` call with stdin, `PATH` and `CLAUDE_PLUGIN_ROOT` per row,
asserted with `ok()` / `report()` and `json_field()`. New host cases rather than more rows on
`hook-input`, whose rows iterate primitives where these iterate payloads. Four rows are fed real
wave-0 captures, durable at `research/wave0-fixtures/` — provenance and consuming row in its `README.md`.

## Case list — 3 host cases, 34 rows (30 tabled, G-FM carrying two variants, `if-placement`'s 2, and the two report rows gap 6 adds)

`gate-input` — the write-time gate wrapper. Fixture log per §"What wave 1 owes" below.

| id | payload fixture | expected wrapper stdout | check exit | D10 clause |
|---|---|---|---|---|
| G-PATH | Write, path under the tree, segment pattern unmatched | deny | 4 | deny: path |
| G-SET | Write, undeclared file name inside a declared home | deny | 4 | deny: file set |
| G-HEAD | Write, content carries an undeclared `##` | deny | 4 | deny: shape |
| G-FM | Write, required frontmatter field absent; twin row with an enum value off-list | deny | 4 | deny: shape |
| G-PLACE | Write, a declared placeholder token in heading text | deny | 4 | deny: shape (D4c) |
| G-SIZE | Write, one section over `max_lines` | deny | 4 | deny: size |
| G-EDIT | Edit whose applied result adds an undeclared `##` | deny | 4 | D4e Edit-on-result |
| G-AMNESTY | Edit over an already-oversized baseline, not worsened | allow + `additionalContext` naming the standing overage | 0 | D4e first-touch amnesty |
| G-CONFORM | conforming Write at a declared name | allow | 0 | allow on a conforming write |
| G-BASH-HEREDOC | `research/wave0-fixtures/pre-tool-use-bash-heredoc.json` | deny | 4 | V12 denied heredoc |
| G-BASH-REDIRECT | `research/wave0-fixtures/pre-tool-use-bash-redirect-false-allow.json`, path re-pointed at the fixture home | deny | 4 | V12 · the field() regression |
| G-BASH-PLAIN | `echo probe-plain-ok` and `git status --porcelain` | allow | 0 | V12 allowed ordinary Bash |
| G-BASH-READ | `cat <home>/seed.md` (reads a home, writes nothing) | allow | 0 | D1c write-operator scope |
| G-SNIFF | Write outside every home, content opens `report:` in the enum | deny | 4 | D9 frontmatter sniff |
| G-PLAIN-MD | Write of a plain `.md` outside the declared tree | allow | 0 | D9 no gate outside |
| G-OUTSIDE-CWD | Write to an absolute path outside `cwd` | allow | 0 | §3 step 4 |
| G-READ | `research/wave0-fixtures/pre-tool-use-read-home.json` fed to the gate wrapper | allow, no `additionalContext` | 0 | R5: the gate owns no Read behaviour |
| G-EXIT1 | any Write; staged log emptied / unsound | explicit allow | 1 | C2 exit 1 pass-through |
| G-EXIT2 | any Write; `mochiko-cli` on `PATH` is a stub that exits 2 | explicit allow | 2 | C2 exit 2 (binary predates `check`) |
| G-EXIT3 | any Write; staged log `grammar: 99` | explicit allow | 3 | C2 exit 3 skew |
| G-EXIT4 | G-HEAD re-asserted on the exit code itself | deny | 4 | C2: only a verdict denies |
| G-UNPARSABLE | malformed JSON on stdin | explicit allow | 2 | §3 step 1 |
| G-ABSENT | any Write; no `mochiko-cli` on `PATH` (`MINIMAL_PATH`) | explicit allow | n/a | fail-open, D7 floor |
| G-NEVER-EMPTY | run-wide over every row above | stdout is non-empty valid JSON carrying `permissionDecision`; wrapper exit always 0 | — | wave-0 platform fold in D3 |
| G-PWSH | hand-written PowerShell-shaped payload (`tool_name: PowerShell`) | deny | 4 | **`report()`, not asserted** — see gap 4 |

`reminder-input` — the `SubagentStart` wrapper. Uses `research/wave0-fixtures/subagent-start.json`, whose real
key set is `agent_id · agent_type · cwd · hook_event_name · prompt_id · session_id · transcript_path`
— no `permission_mode`, which is why it is a capture rather than written from the doc field list.

| id | payload | expected | D10 / ruling |
|---|---|---|---|
| R-INJECT | the capture | exactly one `hookSpecificOutput.additionalContext`; **no** `permissionDecision` key | R5 |
| R-LINE-EXACT | the capture | the line matches a frozen golden byte-for-byte and names mochiko as its source | R5 + wave-0's injection-reading finding |
| R-NO-TOPLEVEL | the capture | no top-level `additionalContext` key | wave 0: the top-level form is not consumed |
| R-ABSENT | the capture, no binary on `PATH` | silent exit 0 | fail-open, D7 floor |
| R-WRONG-EVENT | a `PreToolUse` capture | silent exit 0, empty stdout, `silent(proc)` | leave-alone limb |

`if-placement` — host, static, no session. Wave 0 measured that a matcher-group-level `if` is
silently ignored, so the failure is invisible at runtime and a static assert is the only cheap
guard. Two rows: every `if` in the shipped `hooks/hooks.json` sits on a handler object beside
`type`; no `if` appears on any matcher-group object. The overwritten group-placement JSON is
transcribed in `wave0-probe-report.md` leg 7 for the negative fixture.

## Sandbox session cases — 2 cases, 3 sessions

- **`reminder-spawn` (2 sessions).** One unnamed `Agent` spawn, one spawn with a `name`. Assert the
  golden line reaches each subagent as a `{"type":"hook_additional_context"}` attachment before its
  first turn, and record that `agent_type` is `general-purpose` for both — the wave-0 finding that
  the event's agent-type matcher cannot single out a named teammate. The suite already spawns a
  plugin agent in its `preload` case, so spawning is reachable; if it turns out not to be under the
  sandbox, the gap to state is that the host rows prove the wrapper's **output** and only a session
  proves **delivery**, which wave 0 has already measured once.
- **`gate-live` (1 session).** The one limb no host row carries: a non-conforming write is actually
  blocked (no file on disk afterwards) and the deny text the model received closes with D9's
  advisory halt sentence, while a conforming write in the same session lands.

## What wave 1 owes this suite

1. `check --hook-json -`, with **4** minted as the conformance-deny code.
2. A committed fixture log under `evals/contract/fixture/` carrying at least one `home` document,
   and covering four branches so no row waits on wave 3's census: a templated deliverable with
   per-section budgets · a template-less deliverable with a whole-file bound (D4f) · a `reports/`
   dir with an envelope enum (D2) · one `bounds: elsewhere` kind with its `bounds_cite` (D4f/V2).
3. A two-line stub `mochiko-cli` that exits 2, for G-EXIT2. The skew log reuses `write_skew_log`.
4. The frozen reminder line, as a golden the suite reads rather than restates.

## Gaps and dependencies to settle before the cases are written

1. **`.claude/rules/mochiko/rust-cli.md` still forbids what these cases assert:** "Its hooks MUST
   block only on the binary's absence or a log outside its grammar range, never on behavior."
   Wave 2's GI-019 amend must land first, or the suite asserts against a live rule that bans it.
2. **D10's reminder clause is superseded by R5** ("reminder fires on a home read and not
   elsewhere"). The Read-time rows are replaced by the five `reminder-input` rows plus G-READ.
3. **`wave1-plan.md` §3 contradicts its own §4** on non-deny stdout: steps 3 and 8 say "Miss → 0,
   empty stdout", the exit-code table says exit 0 always emits an explicit allow. Every row here
   asserts the table. §3 needs a one-line repair in wave 1, otherwise G-NEVER-EMPTY fails the crate.
4. **G-PWSH cannot be a real capture.** No PowerShell tool exists on macOS, so its payload is
   hand-written and the row is recorded with `report()` rather than asserted — the posture
   `brainstorm-policy` already uses. Asserting it needs a Windows leg the suite does not have.
5. **Which log the rows resolve — open, ruled at the wave-4 open** (lead note, 2026-09-13, from the
   reviewer's read of the crate fixture). Verified: `crates/mochiko-cli/tests/fixtures/home-log/
   0001-homes.yaml` declares `sections: []` on `report-envelope` (line 147) while its `demo-spec`
   template carries a full block — four ordered sections with `max_lines`, `extra_headings: deny`,
   three placeholder tokens, required frontmatter with an enum. So the gap is narrower than a
   frontmatter-only fixture: every deliverable-bound row (`G-HEAD`, `G-PLACE`, `G-SIZE`, `G-FM`,
   `G-EDIT`, `G-AMNESTY`) runs in full against the crate fixture today, and `G-SNIFF` needs only the
   envelope's enum, which is populated. **Recommendation:** resolve against the plugin's real log via
   `--plugin-root`, matching the suite's standing posture that every per-command case stages
   `plugins/mochiko/` and perturbs its own copy; keep the crate fixture as the fallback only for a
   branch wave 3's log does not carry. To survive a budget re-key, those rows assert **structurally**
   — a deny fired naming the failing measure — never against a specific `max_lines` value.
6. **A row the plan is missing, found while checking gap 5.** D2's open-by-name reports limb has no
   row: nothing writes a report *inside* a declared `reports/` dir, either conforming at an arbitrary
   name (allow) or failing the envelope (deny). `G-SNIFF` only covers a report smuggled *outside*
   every home. Two rows owed — `G-REPORT-OK` and `G-REPORT-BAD` — and they are the rows that need
   wave 3's `report-envelope` to carry real sections, so they cannot run against the crate fixture as
   it stands. Row count becomes 34.
