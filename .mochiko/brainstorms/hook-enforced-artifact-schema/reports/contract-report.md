---
report: verification
feature: hook-enforced-artifact-schema
round: 4
wave: 4
verdict: host cases green; sandbox half repaired and smoke-green
cases_added: 5
host_cases: 3
host_rows: 39
sandbox_cases: 2
sandbox_sessions_declared: 3
sandbox_sessions_run: 8
host_run: 7/7 cases passed, 7 ran, 5 measurements recorded
smoke_run: 5/5 sandbox cases passed (absence, skew, brainstorm-absence, gate-live, reminder-spawn)
metered_sessions: 13 session transcripts plus 3 preflight probes
plan: wave4-contract-plan.md
---

## What landed

| case | shape | rows | result |
|---|---|---|---|
| `gate-input` | host, no session | 31 | pass |
| `reminder-input` | host, no session | 6 | pass (1 recorded) |
| `if-placement` | host, static JSON | 2 | pass |
| `gate-live` | 1 sandbox session | 4 asserts | pass |
| `reminder-spawn` | 2 sandbox sessions | 3 asserts | pass (after a rebuild, below) |

Registered in `HOST_CASES` and in `build_sandbox_cases()`; the module docstring's case list and
`--list` both carry them. The four wave-0 captures are committed at
`evals/contract/fixture/artifact-hooks/` with a README recording provenance and consuming row.

`python3 evals/contract/run.py --host-only` → **7/7 cases passed, 7 ran, 5 measurements recorded**,
exit 0. Evidence per case under `evals/.work/contract-<case>-<id>/`, one `.stdin.json` and
`.stdout.json` per gate row plus `verdict.json`.

Every gate row's payload and expected exit was validated directly against `mochiko-cli check`
before `artifact-gate.sh` existed, so a red row here is the wrapper's, not the row's. The wrappers
landed mid-task and every row passed against them unchanged.

## Deviations from the approved plan

**The fixtures are not in `fixture/hook-input/`.** `load_captures()` globs that directory and
indexes by `hook_event_name`, first sorted filename winning. Three of the four captures are
`PreToolUse` and one sorts ahead of `pre-tool-use-skill.json`, so committing them there would have
handed the dependency-halt case a Bash payload where it expects `tool_input.skill` and its
per-skill rows would have stopped testing what they name, silently. Separate directory, separate
loader.

**Row count is 39, not the planned 34.** `G-FM` split into a missing-field row and an enum row;
the lead's two file-set amnesty rows landed as `G-SET-AMNESTY-EXISTING` and `G-SET-NEW`;
`G-BASH-READ` was added so the Bash arm's scope is pinned from both sides; and `G-PWSH` became
three rows (see below). Nothing planned was dropped.

**`if-placement` asserts the arms the lead specified**, not the plan's: `Write|Edit` carries no
`if`, every shell arm carries one narrowing on `.mochiko`. The shipped registration splits `Bash`
and `PowerShell` into separate groups, so each shell tool is asserted by name — a check that
looked for a matcher containing `Bash` would have passed while the `PowerShell` group carried no
narrowing at all. Both arms hold, and the group-level guard holds too.

**`R-LINE-EXACT` began structural and is now a carried golden.** When the rows were written no
frozen reminder line existed in the crate or the log, so the row asserted only what R5 rules — one
line, self-identifying — and recorded the text. It then briefly read the line out of
`seat-reminder.sh`, which is not a freeze at all: a reword of the script rewrites the golden with
it and the row still passes. The golden is carried in the case now. Detail in § Sandbox repair.

## Failure narrative

**The contract suite's sandbox half cannot start.** `evals/contract/run.py` needs three names from
`evals/run.py` — `SANDBOX`, `sbx_sh`, `claude_args`. Two are gone and the third changed shape: the
skill eval runner converged onto host mode on 2026-09-11 (`8c27460`) while the contract suite was
last touched 2026-09-05 (`32c1ed5`). `preflight()` raises `AttributeError: module
'mochiko_eval_runner' has no attribute 'SANDBOX'` before any sandbox case runs — an unhandled
exception, not a clean skip. Separately `run_probe()` calls `claude_args` positionally against a
signature that changed, so it asks for `--plugin-dir True --add-dir True`; verified by calling it.
That is 77 of 82 cases and roughly 151 metered sessions, both of mine included. GI-012 gate 6 makes
the suite green a condition of a `plugin.json` bump and says a SKIPPED suite is not green; this is
worse than skipped. Not fixed here: the repair is a design call between re-adding the sandbox
helpers and porting the suite to host mode, it touches every existing sandbox case, and it is
outside this brief. My two cases call `claude_args` by keyword so they cannot drift with it again.

## Notes of note

**The PowerShell arm decides on both vocabularies.** `check` branches on `tool_name`, not on the
platform's dispatch, so every decision below is host-testable where the routing is not.
`G-PWSH-REDIRECT` covers the `>` form, `G-PWSH-NATIVE` the native cmdlets, and `G-PWSH-READ` is the
allow control proving the cmdlet table is write-scoped rather than path-scoped. What no row in this
suite reaches is whether Claude Code routes a real PowerShell call to this hook.

**A stale binary hid the file-set amnesty.** `G-SET-AMNESTY-EXISTING` denied until I rebuilt:
`conform.rs` was newer than the binary `host_binary()` prefers. Guarded now — see § Sandbox repair.

**Both directories this wave writes to are undeclared.** `wave4-reports/` and `wave0-fixtures/`
resolve as not declared sub-directories; `reports/` does. Hence this path; options in the message.

## Sandbox repair

The suite no longer imports the skill-eval harness. `SANDBOX`, `sbx_sh` and `claude_args` now live
in `evals/contract/sandbox.py`, lifted from `evals/run.py` as it stood at `32c1ed5` — the last
commit that touched this suite against a working runner — with the commit cited in the module
header. `load_runner()` imports that file instead, and `run_probe()` calls `claude_args` by
keyword rather than positionally, which is what let the 2026-09-11 signature change rebind
`plugin` to `True` without a single test going red.

The duplication is deliberate and the header says so: this suite is a release gate under GI-012,
and a gate may not be at the mercy of a research harness that owes it nothing and is free to change
whenever the skill-eval work needs it. If `evals/run.py` converges again, this file does not move.

Verified after the repair: `load_runner()` returns all three names; `claude_args` produces
`--plugin-dir <path>` under both a keyword and a positional call; 80 sandbox cases and 7 host cases
declare cleanly.

**A bug the repair surfaced in my own case.** `sandbox_path()` verifies that `mochiko-cli` is
*absent* from the sandbox's default PATH — that absence is the premise of the `absence` case — and
my `_artifact_probe` was inheriting it. A `gate-live` session would then have run the gate with no
binary behind it: the wrapper falls through to its explicit allow, the write under test lands, and
the case reports a gate failure that is really a harness one. Both cases now pass
`{sandbox.binary_dir}:{sandbox.path}` explicitly and `path_env` is a required argument, so the
mistake cannot be made again by omission. Caught before the smoke run, not by it.

**Binary staleness guard.** `host_binary()` now refuses a binary older than any `.rs` file under
`crates/mochiko-cli/src/`, naming the changed files and the rebuild command. It refuses rather than
rebuilding: a gate that silently rebuilds its own subject grades something the operator did not
choose. This is the trap that made a conforming amnesty row deny during authoring. It compares
mtimes, so a source file touched without a content change trips it too; the rebuild it prints is
the remedy either way, and a conservative false stop is the right side to err on for a gate.

**The reminder golden is carried by the case, not read from the script.** My first pass parsed the
line out of `plugins/mochiko/hooks/scripts/seat-reminder.sh` on the reasoning that a second copy is
a second thing to drift. That was the wrong trade and the reviewer's W2 caught it: a golden read
from the thing under test is tautological, so a reword of the script would have rewritten the
golden and passed the row, delivering no freeze. `REMINDER_GOLDEN` now holds the text in
`run.py`, and the row asserts it from both ends — the script's own text matches it byte for byte,
and so does the line the script emitted. A reword anywhere fails the row until someone edits the
constant too, and that edit is a visible change to a release gate, which is the freeze. Proved
negatively: a one-word reword of a copied script trips both limbs.

**`reminder-spawn` was rebuilt after the smoke run refused it.** Its first shape asked each
subagent to quote the context it had been given. The lead seat declined the whole task as a
system-prompt extraction attempt — correctly — so no subagent was ever spawned and the case
asserted nothing while reporting a hook failure. That is the same reflex wave 0 recorded when a
seat read the injected line itself as an injection, arriving this time from the other side. The
case now gives each seat a trivial task and reads the line off the `hook_additional_context`
attachment in the sidechain transcript on disk, which takes the model out of the evidence path.

**Smoke result: 5/5.** `absence`, `skew` and `brainstorm-absence` — three existing sandboxed cases
that could not start before the repair — plus both of mine. `gate-live` passes every limb against
what ships: the non-conforming write is denied and leaves no file on disk, the conforming write in
the same session lands, and the deny reaches the model carrying D9's advisory halt sentence and the
failing measure. `reminder-spawn` proves the injection from the sidechain transcript itself, one
`hook_additional_context` attachment per arm, one spawn each. Cost: 13 session transcripts plus a
preflight authentication probe per invocation, against the full set's ~151. The full deterministic
run stays gate 6 at the bump.

**The PowerShell row was promoted in the same round.** `powershell_write_targets` landed in
`hook.rs` while this round was open, so `G-PWSH-NATIVE`'s recorded allow went stale: `Set-Content
-Path <home>/record.md -Value x` now denies, as do `Out-File`, `Add-Content`, `New-Item`,
`Tee-Object` and the `Copy-Item`/`Move-Item` pair. The row is asserted from here, and kept
structural — the deny must name the target it resolved, never a fixed sentence, so a reworded
reason or a widened cmdlet table does not fail it. `G-PWSH-READ` was added beside it as the allow
control: `Get-Content` over a home path allows, which is what proves the table is scoped to write
verbs rather than to paths. Both are host rows, so the promotion cost no metered session.

**The `sandbox.py` header overstated the lift.** It said the three names moved "unchanged in
behaviour"; `claude_args` in fact differs from `32c1ed5` in three ways. Its `LOCAL_MODE` limb,
which inserts `--bare` for the runner's hermetic local mode, is dropped, because this suite has no
local mode and never reads that switch. `stream` and `plugin` gained defaults where the original
required both positionally. The literal `"acceptEdits"` moved to the `PERMISSION_MODE` constant at
the same value. No caller depends on any of it — both call sites pass every parameter by keyword,
so the defaults never fire — and the argv built here is byte-identical to what the original
produced in sandbox mode. The header now says exactly that instead. Reviewer's W6; header-only, no
behaviour change, verified by importing the module and printing the argv.

