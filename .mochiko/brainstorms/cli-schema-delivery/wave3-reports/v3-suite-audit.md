# V3 — contract-suite audit (wave 3, `cli-schema-delivery`)

> **Superseded by the dated delta re-audit at the end of this file: PASS, 2026-09-04.** The original FAIL below is kept as the record of what was found.

**Verdict: FAIL (original pass).** Seven of nine items pass. Item 3 fails on measured evidence; item 9 fails only because that same deviation is undisclosed. Everything the lead needs for the abort
criteria is sound and I reproduced it independently: my own full run is exit 0, 10/10, read-back 3/3, delivered 10,839 bytes. The fix is roughly five lines. Graded on the code, the
evidence on disk, and my own re-run — never P3's report alone. Binary under test: `mochiko-cli 0.1.0 · grammar 1..1`. Five advisories are collected at the end.

## 1. Bar integrity — PASS

`READ_BACK_BAR = 3`, `READ_BACK_REPLICATES = 3` (`run.py:105-106`), matching plan §9 Q-D, pre-registered before the first session. I enumerated the `class: floor` rules straight from
the binary across all six sections: seven, and the set is exactly `FLOOR_IDS` and exactly plan §0's row (`user-record-acceptance` · `author-grader-default-fail` · `transport-floor` ·
the four `fail.*`). The scorer strips backticks and nothing else — I fed it seven mutations: bare and backticked pass; quoted, six-of-seven, seven-plus-one, and a missing `FLOOR:` line
all fail; `**FLOOR:**` passes. Reported, never gating: the score reaches the check list only through `report()`, `run_cases` folds only `status == "fail"` into `failures`, and `main`
returns `EXIT_ASSERT` solely on that counter.

## 2. Assertions keyed to measured shapes — PASS on all five sub-items

**(a) Delivery.** Seven head and seven end lines are checked in the session transcript, with expected ids and per-section counts read from the preamble render rather than from
constants. The head-line matcher drops placeholder-shaped ids: by mutation, the halt clause alone yields the empty set, clause-plus-one-real yields exactly the real one, and the quoted
end-line template (`<N> rules`) matches nothing. Dropping a block, corrupting one count, and passing a `!` line through literally each turn their own check red. The no-Read check
matches any `.yaml` under a `schemas/` directory as well as the `schema.yaml` suffix, which the staged copy needed.

**(b) Absence.** `assert_halt_before_model` is keyed to the measured shape: no assistant event, `num_turns` 0, and a non-empty `result` accepted only when it starts with
`UserPromptExpansion operation blocked by hook:` — any other result text fails, which is the load-bearing half. The install line is asserted. That no `<local-command-stderr>` is
injected is recorded rather than asserted, deliberately, so the assertion survives whichever limb wins the race; `halt_shape` and the `which limb halted first` record keep it readable.
Accepted.

**(c) Skew.** `swap_plugin_log` replaces the staged plugin's own `migrations/`, proved before the session by a direct binary run against that root asserting exit 3, empty stdout, and
the D5 message — my run's evidence file reads `0001-skew.yaml: the migration log is written in grammar 99, and this binary reads grammar 1..1.`

**(d) Hooks-off.** The wave-1 shape is asserted directly: an injected `<local-command-stderr>` block naming `mochiko-cli` and `command not found`, alongside no model turn. Both green in
my run.

**(e) Policy.** All six checks are `report`; the case contributes nothing to `failures`. The prose-halt check reads `final_assistant_text` only, so the clause quoting its own trigger
phrases cannot satisfy it.

## 3. No tolerant leftovers — FAIL

P3's plan §1 committed: "Until then they search the union of transcript, process stdout and process stderr and report the channel; a tolerant assertion is not the shipped form." The
probes ran and the shape was measured, but the union survived as the shipped form. `session_output` returns `transcript_text(events)` plus `proc.stdout` plus `proc.stderr`,
`session_output_with` adds the fetched transcript, and its docstring now states the union as permanent: "the union is what is asserted, the channel is evidence." Three gating positive
assertions read it — the install line in `brainstorm-absence` and `brainstorm-skew`, `grammar 99` in `brainstorm-skew`, and the SessionStart line in `brainstorm-delivery`.

The keying was available and was not done. Across seven recorded runs on disk plus my own, the `channels` record is invariably `["session-transcript", "stream-events"]` — never
`<local-command-stderr>`, never `process-stderr`. `proc.stdout` is the stream itself and wholly redundant with `transcript_text`; the extra reach is `proc.stderr`. Not cosmetic: a
regression in which the hook's message stops reaching the session but still lands on the CLI's own stderr would keep an assertion named "the install line reached the session" green —
exactly the false-pass class this item exists to rule out. **Fix:** assert against the session transcript plus the stream events, keep the process streams in `channels_of` as evidence
only, and correct the docstring. Negative assertions may keep the wider union; for those, breadth is strictness.

## 4. `pending` handling — PASS

No `pending()` call survives anywhere. The wave-1 entry is gone from `case_absence`, its docstring says where it went, and its subject is asserted in `brainstorm-absence` as "the
install line reached the session". In the summary path `pendings` is counted and printed separately and never folded into the passed tally. The `pending` helper is now unreferenced —
dead code unless wave 4 uses it.

## 5. Evidence completeness — PASS, one gap and one unreproduced figure

Every session case in my run holds `stream.jsonl`, `argv.txt`, `script.sh`, `stderr.txt`, `verdict.json` and its staged plugin; `transcript.jsonl` is present for the three delivery
replicates, `brainstorm-absence`, `brainstorm-skew` and `brainstorm-policy`. `brainstorm-hooks-off` has none, because the case never fetches one and nothing it asserts reads a
transcript — defensible, but it is the one brainstorm case a later reader cannot re-derive from disk. I recomputed rather than read: delivered blocks parse to 10,693 bytes and 10,506
chars in all three replicates, identical to the byte as claimed, and with the 80-byte SessionStart line and the 66-byte hook presence line that is 10,839 total, 15.4 % under the
12,819-byte baseline. Plugin sizes recompute exactly — 1,366,116 before the move, 626,780 for the log and its README, 1,992,896 after, genesis 618,122, growth 45.9 % — as does the
largest render at 2,055 chars / 2,102 bytes. Latency is the one set I could not reproduce; see V3-d.

## 6. Scope (GI-019 / GI-020) — PASS

`git status` after my full run shows no file changed under `plugins/mochiko/` beyond P1's move and P2's own work. P3's unit is `evals/contract/run.py` and `evals/contract/README.md` and
nothing else, with no new file under `evals/contract/fixture/`. Every perturbation acts on a staged copy, including the stub `SKILL.md` the hook-input case writes and the log swap. The
suite dispatches no agent — the headless `claude -p` sessions are the subject D8 prescribes, not orchestration — and grades no content: the read-back metric is an exact token-set
equality and never gates.

## 7. Independent re-run — PASS

`python3 evals/contract/run.py`, full, sandbox `claude-mochiko`: **exit 0**, `contract suite: 10/10 cases passed, 10 ran, 14 measurement(s) recorded and not asserted`. Read-back **3/3 —
MET**. Delivered read cost **10,693 bytes / 10,506 chars, -16.6 %** on the blocks alone, identical to P3's run. Not SKIPPED.

## 8. The tenth case, `converted-shape` — PASS, legitimate

Delivery mechanics, not content grading. It reads each converted `.md`, extracts its `!` lines with a strict line regex, and checks that every line renders its own primitive, that the
requested section list equals the list the primitive's own preamble render declares in the same order, and that the `allowed-tools: Bash(mochiko-cli *)` grant is present. Nothing is
judged about what a rule says. It closes a real hole — a `.md` enumerating six of seven sections delivers six well-formed blocks that every session assertion would pass — on the host,
before any metered run. Keep it.

## 9. Report honesty — FAIL on one omission; every other claim verified

All seven disclosed deviations check out against the diff and the evidence, including both self-found defects: the pre-fix policy verdict on disk reads `1 of 7 expected` and the two
post-fix ones read `0 of 7`, and the wave-1 no-Read form in the diff is `path.endswith("schema.yaml") or "plugins/mochiko/schemas/" in path`, which the staged path
`…/mochiko/schemas/brainstorm.yaml` matches neither way. The full suite really was re-run after the last code change: the newest complete case set postdates `run.py`'s modification time
and its policy verdict carries the post-fix figure. The tally, the 14 recorded observations, and the abort numbers all match my run. The omission is item 3 — the plan said in writing
that the tolerant union was interim, the shipped code keeps it, and no deviation entry says so.

## Advisories

- **V3-a.** `run.py:92-94` claims the pilot command and the floor ids are both verified against the binary's render "so a schema change breaks the check". True of the section ids and
  counts; `FLOOR_IDS` is a hardcoded frozenset never cross-checked, so a floor rule added at wave 4 makes the bar quietly wrong. Harmless while the metric never gates.
- **V3-b.** The limbs of `assert_no_schema_read` are asymmetric: the `Read` limb catches `skills/<name>/schema.yaml` through `is_schema_path`, while the `Bash` limb's separate regex
  needs a `/schemas/` segment and returns `None` for the same file — verified both. No live risk at wave 3, since no skill is converted; it matters from wave 4.
- **V3-c.** `brainstorm-hooks-off`'s "no version triple was delivered" check reads `session_output`, which excludes the session transcript — the one channel P3 measured as carrying
  delivered rules — so it cannot fail. Redundant rather than dangerous, since the halt check is decisive, but a vacuous assertion in this suite should read the transcript or go.
- **V3-d.** No artifact backs the latency figures: nothing under `evals/.work/` records timings and `run.py` measures none. Two independent sandbox passes give 42 to 77 ms per section
  and 648 ms per fire against P3's 27 to 54 ms and 458 ms — same order and same conclusion, but roughly half the reported band; sandbox load is the obvious explanation.
- **V3-e.** Three places still describe a nine-case suite: `run.py`'s module docstring omits `converted-shape` and says "The first two need neither a sandbox nor a session", and
  `README.md` says "Two cases need no session and no sandbox at all" and calls `hook-input` "the only one that needs neither sandbox nor session", while its own table and Running
  section correctly say three.
- **For the lead, not a defect.** The policy environment's prose clause has now held in one observation of three, not one of two: both post-fix runs on disk and my own re-run replied
  `FLOOR: none` rather than surfacing the halt line. It strengthens the GI-020 unsupported declaration rather than weakening it, but the number should be corrected before the record
  quotes it.

## Fix list

1. **Blocking.** Key the three gating positive assertions to the measured channels — session transcript and stream events — dropping the process streams from what is asserted while
  keeping them in `channels_of`. Correct the `session_output` docstring.
2. **Blocking.** Add the retained union to `p3-suite.md`'s deviation list, or remove it under fix 1 and say so.
3. Advisory: route the Bash limb of `assert_no_schema_read` through `is_schema_path` (V3-b); narrow the `FLOOR_IDS` comment or derive the set (V3-a); fix or drop the vacuous check in
  `brainstorm-hooks-off` (V3-c); state latency as load-dependent or emit it from the suite (V3-d); reconcile the nine-versus-ten case count (V3-e).

---

# Delta re-audit — 2026-09-04

**Verdict: PASS.** Both blocking items are discharged and all five advisories are closed. Graded on the reworked diff, the newest full-run evidence set (`contract-*` verdicts written 12:46:27 to 12:47:26, postdating `run.py`'s 12:45:44 modification time), and my own probes against the current code. I did not re-run the suite: every rekeyed assertion is judgeable from the code and the evidence, per the lead's instruction.

## Item 3 — no tolerant leftovers: now PASS

The union is gone from every positive assertion. `asserted_output` returns the stream events plus the session transcript and nothing else; `session_output` and `session_output_with` survive for negative assertions only, and each of the three docstrings states which role it plays. All four positive assertions I named now route through it — the install line in `brainstorm-absence` and `brainstorm-skew`, `grammar 99` in `brainstorm-skew` (`assert_message` is retired in its favour and is now dead code), and the SessionStart line, whose `_session_start_line` takes the transcript as a parameter. The three negative `assert_no_version_triple` calls keep the wide union, which is the correct direction.

I reproduced P3's red probes rather than taking them on report. A fragment present only on `proc.stderr` fails the assertion, and so does one present only on `proc.stdout`; both still show up in the channel record, and both pass when placed in the transcript or the stream events. The failure message even names where the fragment actually was.

*One cosmetic nit, not a defect.* On the stdout-only probe the failure message reads `it is on the process streams: []`, because `channels_of` has a `process-stderr` channel but no `process-stdout` one. The verdict is right; only the parenthetical is empty.

## Item 9 — report honesty: now PASS

Deviation 8 discloses the retained-then-removed union in full, including why it mattered, and deviations 9 through 13 disclose each advisory fix with its red probe. The rework preamble names the FAIL and concedes it. The tally claim verifies against the newest verdict files: ten cases, zero failed checks, fifteen reported, zero pending. The delivered figure recomputes to 10,693 bytes and 10,506 chars in all three replicates, plus an 80-byte SessionStart line and a 66-byte hook presence line, giving the reported 10,839.

The four-observation policy count is correct, and I checked it against the verdict files rather than the report. Four `brainstorm-policy` verdicts sit on disk: the prose clause held in the 12:23:52 and 12:47:26 runs and did not in the 12:25:39 run or in my own 12:32:40 re-run. Two of four, as reported.

## The five advisories — all closed

- **V3-a.** `converted-shape` now compares `FLOOR_IDS` against the `class: floor` ids the binary renders and reports both directions of difference. I ran `rendered_floor_ids` myself: seven ids, set-equal to the constant. An invented eighth id and a dropped seventh each turn the check red.
- **V3-b.** The Bash limb routes through `is_schema_path`. A shell `cat` of `skills/foo/schema.yaml` now returns a finding where it previously returned clean; the `/schemas/` form and a `.md` path still behave correctly.
- **V3-c.** `brainstorm-hooks-off` fetches its transcript and its negative assertion reads the union. The case's transcript is on disk at 2,216 characters and carries no version-triple markers, so the check now reads a channel that could have carried the thing.
- **V3-d.** `measure_latency` times ten runs per section inside the sandbox and writes `latency.json` with every individual run. The emitted artifact matches the reported band exactly: means 26 to 32 ms, worst single run 36 ms, whole fire 182 ms. The report presents all four passes side by side, including my two, and marks the figure load-dependent. A missing measurement degrades to "not measured" rather than raising.
- **V3-e.** The module docstring lists all ten cases including `converted-shape` and says "the first three"; the README says "Three of the ten cases", "one of the three", and "only the three that need no sandbox".

## Unchanged and re-confirmed

The exit path is untouched: only `status == "fail"` increments `failures`, and the new latency check is a `report`. `pending` is still counted and printed separately, and no `pending()` call exists — the helper is kept deliberately, disclosed in the report. Every mutation from my first pass still bites. `git status` shows no file changed under `plugins/mochiko/` by this seat, and P3's unit is still `run.py` and `README.md` alone. `--list` prints ten cases and exits 0.

Nothing blocks the landing from the suite side.
