---
report: review
feature: hook-enforced-artifact-schema
round: 4
verdict: PASS
graded_artifact: >-
  plugins/mochiko/hooks/hooks.json · hooks/scripts/artifact-gate.sh · hooks/scripts/seat-reminder.sh ·
  crates/mochiko-cli/src/conform.rs (the `UndeclaredFile` arm) · src/hook.rs (the PowerShell write
  vocabulary) and the tests carrying both · evals/contract/sandbox.py · evals/contract/run.py (the
  sandbox repair and the five new cases)
graded_against: >-
  wave4-plan.md (a)/(b)/(d) as superseded by the lead's two calls — one un-narrowed `Write|Edit`
  handler with `if` on `Bash`/`PowerShell` only, and D4e file-set amnesty as ratified ·
  wave0-probe-report.md legs 3/5/7 · ledger AM-3 (the amnesty paragraph, the explicit-allow rule,
  the bright line's clause iv) · record D3/D7/D9/D10 · .claude/rules/mochiko/rust-cli.md ·
  reports/contract-report.md § Sandbox repair
reviewer_role: >-
  independent non-author code review per rust-cli.md; authored none of the graded material and
  edited nothing outside this file
producer_report: reports/wave4-cycle-report.md · reports/contract-report.md
fix_list_count: 6 raised, 0 open — every one was closed by the seats and re-verified on the
  tree in the delta-check below
blocking: []
nothing_edited: true

item_grades:
  - id: 1
    name: the two wrappers and the registration
    grade: PASS
    method: >-
      Every cell run by me, not read. The four `wave0-fixtures/` payloads through each wrapper (eight
      runs), a stub-binary matrix over CLI exit 0/1/2/3/4, an absent-binary run for both scripts, and
      a static assertion over hooks.json.
    registration: >-
      Seven handlers across four events, every one carrying `timeout: 5`. `if` appears on exactly two
      handler objects — `Bash(*.mochiko*)` and `PowerShell(*.mochiko*)` — and on no matcher-group
      object, which is leg 7's placement trap avoided. `Write|Edit` is one un-narrowed handler and
      `SubagentStart` carries no matcher, both as the lead ruled. No `if` sits on a non-tool event,
      where leg 7's citation says a hook carrying one never runs at all.
    gate_wrapper: >-
      POSIX clean — `dash -n` parses both scripts, and the only `[[` in either file is the POSIX
      bracket expression `[[:space:]]`. The gate parses nothing: it never reads its own stdin, so the
      payload reaches the binary untouched. Proved rather than asserted — the `r0-retry-2` redirect
      fixture, the one that truncated at an escaped quote and produced wave 0's false allow, is
      **denied** when its target is repointed at a real home. On exit 4 the wrapper prints the
      binary's line **byte-identical** to the same payload run directly against the CLI (`diff`
      clean), and exits 0. Exit 0 with content echoes it, so an amnesty `additionalContext` survives;
      exit 0 with empty stdout, exits 1, 2, 3 and an absent binary each emit the bare explicit allow.
      Wrapper exit status was 0 on every one of those paths.
    reminder_wrapper: >-
      Emits the frozen line only on the `SubagentStart` fixture; silent with exit 0 on all three
      `PreToolUse` fixtures and silent with exit 0 with no binary on `PATH`. A grep for
      `permissionDecision` over its output returns zero. The event test is a presence match over the
      whole payload, not field extraction, which is the leg-3 defect not repeated.
    exit_matrix: "cli 0 -> allow · 1 -> allow · 2 -> allow · 3 -> allow · 4 -> the binary's deny verbatim · absent -> allow; wrapper exit 0 in every cell"
    findings: [W2, W3, W5]
  - id: 2
    name: the crate fix
    grade: PASS
    amnesty_cells: >-
      All four run through the real binary against the shipped log, with a real baseline file on
      disk. Fresh `Write` at an undeclared name -> deny, exit 4. `Write` over the existing undeclared
      name -> allow, exit 0, `additionalContext` naming `notes.md` and the standing violation. `Edit`
      over the same -> allow, exit 0, same context. A *different* undeclared name in the same home,
      with the amnestied neighbour already on disk -> deny, exit 4. The amnesty does not leak between
      names because the fault key carries the name (`file-set:<name>`), so a neighbour's baseline can
      never excuse it — the right mechanism, not a special case.
    path_control: >-
      My own addition, because the ruling turns on it: an `Edit` to an *existing* file in an
      undeclared sub-directory still denies. Path is not relaxable and the code does not accidentally
      relax it through the same door.
    powershell: >-
      Landed, and it closes an arm that was inert. Thirteen shapes reproduced on my run: `Set-Content`
      in both parameterised and positional form, `Out-File`, `Add-Content`, `New-Item`, `Tee-Object`,
      `Copy-Item`, a lower-case spelling, and `>>` all deny into a home; `Get-Content`,
      `Get-ChildItem` and a home path riding as `-Value` all allow. I also probed the shape the table
      does not name — `New-Item -Path <dir> -Name <file>` — and it denies through the directory
      target, so skipping `-Name` costs nothing here. Cross-talk holds both ways: the same cmdlet text
      under `tool_name: Bash` allows, because the vocabulary is keyed to the tool.
    scope: >-
      Two `src/` files, against the plan's "the one `src/` change wave 4 takes". Disclosed in the
      cycle report's frontmatter and argued in its own section, and the lead called the PowerShell
      work in; recorded here rather than raised as a finding.
    no_schema_read: >-
      Confirmed structurally. The crate's read sites are the migration directory (`replay.rs`), the
      plugin manifest for its version stamp (`cli.rs`), and — on an `Edit` — the on-disk baseline
      (`hook.rs:174`), which is exactly what AM-3's "what the gate reads" clause licenses. No schema
      file ships in the plugin for a run to read instead.
    findings: [W1, W4]
  - id: 3
    name: cost
    grade: PASS
    detail: >-
      Reproduced on the released binary, median of 20 per cell: wrapper 30.2 ms on an allow, 30.1 ms
      on a deny, binary alone 24.2 ms, so the shell adds 6.0 ms. The seat's 31.5 / 30.9 / 27.0 / ~4.5
      reproduces within noise, and the per-call median stays well under the 100 ms cache trigger.
      One caveat worth carrying: the same measurement against a `cargo build` debug binary is 134.5 ms,
      four times over that trigger. The figure holds for what consumers install, not for a maintainer
      running out of `target/debug`.
  - id: 4
    name: the ledger fact
    grade: PASS — the seat is right and the ledger sentence is wrong
    detail: >-
      Confirmed against a genuine pre-fix binary: the installed `~/.cargo/bin/mochiko-cli` is stamped
      2026-09-13 19:56, predates the conform.rs edit, and carries no PowerShell table. Against it,
      both a `Write` and an `Edit` over an existing undeclared file name return
      `permissionDecision: deny` at **exit 4**. The same `Edit` against the fixed build returns exit 0
      with the violation in `additionalContext`. The ledger's "an `Edit` ... rides amnesty with a bare
      allow and no `additionalContext`" does not reproduce in either limb: there was no allow to be
      bare, and the mis-homed file was wedged. The bump's PATCH can correct that sentence on this
      evidence.

  - id: 5
    name: the contract suite — the sandbox repair and the five new cases
    grade: PASS
    sandbox_py: >-
      `evals/contract/sandbox.py` carries the three names the suite needs, and I diffed each against
      `evals/run.py` at `32c1ed5` rather than taking the header's word. `SANDBOX` is identical.
      `sbx_sh` is identical in argv, capture and its 1800-second default. `claude_args` keeps the
      pre-convergence parameter *order* — the point of the exercise — and produces the same argv;
      `acceptEdits` is hoisted to a module constant at the same value, and `--setting-sources ''`
      survives with its reason. The header states the provenance commit and the decoupling reason:
      a release gate under GI-012 may not depend on a research harness free to change under it,
      and it names the 2026-09-11 convergence as the failure it is answering.
    import_and_call: >-
      `load_runner()` loads that file by path under its own module name, with a docstring recording
      why it no longer imports `evals/run.py`. `run_probe()` calls `claude_args` with all five
      arguments by keyword, with a comment naming the positional call as what let the signature
      change rebind `plugin` to `True` silently. `_artifact_probe()` does the same.
    staleness_guard: >-
      Exercised directly, not read. Against the stale `~/.cargo/bin/mochiko-cli` the guard returns
      the refusal naming `conform.rs, hook.rs, validate.rs` and the rebuild command; against the
      fresh `target/release` binary it returns `None`, and `host_binary()` selects that one. It
      refuses and never rebuilds, which is the right call for a gate — a suite that rebuilds its own
      subject grades something the operator did not choose. QA discloses that it compares mtimes, so
      a content-free touch trips it; I tripped it myself with a `cargo fmt` pass, and a conservative
      false stop is the right side to err on.
    cases: >-
      Counted off the source and confirmed against my own run: `gate-input` carries exactly 30
      `GateRow` rows plus the `G-NEVER-EMPTY` wrapper-contract check, which is QA's 31;
      `reminder-input` 6 checks (5 asserted, 1 recorded); `if-placement` 2. Every row runs the real
      wrapper against the real staged plugin, so the log behind each decision is the shipped one,
      reached through `CLAUDE_PLUGIN_ROOT` and the wrapper's own `--plugin-root`. The three
      exception roots are per-row and scoped — a synthesized unsound log, a skewed one, a stub
      binary, and a binary-free PATH — so no row silently borrows another's substrate.
    d10_coverage: >-
      I walked D10's clause list against the rows rather than trusting the labels. Every clause has
      a row: the exact frozen line with no `permissionDecision`, fail-open with no binary, silence
      on a wrong event, a `Read` payload as a plain allow under R5, a deny on each of the four
      failure classes and an allow on a conforming write, the denied Bash heredoc beside an allowed
      ordinary command under V12, and one row per CLI exit code under C2.
    pwsh_promotion: >-
      `G-PWSH-NATIVE` is now an asserted deny keyed on three keywords including the resolved target,
      and `G-PWSH-READ` was added beside it as the allow control that proves the cmdlet table is
      scoped to write verbs rather than to paths. Both are host rows, so the promotion cost no
      metered session. This closes W1.
    path_env_trap: >-
      `_artifact_probe`'s `path_env` is keyword-only with no default, and its docstring records the
      trap: the sandbox's own PATH deliberately lacks the binary because the `absence` case is built
      on that, so an inheriting session would have run the gate with nothing behind it and reported a
      harness failure as a gate one. I audited every `path_env` site in the file. Four pass the
      bare `sandbox.path` — `case_absence`, `case_command_absence`, `case_skill_absence` and
      `case_brainstorm_hooks_off` — and each is an absence case by name and by docstring. Every
      other site, both new cases included, prepends `sandbox.binary_dir`. No case inherits the trap.
    reminder_spawn: >-
      Reads the injected line off the `hook_additional_context` attachment in the sidechain
      transcript on disk, never from the subagent's reply, with the reason recorded: the first shape
      asked each seat to quote its own context and the lead seat declined it as a prompt-extraction
      attempt, so nothing was spawned and the case asserted nothing. It also counts spawns under
      both `Agent` and `Task`, which is the wire-name question wave 0 left open.
    release_mechanics: >-
      `evals/contract/sandbox.py` and `evals/contract/fixture/artifact-hooks/` are untracked today.
      So is every other file this wave adds, so this is a line on the bump checklist rather than a
      defect — but gate 6 runs the suite from what is committed, and an uncommitted `sandbox.py`
      reproduces the exact `load_runner()` failure this repair exists to end.
    findings: [W6]

gates_my_own_run:
  cargo_test: "PASS — 457 passed, 0 failed, exit 0 (the brief's 455 is stale; the cycle report also records 457)"
  cargo_fmt: "PASS — exit 0"
  cargo_clippy: "PASS — exit 0, zero warnings; source touched first to force a fresh lint"
  cargo_audit: "PASS — `--deny warnings`, exit 0, 31 dependencies"
  migrate_validate: "PASS — `0 rejecting · 104 advisory`, exit 0"
  contract_suite_host: >-
    PASS — 7/7 cases, 5 measurements recorded, against the fresh `target/release` binary the
    staleness guard selected. Re-run after the sandbox repair and the row additions.
  not_run: >-
    The sandboxed set was not run here, as the lead directed; QA's smoke of 5 of those cases is
    recorded in contract-report.md § Sandbox repair (13 session transcripts plus 3 preflight
    probes). Gate 6 wants the full deterministic set against the tagged binary, so this review does
    not clear it.

fixes_closed_since_this_review_opened:
  note: >-
    Four of the six findings were fixed by the seats while this review was open. Each was
    re-verified on the current tree; none is taken on the seat's say-so.
  W1: >-
    Closed. `G-PWSH-NATIVE` asserts a deny and `G-PWSH-READ` is the new allow control. Both green on
    my run.
  W2: >-
    Closed, and closed better than the fix I named. `REMINDER_GOLDEN` is now a literal in `run.py`,
    and `R-LINE-EXACT` asserts it from both ends — the script's own text must match it byte for
    byte, and so must the line the script emitted. A reword anywhere fails the row until someone
    edits a release gate's constant, which is what a freeze is.
  W3: >-
    Closed. `artifact-gate.sh:48` now reads `if [ \"$code\" -eq 4 ] && [ -n \"$out\" ]`, with the
    reason in a comment. Re-verified: the empty-stdout exit-4 stub now draws the explicit allow
    instead of an empty line, the real deny still prints byte-identical to the direct CLI output,
    every other cell is unchanged, and `dash -n` is clean on the edited script.
  W4: >-
    Closed. The Edit cell now asserts `rendered.contains(\"notes.md\")` alongside
    `additionalContext`, with a comment naming the ledger's gap as the limb it pins.
  W5: >-
    Closed. The cycle report gained `## The shell parse's known gaps, disclosed` and a frontmatter
    count of three. Its whole table reproduces on my run — see the delta-check.
  W6: >-
    Closed. The header now names all three ways `claude_args` differs from the lifted original, and
    the argv it builds is byte-identical to that original's — tested, not taken.

findings:
  - {id: W1, status: closed, type: stale-assertion, sev: Minor,
     at: "evals/contract/run.py, the `G-PWSH-NATIVE` row",
     gap: "Its clause still reads `D1c: the parse vocabulary has no PowerShell verbs`, its expectation is `allow`, and it is `status=\"report\"`. On my run it records `decision 'deny'; decision 'deny', expected 'allow'`. The suite stays green because a recorded row never fails — which is the problem: the only cmdlet cell the contract suite carries now asserts nothing, so gate 6's deterministic set has its hole exactly where this wave's new code is. The seat found this and routed it to QA; it is still owed.",
     fix: "QA flips the row to an asserting `deny` with a keyword on the reason, and rewrites the clause. Until then the crate tests are the sole cover for the cmdlet table."}
  - {id: W2, status: closed, type: unenforced-freeze, sev: Minor,
     at: "evals/contract/run.py `frozen_reminder_line()` against wave4-plan.md (b)",
     gap: "The plan says the line is frozen because `the contract suite reads it as a golden ... never reworded without a row change`. The suite reads the golden **out of `seat-reminder.sh` itself**, by regex over the script's own literal. So `R-LINE-EXACT` compares the script against the script: it catches a line built dynamically, a multi-line line, and a line that stops naming mochiko, but a reword passes silently. The docstring argues the choice deliberately — a second copy is a second thing to drift — and that is a real trade, but the freeze the plan claims is not the freeze that shipped.",
     fix: "Either pin the line in the suite (or in a fixture) and accept the second copy, or correct the plan's and the script's claim to what the row actually holds."}
  - {id: W3, status: closed, type: robustness, sev: Minor,
     at: "plugins/mochiko/hooks/scripts/artifact-gate.sh:45-48",
     gap: "The exit-0 branch guards on `[ -n \"$out\" ]` and falls back to the bare allow; the exit-4 branch has no such guard. A stub binary exiting 4 with empty stdout makes the wrapper print an empty line and no decision at all — I measured it. The wrapper's own contract, and the suite's `G-NEVER-EMPTY` row, is that every outcome speaks an explicit decision, because silence reads as a deny on a background subagent's call. The shipped binary always prints on exit 4, so this is unreachable today; it is one line from being unreachable by construction.",
     fix: "Guard the deny branch the same way the allow branch is guarded, falling through to `allow` on empty output."}
  - {id: W4, status: closed, type: test-coverage, sev: Minor,
     at: "crates/mochiko-cli/tests/hook.rs, the `Edit` cell of `an_existing_file_at_an_undeclared_name_is_editable_while_a_new_one_still_denies`",
     gap: "The `Write` cell asserts `additionalContext` *and* that it contains `notes.md`. The `Edit` cell asserts only that `additionalContext` is present. The ledger's recorded gap is `Edit`-specific — an Edit riding amnesty with no signal, carried to wave 4 as `name the file-set violation too` — so the one cell that pins the ruling's own wording does not pin the name.",
     fix: "Assert the file name in the Edit cell's context too. One line."}
  - {id: W5, status: closed, type: disclosure, sev: Minor,
     at: "reports/wave4-cycle-report.md, the wrappers and cost sections",
     gap: "The shell arm has three shapes that do not deny, and none is named. With cwd already inside a home, `printf x > gates.md` carries no `.mochiko`, so `if` never fires and the hook does not run. Two more reach the binary and are allowed by it: `D=.mochiko/...; printf x > $D/gates.md` and `cd .mochiko/... && printf x > gates.md`. I ran all four; only the literal-path form denies. The parse is documented best-effort in `write_targets` and the deny reason says so, and D9 makes escapes the reviewer's — so this is ruled-acceptable, not a defect. But the plan's stated reason for the Bash arm is that it closes the cheapest route around a `Write` deny, and `cd` is the cheapest route.",
     fix: "Name the three shapes in the report's limits so the D9 reviewer knows what to look for, and consider a test pinning them as known-allow so a later reader does not read them as bugs."}
  - {id: W6, status: closed, type: header-accuracy, sev: Minor,
     at: "evals/contract/sandbox.py, the module header",
     gap: "It says the three names moved `unchanged in behaviour`. Two did. `claude_args` dropped the `LOCAL_MODE` limb that inserts `--bare` for a hermetic local run, and gave `stream` and `plugin` defaults the original had none of. Neither costs this suite anything — I grepped the whole contract file for `LOCAL_MODE`, `--local`, `--bare` and `parse_stream` and it uses none of them, and `sandbox.py` has exactly one importer — so the claim is true of what the suite calls. It is not true of the copy, and the header is the artifact a later reader diffs against `32c1ed5`.",
     fix: "One clause naming the dropped local-mode limb and the added defaults, so the diff against 32c1ed5 has no unexplained line. Nothing in the code needs to move."}

strengths: >-
  The gate wrapper's refusal to parse is the whole design and it is verified by the one payload that
  broke the old reader, not by argument. The deny text reaches the platform byte-identical to the
  binary's own. The amnesty fault key carries the file name, so the relaxation is per-file by
  construction rather than by a guard someone can forget. Path stays outside the amnesty even on an
  existing file, which is the limb the ruling turns on. The PowerShell table ships with a false-deny
  control (`-Value` holding a home path) and a cross-talk test, both of which are the failure modes a
  vocabulary table actually has. The seat volunteered the stale QA row and the second `src/` file
  rather than leaving either to be found. On the suite side the two best moves are both refusals:
  the staleness guard refuses instead of rebuilding, and `reminder-spawn` refuses to let the model
  carry the evidence.
---

## Notes of note

**The verdict is PASS against the superseded plan, not the plan as written.** Both of the lead's
calls are implemented as ruled, and I graded the superseding text.

**What this review does not clear.** Gate 6 wants the full deterministic set against the tagged
binary; I ran the seven host cases only. The bump's precondition is untouched — two controls still
owed — so the staged `plugin.json` at 0.109.0 cannot land on my say-so.

**One stale line elsewhere, still stale at the delta-check.** The cycle report's carried item 1
reads "the reminder golden is authored and unfrozen. QA owns the row." QA has frozen it; discharged.

**One advisory, below finding severity.** All three `settle(...)` sites end
`.unwrap_or_else(Verdict::allow)` and `settle` never returns `None`; the unreachable default is allow.

## Delta-check

Bounded to the five minors this review raised, re-checked on the tree of 2026-09-15 with my own
runs. I took no seat's word for any of them, and edited nothing outside this section.

**W1 — held.** `G-PWSH-NATIVE` is an asserted deny, keyed on three fragments including the resolved
target path rather than a fixed sentence, so a reworded reason or a widened cmdlet table does not
fail it. `G-PWSH-READ` sits beside it as the allow control that proves the table is scoped to write
verbs rather than to paths. Both green on my host run, alongside `G-PWSH-REDIRECT`. The stale
recorded row that asserted nothing is gone.

**W2 — held, and stronger than the fix I asked for.** `REMINDER_GOLDEN` is a literal in `run.py`,
and `R-LINE-EXACT` compares it against two independent things: the line lifted out of
`seat-reminder.sh`'s own text, and the line the script actually emitted. I proved the static limb
myself rather than reading it — copying the plugin's hooks tree and changing one word of the frozen
line ("templates" to "a template") makes `script_reminder_line()` stop matching the golden, while
the shipped script still matches byte for byte. A reword now costs an edit to a release gate's
constant, which is what a freeze is.

**W3 — held, with a control.** `artifact-gate.sh`'s deny branch now reads
`if [ "$code" -eq 4 ] && [ -n "$out" ]`. I ran the stub test myself, three cells: a stub exiting 4
with nothing on stdout draws the explicit allow at wrapper exit 0, where it previously emitted a bare
newline and no decision; a stub exiting 4 that does print draws that deny through unchanged, so the
guard did not disable the branch; and the real binary's deny still reaches stdout verbatim. The
wrapper's own exit was 0 in all three.

**W4 — held.** The Edit cell of
`an_existing_file_at_an_undeclared_name_is_editable_while_a_new_one_still_denies` now asserts
`rendered.contains("notes.md")` beside `additionalContext`, with a comment naming the ledger's gap
as the limb it pins. `cargo test --all` is 457 passed, 0 failed, and that test is in the run. I also
re-ran the cell through the release binary: the Edit allow's `additionalContext` names `notes.md`
and the declared set, so the assertion is pinning something real.

**W5 — held.** `reports/wave4-cycle-report.md` gained a `## The shell parse's known gaps, disclosed`
section and a frontmatter line counting three. I reproduced its whole table against the shipped
binary rather than accepting it: the absolute-path control denies at exit 4, and `cd` into the home,
a target held in a shell variable, and a relative write from a `cwd` already inside the home each
allow. The section's closing claim checks out too — a `Write` payload for the same file still denies
at exit 4, so the path arm is unaffected and only the text scanner is best-effort. The framing is
right: this is D1c's stated limit, disclosed, not a defect repaired.

**W6 — held, checked last per the lead's sequencing.** `sandbox.py`'s header no longer claims the
three names moved unchanged. It now says `SANDBOX` and `sbx_sh` are verbatim and names all three
ways `claude_args` differs: the dropped `LOCAL_MODE` limb that inserted `--bare` for the runner's
hermetic local mode, the defaults `stream` and `plugin` gained, and `acceptEdits` moving to the
`PERMISSION_MODE` constant at the same value. I checked every claim rather than reading them.
`LOCAL_MODE` now appears in the file only inside that docstring, never as code. The defaults are
`True` and `None` and the pre-convergence parameter order is intact. There are exactly two
`claude_args` call sites in `run.py` and both pass every parameter by keyword, so the defaults are
never exercised, as the header says. The decisive claim is the byte-identical argv, and I tested it
directly: I reconstructed the original function from `git show 32c1ed5:evals/run.py` in an isolated
namespace with `LOCAL_MODE` false, and its argv equals the shipped one across three shapes —
streaming with a plugin, non-streaming without, and streaming without. The header is now an accurate
account of the copy, which is what a reader diffing against `32c1ed5` needs.

**One thing still stale, outside these six.** The cycle report's carried item 1 calls the reminder
golden "authored and unfrozen", which W2's fix discharged.

**Gates on this run:** `cargo test --all` 457 passed 0 failed · `python3 evals/contract/run.py
--host-only` 7/7 cases, 5 measurements recorded, re-run after the `sandbox.py` edit · both Python
files compile.

**Status: CLEAN** — 6 of 6 held.
