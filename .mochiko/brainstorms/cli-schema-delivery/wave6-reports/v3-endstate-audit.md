# V3 — cold audit of P3's unit: the contract suite (wave 6, `cli-schema-delivery`)

**PASS — 2026-09-05.** Unit graded: `evals/contract/run.py` (mtime 11:59) and
`evals/contract/README.md` (14:35), against P3's report (14:35), the wave-6 plan's P3 charter,
`p3-endstate-plan.md`, the wave-5 V3 audit, `record.md` §"Wave 6", the run output `full-run-2.txt`
(1,692 lines, `EXIT=0`), and the 82 evidence directories under `evals/.work/` written between
12:36 and 14:00 today. Every figure below was re-derived from the evidence or from a fresh run;
none was read off P3's report. I authored none of this.

Own runs: `--host-only` exit **0** (4/4 cases, 349 `ok`, 4 `rec`); an independent subset re-run of
`preload` + `review-specifications-delivery` exit **0** (2/2, 7 `rec`, 5 metered sessions); three
scratchpad proof scripts of my own; P3's `prove.py` re-run green (16/16).

Findings F1 and F2 are defects in **P3's report and in `record.md`**, not in the unit. Neither
changes `run.py` or `README.md`. Both must be corrected before the landing.

---

## 1. Run-wide no-Read keying — PASS

`sweep_evidence` (`run.py:880`) globs `*.jsonl` in the case's own evidence root, so it reads
`stream*`, `transcript*` and `sidechain*` alike; nothing is passed in. Structural, not textual:
`schema_reads_in` (`run.py:824`) walks `tool_use` blocks through `tool_uses` (`run.py:726`) and
splits gating reads (`Read`, `NotebookRead`, `Bash` `.yaml` tokens, content-mode `Grep`) from
recorded listings (`Glob`, name-mode `Grep`). No channel captured ⇒ failure (`run.py:906`).

Call sites re-derived by enclosing function: `case_delivery:2762`, `case_command_absence:3164`,
`case_skill_absence:3260`, `case_preload:3426`, `case_brainstorm_skew:3503`,
`case_brainstorm_hooks_off:3557` — 36 + 6 + 30 + 1 + 1 + 1 = **75 wide sweeps**.
`case_brainstorm_policy:3620` calls `sweep_evidence(...)[0]` inside `report()` — **1 recorded**.
The two fixture cases keep `assert_no_schema_read(events)` at `run.py:1193` (`absence`) and
`run.py:1257` (`skew`) — **2 narrow**. Run output confirms the split: 75 / 2 / 1, and zero
`schema paths listed but never read` rows.

Sweep reached the preload sidechain. Evidence directory `evals/.work/contract-preload-37c44e3e`
holds 7 channels: `stream-{present,absent}.jsonl`, `transcript-{present,absent}.jsonl`,
`sidechain-present-{1,2}.jsonl`, `sidechain-absent-1.jsonl`. Its `verdict.json` carries
`no schema file was read on any captured channel · ok`.

Bite, proved independently. My `v3_sweep_proof.py` copies that real evidence directory, sweeps the
untouched copy clean, then plants one `Read` of `plugins/mochiko/schemas/common.yaml` into the
copied `sidechain-present-2.jsonl` and again into `sidechain-absent-1.jsonl`. Both fail, named by
channel, and reach the check list as a single `fail`. Restoring the copy restores the clean sweep.
A schema path appearing only in prose text does **not** fire — the structural claim holds. The
evidence itself was never touched.

P3's `prove.py` re-runs green: 16/16, including the empty-directory failure, the `Glob`/name-mode
`Grep` recording, and the relative-path `schemas/common.yaml` case that `is_schema_path`
(`run.py:789`) was widened for.

## 2. Host dead-phrase limb — PASS on the limb, FAIL on P3's figure (F1)

`DEAD_PHRASES` (`run.py:547`) is checked per line of every section render inside
`case_render_ceiling` (`run.py:2452`). Subject is the render, never the log. The case's
`verdict.json` for the graded run carries `"dead_phrases": {"found": []}` across 252 measurements
= 36 primitives × 7 sections. My own `--host-only` run reproduces:
`no rendered rule still names a shipped schema file · ok`.

Bite reproduced on a copy with `migrations/0003-two-arm-to-cli.yaml` removed. My count, using the
case's own line-level counting, is **32 findings across 18 distinct section renders, 14
primitives** — `plugins/mochiko/schemas/` 24, `when the binary is absent` 8. Six renders carry
both phrases. P3's "24" is `prove.py`'s render×phrase pair metric (18 + 6). See F1.

## 3. `deliverables` host case — PASS

`case_deliverables` (`run.py:3831`) stages the plugin, runs `views emit --out`, discovers
`templates/*`, `shelves/*`, `labels/*` from the emitted layout, then cross-checks against
`TEMPLATE_NAMES` (8) and `DOC_NAMES` (3) in **both** directions via `set_delta` (`run.py:3823`).
19 invocations: 8 templates × {producer, `--check`} + 3 documents through `doc`. Each owes
`render_shape_problems` — exit 0, empty stderr, non-empty stdout. `doc` additionally owes a head
line opening `mochiko-cli doc <name> · ` and carrying all of `TRIPLE_MARKERS`, and a closing line
exactly `mochiko-cli doc end · <name>`. `template` is held to its own shape (`# ` title first,
`TEMPLATE_FOOTER` last) per the record's wave-6 ruling.

My `--host-only` run: exit 0, 4/4 cases, **349 `ok`**, 4 `rec`; the case emits all 19 rows and
`deliverables rendered — 8 templates × 2 views through `template`, 3 through `doc` — 19
invocations, no session`. `prove.py` shows `set_delta` fires in both directions, an unknown
document name fails `render_shape_problems`, a template's last line is the footer and not a `doc`
end line, and a `doc` head line carries all three markers with an exact end line.

## 4. Numbers reproduce — PASS

Every figure below re-derived from `full-run-2.txt` or the evidence directories.

| figure | P3 | re-derived |
|---|---|---|
| cases | 82/82 | `contract suite: 82/82 cases passed, 82 ran` |
| assertions | 1,109 `ok` · 0 fail · 0 pending · 285 `rec` | 1,109 `ok`, 285 `rec`, no `FAIL`/`pend` tokens |
| sessions | 151 (149 transcripts, 3 sidechains) | 82 evidence dirs: 151 `stream*`, 149 `transcript*`, 3 `sidechain*` |
| no-Read split | 75 wide + 2 narrow + 1 recorded | 75 / 2 / 1 in the run output; 0 listing rows |
| criterion (1) rows | 36, all N/N | 36 rows; ids sum 336 |
| floors per family | 110 · 78 · 67 · 46 · 35 · preload 8 | 110 · 78 · 67 · 46 · 35; preload verdict `floor_ids` 8 |
| read-back | 108/108 both lines | 36 rows of `count 3/3 … ids 3/3 exact` |
| latency | means 27–29 ms, worst 42 ms, fire 180–197 ms | 36 latency lines: means 27–29, max worst 42, fire 180–197 |
| command delivered | 11,202 · 19,021 · 17,556 · 36,265 · 16,507 · 19,466 | identical, `read_cost.per_replicate[0].bytes` |
| render − delivered | 7 bytes on all six | 7 on all six (render totals 11,209 · 19,028 · 17,563 · 36,272 · 16,514 · 19,473) |
| largest renders | `impl.sec.tools` 15,499 B · `authoring-constitution.sec.artifact` 9,838 B | identical; 15,332 and 9,716 chars, 51.1 % and 32.4 % of the 30,000-char ceiling |
| family bytes | 121,579 · 151,781 · 117,823 · 91,309 | identical |
| family pre-conversion bytes | 125,499 · 156,070 · 102,016 · 85,143 | identical |
| family chars | 119,581 · 149,564 · 115,981 · 90,099 | identical, `per_replicate[0].chars` + `len(SKILL.md)` |
| vs F3 | −0.3 % · −0.7 % · +21.0 % · +10.1 % | identical |
| family rendered | 90,603 · 107,984 · 79,978 · 55,626 | identical |
| provenance | plugin 0.106.0 | renders report `plugin 0.106.0`; `plugin.json` `"version": "0.106.0"` |

Channel shapes reconcile the session count exactly: 36 delivery cases at 3 streams + 3
transcripts, 39 single-session cases at 1 + 1, 2 fixture cases at 1 + 0, `preload` at 2 + 2 + 3,
4 host cases at 0. No session-bearing case captured nothing.

Legend counted from the binary, not from the report: `rules brainstorm --section preamble` prints
a **nine**-bullet legend; `rules review-specifications --section preamble` prints a **six**-bullet
legend and carries no `vars` or `moments` block. P3's claim 5 is exact.

Note the char columns are `per_replicate[0].chars` plus `len(SKILL.md)` **as text**, not
`body_bytes_new`. Using the byte size instead yields 119,863 / 149,966 / 116,318 / 90,356 — P3's
figures are the correct ones.

## 5. Freeze handling — PASS with a documentation gap (F3)

`expected-skills.json` is unchanged this wave: `git diff HEAD -- evals/` touches only `README.md`
and `run.py`; the file's mtime is 2026-09-04 20:49, predating every session of the graded run.

The frozen floor sets still match the current renders. `converted-shape` asserts this in both
directions per primitive, and my `--host-only` run carries **36** `the pre-registered floor set
matches the <name> render (N ids)` rows and **36** `the `floors:` line agrees with the section
renders` rows, all `ok`. That is the substantive check, and it is green.

`freeze_expectations.py --verify` against the default plugin root exits 1 — see F4. The README
carries the pre-v0.107.0 plugin-root recipe (`README.md:590–596`). What it does not carry is the
remedy when a migration moves a floor set — see F3.

## 6. Sanitization (GI-003) — PASS

No session ids, home-directory transcript paths, tokens or credentials in P3's report, in
`README.md`, or in the `run.py` diff. The only `session_` hit in either document is the function
name `session_output_with`. Every write `run.py` makes is under `staged.root`, which is
`WORK / contract-<case>-<id>` with `WORK = REPO/evals/.work` (`run.py:97`, `run.py:1072`) — the
stub `SKILL.md`, the stub command, the skew log, the symlinked `bin/`, and the policy home
included. `evals/.gitignore:1` is `.work/`, and `git check-ignore -v evals/.work/` confirms the
directory is ignored. Sandbox transcript paths recorded in `preload`'s `verdict.json` are
container paths and never leave `evals/.work/`; the printed check reports counts only.

## 7. Independent subset re-run — PASS

The runner's parser (`run.py:4054–4062`) accepts only `--list` and `--host-only`. **Case selection
is not supported.** I drove a subset through the module's own helpers instead — `load_runner`,
`preflight`, `build_binary`, `sandbox_path`, then `run_cases` over a two-case slice of
`SANDBOX_CASES`.

**Exit code 0.** `subset: 2/2 cases passed, 0 pending, 7 recorded.` Assertion lines:

- `review-specifications-delivery` — all 7 head lines, all 7 end lines, end-line count matches the
  preamble section list, `!` lines expanded, no Bash denial, SessionStart hook reported the binary,
  six commands registered, dependency hook confirmed presence, init event registered the skill,
  **no schema file was read on any captured channel**, `floors:` line agrees (8 ids), case-time
  floor set matches the frozen expectation, every one of the 8 floor rules delivered.
  Recorded: read-back 3/3 · 3/3; delivered 12,732 bytes / 12,489 chars vs the 14,160-byte baseline,
  −10.1 % — byte-identical to the graded run.
- `preload` — all 7 blocks to the subagent, 8/8 floor rules, nothing delivered with the binary
  absent, no version triple in either transcript, **no schema file was read on any captured
  channel**; 2 present sidechains + 1 absent, and the parent explaining the failed preload shell
  command. The wave-5 shape reproduces.

## 8. Plan conformance and scope — PASS

Scope is clean. `git status --porcelain -- evals/` shows exactly two modified files,
`evals/contract/README.md` and `evals/contract/run.py`; no untracked files under `evals/` outside
the ignored `.work/` and `__pycache__/`. Under `wave6-reports/`, P3's files are `p3-endstate.md`
and `p3-endstate-plan.md`. `freeze_expectations.py`, `expected-skills.json`, `diagnostic.py` and
`fixture/` are untouched.

The `run.py` diff (384 insertions, 46 deletions) is confined to the charter: the sweep replacing
the per-replicate and per-state stream asserts at six call sites, `brainstorm-policy` reading the
same sweep through `report()`, the dead-phrase limb folded into `render-ceiling` at zero extra
render, the `deliverables` case and its two helpers, the hook-input rewording, and docstrings. No
unrelated behaviour changed.

Plan §4.2 required head and end lines of `template` as well as `doc`. P3 disclosed the conflict in
the plan; `record.md` rules `template` keeps its shape this wave with the wrap booked as a
follow-up. Applied as ruled.

Plan §4.3 states ≈80 sessions; the run took 151. P3's plan §3 disclosed the arithmetic before
approval (a delivery case is three replicates), and the count is wave 5's unchanged. Disclosed,
not silent.

Leave-alone hook row reworded, not removed. `unconverted_primitive` (`run.py:1548`) still supplies
the subject; the staged command stub survives (`run.py:2089`); check names are now `rule-less`
(`run.py:2176`, `run.py:2179`). My host run: `rule-less command \`contract-unconverted\` is left
alone` and `rule-less skill \`analysis-iterative\` is left alone`, both `ok`, with the row
provenance still recorded.

## 9. README honesty — PARTIAL (F2)

`git diff HEAD -- evals/contract/README.md` is 16 hunks. Fifteen are accounted for by §4's
disclosure: the host-case count, the assertion-table no-Read row and its three new paragraphs, the
case table's `render-ceiling` rewording and `deliverables` row, the 82/151 totals, the preload
section's paragraph on why the rewrite was forced, the hook-input leave-alone rewording and its new
justification paragraph, the channels table and its note, the read-cost "baselines are history"
paragraph, the Evidence section's assertion-subject paragraph, the `--verify` recipe under Running
it, and the closing "reads no schema file either" line.

All eleven §4 items verified correct:

1. five command delivered figures — reproduce;
2. `brainstorm` delivered unchanged — reproduces (11,202, and its render total 11,209 also unmoved);
3. four family converted-byte figures — reproduce;
4. `authoring-constitution.sec.artifact` 9,838 B / 32 % — reproduces (9,716 chars);
5. `impl.sec.tools` 15,499 B / 51.1 % — reproduces (15,332 chars);
6. latency 27–29 / 42 / 180–197 ms — reproduces;
7. the five commands' `rendered` and `largest render` cells — reproduce;
8. the family `rendered` column — reproduces;
9. the provenance line 0.106.0 after `0003` — reproduces;
10. "wave 6's, measured on the run that closed the wave" — correct;
11. the wave-6 preamble, nine-line command legend and six-line skill legend — counted from the
    binary, correct.

Three further figure movements in "Measured figures" are outside §4's list — see F2. Every value
in them is correct.

---

## Findings

**F1 — MEDIUM. The pre-`0003` dead-phrase count does not reproduce as worded, and has propagated
into the record.**
`.mochiko/brainstorms/cli-schema-delivery/wave6-reports/p3-endstate.md:74` ("the dead-phrase limb
finds **24 section renders carrying a phrase**") and `:168` ("24 phrase-carrying renders");
`.mochiko/brainstorms/cli-schema-delivery/record.md:1510` ("24 found when 0003 is removed from a
copy").
Re-derived on a copy with the migration removed: **18** distinct section renders carry a phrase,
across 14 primitives; the case's own `dead_phrases.found` list would hold **32** entries, one per
line per phrase (`plugins/mochiko/schemas/` 24, `when the binary is absent` 8). The 24 is
`prove.py`'s render×phrase pair metric — 18 renders plus the 6 that carry both. The limb itself is
correct and green; only the reported figure is wrong.
Fix: in both report lines, state "18 section renders, 32 line-level findings"; correct
`record.md:1510` to the same figure at the landing.

**F2 — MEDIUM. §4's "Five things moved beyond the six figures … Nothing else in the README was
touched" is incomplete.**
`.mochiko/brainstorms/cli-schema-delivery/wave6-reports/p3-endstate.md:132` and `:153`.
Three further figure groups moved in "Measured figures" and are named by neither the six-figure
table nor the five forced cells:
- the `vs baseline` column of the five moved command rows (`README.md:500–505`): −16.7 → −17.4,
  −15.8 → −16.5, −17.8 → −18.1, −16.7 → −18.5, −15.0 → −16.9;
- the family table's `converted ch`, `Δ B` and `Δ ch` columns (`README.md:522–525`);
- a new two-sentence F3 comparison in the criterion-(2) prose (`README.md:530–533`).
All are correct and all follow from the disclosed six, but the claim as written is false. Fix:
extend the §4 list to name them, or replace the closing sentence with "nothing outside the sections
§4 names".

**F3 — LOW. The README states the consequence of a moved floor set, never the remedy.**
`evals/contract/README.md:245–247`: "the freeze cannot be edited to match a render that changed, so
a floor rule added or renamed later breaks a check instead of quietly regrading." Nothing else in
the file says what a maintainer does next, and the read-cost section (`:474–478`) refuses
re-freezing twice over. A maintainer landing a migration that adds a floor rule is left with a
failing `converted-shape` and no stated path.
Fix: one sentence beside `:247` — a migration that moves a floor set takes a new pre-registration
recorded by ruling, not an edit to the frozen file, and name where that ruling lands.

**F4 — LOW. `freeze_expectations.py --verify` dies with an unhandled traceback on the default
plugin root.**
`evals/contract/freeze_expectations.py:124` (`common.stat().st_size`) raises `FileNotFoundError`
for `plugins/mochiko/schemas/skill-review-common.yaml`; exit 1, no stated reason. P3 did not touch
this file and the README carries the pre-v0.107.0 recipe, so this is pre-existing, but the
documented command now fails ugly for the first maintainer who runs it as written.
Fix: guard the missing-root case with the message the README already gives — the plugin root must
predate v0.107.0.

---

# Delta grade — F1, F2, F3 (2026-09-05, 14:57)

**PASS.** All three findings are closed. Graded from the rewritten
`p3-endstate.md` (14:55), the rewritten `evals/contract/README.md` (14:54), the lead's
`record.md` edit (14:53), and a fresh `git diff HEAD` of the README. One new finding, F5, LOW, on
the F3 remedy's field scope. F4 stays open and is unchanged.

**Untouched, confirmed by mtime and by diff.** `run.py` 2026-09-05 11:59:28,
`freeze_expectations.py` 2026-09-04 22:12:27, `expected-skills.json` 2026-09-04 20:49:47 — all
predate the delta. `git diff HEAD -- evals/contract/run.py` is byte-identical to the diff I graded
(md5 `0fcf0409…`). The README diff grew from 193 changed lines to 203; the ten are the two F3
additions and nothing else.

## F1 — CLOSED

`p3-endstate.md:74` and `:191` now read "32 line-level findings across 18 section renders in 14
primitives … 24 of `plugins/mochiko/schemas/` and 8 of `when the binary is absent`, with 6 renders
carrying both". Every figure matches my re-derivation exactly. The §2 correction paragraph names
the cause correctly: `prove.py` computed `sum(phrase in render for phrase in DEAD_PHRASES)`, a
render×phrase pair metric, and 24 is 18 plus the 6 double-carriers; the limb's own counting is per
line per phrase, which is 32. "Walking all 252 section renders" is right — 36 primitives × 7
sections, the count in `render-ceiling`'s `measurements`.

`record.md:1510` now reads "18 section renders across 14 primitives, 32 line findings, when 0003
is removed from a copy — V3 F1's re-derivation". Correct. Cosmetic only: that line runs well past
the file's wrap.

## F2 — CLOSED

Re-diffed the README against `HEAD`: 17 hunks, and the two "Measured figures" hunks are unchanged
from the ones I graded. Every changed line in them maps to one of the six figures or eight regions:

| changed line | maps to |
|---|---|
| "wave 5's, re-measured under the `floors:` line" → wave 6's | region 7 |
| five command rows, `rendered` and `largest render` cells | region 4 |
| five command rows, `delivered` cells | figure 1 |
| five command rows, `vs baseline` cells | region 1 |
| `brainstorm` row | unmoved — figure 2 |
| "the largest at 51.5 %" → 51.1 % | figure 5 |
| section header: wave-6 preamble, nine- and six-line legends | region 8 |
| section header: plugin 0.106.0 after `0003` | region 6 |
| family table `rendered` column | region 5 |
| family table `converted B` column | figure 3 |
| family table `converted ch`, `Δ B`, `Δ ch` columns | region 2 |
| family table `pre-conversion B` / `pre-conversion ch` | unmoved — correctly, both are frozen |
| the new F3 comparison in the criterion-(2) prose | region 3 |
| `authoring-constitution.sec.artifact` 9,938 → 9,838, 33 → 32 % | figure 4 |
| `impl.sec.tools` 15,617 → 15,499 | figure 5 |
| the latency paragraph, figures and its comparison sentence | figure 6 |

Nothing in the section is unaccounted for. Two notes on the accounting rather than on the file:

- **P3 is right and I was wrong on the F3 sentence count.** It is one sentence spanning four lines,
  not two. My F2 said two. Region 3 is correct as P3 states it.
- The latency paragraph's explanatory sentence was rewritten as well as its figures — "The spread
  is wider than wave 4's …" became "The band is far tighter than wave 5's — means of 26 to 86 ms
  and a worst run of 159 ms …". That is inside figure 6's own paragraph and carries the wave-5
  figures the table's `was` column already gives, so it falls within the region rather than outside
  the eight. The wave-5 figures it quotes are the ones the old README carried; the binary is
  unchanged across both waves, so "the same binary" is accurate.

## F3 — CLOSED, remedy adequate, one clause missing (F5)

Two additions, both outside "Measured figures", both disclosed in §7: a seven-line paragraph at
`README.md:249–255`, beside the criterion (1) refusal, and three lines appended to the read-cost
refusal at `README.md:487–489` pointing back at it.

**Both guards exist where P3 implies, and I fired one.** `freeze_expectations.py:207`
(`if args.out.exists()`) and `:215` (`if converted:`), the latter over the test at `:213`
(`CONVERTED_MARK` = ``!`mochiko-cli rules``, defined at `:76`). Run against the converted tree with
`--out` pointed at my scratchpad so the output-file guard could not mask it, the script exits **1**
with `skills are already converted ([...])` and writes nothing. It names **30** skills, exactly the
30 in `expected-skills.json` — so §7's "all thirty do" is exact, and the claim that a converted
tree cannot regenerate the file is true rather than argued.

**The remedy is honest about what a maintainer actually does.** It does not send anyone to a script
that would refuse them, which the finding's own proposed wording risked. Pointing `--plugin-root`
at a pre-v0.107.0 worktree does not help either: such a tree by construction lacks the migration
that moved the floor set, so it can only reproduce the old bar. Landing the replacement set with
the migration, by ruling, under the same author≠grader audit, is the only path the tree supports
and it matches the repo's own posture.

**Ruling: adequate, not a finding in itself.** A frozen set no tool regenerates is the direct cost
of the guards, and the guards are the reason criterion (1) is a bar at all; mechanising the
replacement would mean re-deriving the set from the render it grades, which is the exact failure
the wave rejects. The git trail is the durable control and it does record a replacement — the
`converted-shape` freeze-mtime observation would not, since a replacement still predates the next
run. What is missing is narrower, and cheap — see F5.

One imprecision, not worth a finding: `README.md:253` says "a plugin root that satisfies
`freeze_expectations.py`'s two guards", but the first guard is about the output file and is
independent of the plugin root. The enumeration that follows it is correct.

---

## New finding

**F5 — LOW. The ruling-authorised replacement has no stated field scope.**
`evals/contract/README.md:249–255`.
The remedy authorises a new pre-registration but never says which fields of
`expected-skills.json` may move. `floor_ids` and `floor_pin` legitimately do; `schema_bytes`,
`common_bytes`, `baseline_bytes` and `body_bytes_pre` are frozen historical constants whose source
files were deleted at v0.107.0 and must never move. Nothing mechanical distinguishes the two:
`--verify` cannot, because it needs a pre-v0.107.0 root that by construction lacks the new
migration, and `converted-shape`'s freeze-mtime observation cannot, because a replacement still
predates the next run. The audit is the only control, and it is not currently told what to look at.
Also, `README.md:245–247` still states flatly that the freeze "cannot be edited", two lines before
the paragraph that authorises replacing part of it — reconcilable in intent, jarring in sequence.
Fix: one clause on `:255` — a replacement moves `floor_ids` and `floor_pin` only, the byte columns
being frozen constants that never move, which makes the audit a diff check; and soften `:245–246` to
"cannot be edited to match a render that changed — only replaced by ruling".

**F5 — CLOSED (2026-09-05, 15:02).** `README.md:245–247` now reads "cannot be edited to match a render that changed, only replaced by ruling"; `README.md:256–260` adds the field scope — `floor_ids` and `floor_pin` move, `schema_bytes` · `common_bytes` · `baseline_bytes` · `body_bytes_pre` never do, and the audit is a field-scoped diff (the two floor fields against the migration's own render, every byte column byte-identical to the row it replaces); verified against `expected-skills.json`, where all 30 rows carry one identical nine-key set splitting exactly 2 movable + 4 frozen + 3 descriptive (`family` a label, `common` a path, `baseline_source` a prose provenance line — correctly left out of the byte list); the README diff against my last graded state is those two changes and line-number shift alone, and `run.py` (11:59:28), `freeze_expectations.py` (Sep 4 22:12:27) and `expected-skills.json` (Sep 4 20:49:47) are untouched by mtime; the flagged "a plugin root that satisfies `freeze_expectations.py`'s two guards" at `:253` stays a **non-finding** — the attribution is loose but the enumeration after it is correct and a reader acting on it reaches the true conclusion; two nits, neither blocking: `body_bytes_pre` is the pre-conversion `SKILL.md` size, so "measured from files deleted at v0.107.0" is loose for one of the four (the file ships, its pre-conversion body does not — the read-cost section states this correctly), and the new sentence has no blank line before it at `:256`, so it renders inside the preceding paragraph rather than as its own. **Verdict on the unit stands at PASS.**
