# P3 — the end state: contract suite (wave 6, `cli-schema-delivery`)

**Status: built, and the full suite is GREEN on the frozen end-state tree.** Unit:
`evals/contract/**`, two files. Nothing outside the unit was touched and nothing was committed.
Plan of record: `wave6-reports/p3-endstate-plan.md`, approved with four rulings, all four applied.

| run | result |
|---|---|
| `--host-only`, P2's end-state tree | exit **0** — 4/4 cases, 349 assertions, 0 failures |
| **full suite**, same tree, after the user's `sbx login` | exit **0** — **82/82 cases, 151 sessions, 1,109 assertions, 0 failures, 285 recorded** |

Section 6 carries the tally and every figure, all of it recomputed from the run's own evidence
rather than read off the printed summary. Section 4 names the README figures this run now
contradicts; per the lead's freeze for V3's cold grade I named them and edited nothing.

## 1. What was built

**The no-Read assert went run-wide by reach, not by keying (§4.1).** It already ran in every
session-bearing case; what it could not see was the `preload` subagent, whose turns are in a
sidechain transcript and in neither the parent's stream nor the parent's transcript. So the one
case whose entire subject is a subagent was the one case where a fallback read was invisible to the
assertion written to catch it. `sweep_evidence(root)` now reads every JSONL channel the case
captured — `stream*.jsonl`, `transcript*.jsonl`, `sidechain*-N.jsonl` — all of which are already on
disk by the time a check list is built, so nothing is passed in and a case that grows a fourth
channel is covered by having captured it. A case that captured no channel **fails**: an assertion
with nothing to read has proved nothing.

The sweep is structural. `schema_reads_in(rows)` walks `tool_use` blocks and splits by what the
call returns: `Read`, `NotebookRead`, a shell read and a content-mode `Grep` hand back rule text and
gate; `Glob` and a name-mode `Grep` hand back paths and are recorded beside the assertion. A text
search was not an option and this is the reason to write down — a transcript carries the rendered
rules, the prompt and the model's prose, so searching it for a schema path would have fired on a run
that merely named one, which until migration `0003` the rules themselves did.

The two fixture cases keep the narrow `assert_no_schema_read(events)`: they halt before any model
turn and fetch no transcript, so the stream is the only channel they have. `brainstorm-policy` reads
the wide sweep but still through `report()`, so what it records is the same fact the others assert
rather than a narrower one.

**The host limb rides the renders `render-ceiling` already had.** No section render of any of the 36
primitives may contain `when the binary is absent` or `plugins/mochiko/schemas/`. Zero extra renders,
because that case already walks every section of every primitive and holds each `stdout`. The
subject is the **render**, never the log: the log is append-only, so `0001-genesis.yaml` carries the
two-arm wording by construction and always will, and only the replayed state has to be clean. The
failure names the primitive, the section and the offending line; the full finding list lands in
`verdict.json` under `dead_phrases`.

**`deliverables` is the fourth host case (§4.2).** No sandbox, no session. It stages the plugin, runs
`views emit --out <staged tmp>`, and discovers its subjects from the emitted layout: `templates/*`
through `mochiko-cli template <name>` and `--check`, `shelves/*` and `labels/*` through `mochiko-cli
doc <name>`. Discovery is then cross-checked in both directions against a written-down set, because
discovery alone would shrink silently if a document vanished from the log and a written-down set
alone would go stale when one was added. Nineteen invocations: 8 templates × 2 views, 3 documents.
Each owes exit 0, empty stderr, non-empty stdout; a `doc` additionally owes the head line opening
`mochiko-cli doc <name> · ` with all three version markers and the closing line exactly
`mochiko-cli doc end · <name>`.

**The leave-alone hook row was reworded, not removed.** Its justification was the transition clause;
it is now that a primitive with no rules of its own has nothing for a missing binary to have cost
it. The subject is unchanged and permanent: eight shipped skills carry no rules and never will.
Check names moved from "unconverted" to "rule-less"; the staged command stub stayed.

## 2. That it bites, proved rather than asserted

Sixteen proofs, all green, run against the real binary and the real log. They are in the scratchpad
as `prove.py` and reproduce in about a minute; the load-bearing ones:

- the sweep fails on a `Read` of a shipped schema, a `Read` of a skill schema, a shell read, and a
  content-mode `Grep` pointed at the schemas directory;
- **the sweep fails on a `Read` inside a `sidechain-1.jsonl`** — the case the widening exists for,
  proved on the channel the old form could not see;
- an empty evidence directory fails; a clean channel passes; a `Glob` and a name-mode `Grep` are
  recorded and do not gate, and reach the check list as one `ok` plus one `report`;
- **the dead-phrase limb reports 32 line-level findings across 18 section renders in 14
  primitives** when migration `0003` is removed from a copied plugin — 24 of
  `plugins/mochiko/schemas/` and 8 of `when the binary is absent`, with 6 renders carrying both —
  so the green result on the real tree is `0003` working, not the check looking at nothing;
- `set_delta` names both a document missing from the log and one nobody wrote down; an unknown
  document name fails `render_shape_problems`; a template's last line is the provenance footer and
  not a `doc` end line; a `doc` head line carries all three markers and its end line is exact.

**The dead-phrase figure above is a correction (V3 F1), and my re-derivation agrees with V3's.**
I first reported "24 section renders", which is not a render count at all: `prove.py` counted
`sum(phrase in render for phrase in DEAD_PHRASES)`, a render × phrase pair metric, and 24 is 18
renders plus the 6 that carry both phrases. The limb itself counts per line per phrase, which is
32. Re-derived on a fresh copy with `0003` removed, walking all 252 section renders with the
limb's own counting: 32 findings, 24 of `plugins/mochiko/schemas/` and 8 of `when the binary is
absent`, across 18 distinct renders in 14 primitives, 6 of them carrying both. Every figure
matches V3's independently. The limb was always correct; only my wording was.

**One real defect found and fixed, in code that predates this wave.** `is_schema_path` tested
`"/schemas/" in path`, so a **relative** `schemas/common.yaml` — the form a Read written from the
plugin root produces — matched neither limb and passed. It was the sidechain proof that caught it.
The leading-segment case is now its own test in both `is_schema_path` and `names_schema_source`, and
the docstring records why.

## 3. The run's shape, and the baselines it grades against

Run once against the frozen end-state tree, at the approved budget: **82 cases, 151 sessions**.
Four host cases, then 78 sandbox — 2 fixture, 6 + 6 command, 3 pilot mechanism, 30 + 30 skill,
`preload`. The session count is wave 5's unchanged, because wave 6 adds only a host case. The
tally and every figure are in §6.

**No re-freeze of `expected-skills.json` happened, and none was needed.** The file's mtime is still
2026-09-04 20:49, predating every session of this run, and the reason is structural rather than
lucky. Nothing
in `run.py` reads a schema file at run time: floor sets come from the binary, `baseline_bytes` and
`body_bytes_pre` are constants in the frozen JSON, `body_bytes_new` is a `stat()` on the staged
`SKILL.md`, delivered bytes come from the transcript. Migration `0003` and P2's deletions leave every
case runnable and every baseline intact. The read-cost columns therefore read against the frozen
wave-5 baselines, never re-measured from files that no longer exist. `freeze_expectations.py` stays
in the tree unrun as the reproducibility record; its `--verify` now needs a plugin root from before
v0.107.0, and the README carries the `git worktree add` recipe.

## 4. README

Everything the plan named, plus the measured figures in a second pass. The case table gains `deliverables` at
0 sessions and the totals become 82 cases / 151 sessions; the assertion table's no-Read row says
run-wide and has its own three paragraphs on reach, on why the sweep is structural, and on the
host-side dead-phrase limb; the preload section says why that case is what forced the rewrite; the
channels table notes the assertion now sits in neither column; the Evidence section states that the
`.jsonl` files are now an assertion subject and not only a record; the hook-input section carries
the leave-alone justification; the read-cost section states that the baselines are frozen constants
whose sources no longer ship, with the `--verify` recipe and why re-freezing is refused; "What is
not here" gains the line that the suite reads no schema file either.

**"Measured figures" carries this run's numbers.** The lead lifted the freeze for one pass over
that section; `run.py`, `freeze_expectations.py` and `expected-skills.json` stayed untouched
throughout. Six figures were flagged as contradicted and all six were updated. Every one moved
because migration `0003` reworded rule text, which is the wave's whole point, and none is a
regression.

| figure | was | now | source |
|---|---|---|---|
| command delivered: `architecture` · `feature` · `implement` · `setup` · `specify` | 19,186 · 17,707 · 36,391 · 16,865 · 19,910 | 19,021 · 17,556 · 36,265 · 16,507 · 19,466 | each delivery case's `verdict.json`, `read_cost.per_replicate[0].bytes` |
| command delivered: `brainstorm` | 11,202, −12.6 % | unchanged | same; its rules carried no two-arm phrase |
| family converted B: review · authoring · patterns · dense five | 123,570 · 154,143 · 120,144 · 92,531 | 121,579 · 151,781 · 117,823 · 91,309 | `read_cost.delivered_at_invoke_new` summed per family; the char columns add the staged `SKILL.md` length to `per_replicate[0].chars` |
| largest skill render `authoring-constitution.sec.artifact` | 9,938 B, 33 % | 9,838 B, 32 % | `render-ceiling`'s `verdict.json`, `measurements` |
| largest render overall `impl.sec.tools` | 15,617 B, 51.5 % | 15,499 B, 51.1 % | same |
| latency | means 26–86 ms, worst 159 ms, whole fire 183–531 ms | 27–29 ms, 42 ms, 180–197 ms | the `store latency` lines in `full-run-2.txt`, load-dependent label kept |

**Eight regions moved beyond the six figures, and each was forced by one of them.** Every one is a
statement about the numbers I changed, so leaving it would have put a false claim next to a true
figure — the old defect in new clothes rather than a fix. The first three were named late, after
V3 found the disclosure incomplete (F2); every value in them is correct, and the list below is now
the whole diff of the section against `HEAD`.

1. The `vs baseline` column of the five moved command rows: −16.7 to −17.4, −15.8 to −16.5,
   −17.8 to −18.1, −16.7 to −18.5, −15.0 to −16.9. A delta is a function of the delivered figure
   beside it, so it could not stay put.
2. The family table's `converted ch`, `Δ B` and `Δ ch` columns, for the same reason: only the two
   pre-conversion columns are frozen constants, and they did not move.
3. A new sentence of F3 comparison in the criterion-(2) prose, carrying the converted side's
   distance from the record's estimates — a claim that did not exist before because the converted
   figures had not been measured against F3. (V3 calls it two sentences; it is one, spanning the
   four lines the finding cites. Same region either way.)
4. The `rendered` and `largest render` cells of the five moved command rows. The section states
   that the delivered figure sits seven bytes below the render total; mixing wave-5 render totals
   with wave-6 delivered figures makes that sentence false. Verified after the edit: the gap is
   exactly 7 bytes on all six commands, and on `brainstorm` nothing moved at all.
5. The family `rendered` column, for the same reason, alongside the family table.
6. The provenance line, from `plugin 0.105.0 after migration 0002` to `0.106.0` after `0003` —
   otherwise the section attributes this run's numbers to a tree they were not measured on.
   0.106.0 is what the renders reported; the bump to 0.107.0 is the lead's landing step.
7. "The rendered and delivered columns are wave 5's, re-measured under the `floors:` line" became
   wave 6's, measured on the run that closed the wave.
8. "the wave-5 preamble … the nine-line legend" became the wave-6 preamble, nine lines on a
   command and six on a skill. The skill legend drops the two `enforces:` lines and the
   `moments:` line, and a skill preamble carries no `vars` or `moments` block, so the old phrase
   described the command case only. (I first wrote "one line" here off a truncated render read;
   the lead caught it. Counted from the binary: nine bullets on `brainstorm`, six on
   `review-specifications`.)

Nothing outside the regions this section names was touched in "Measured figures". The later F3
pass added the moved-floor-set remedy in two further places, both outside that section and both
listed in §7. `run.py`, `freeze_expectations.py` and `expected-skills.json` were not touched at
any point in the wave.

## 5. The host gate on the end-state tree

Run after P2 closed, with `plugins/mochiko/schemas/` gone, no `skills/*/schema.yaml`, and only
`similar-rules-allowlist.yaml` left under `scripts/`. Exit 0, 4 of 4 cases, 349 assertions, no
failures, no pending. Three of those results are worth naming because they are the wave's own
claims rather than carried-over ones.

- **The suite itself reads no schema file.** It ran to completion on a tree that has none. That is
  the cheapest possible demonstration of the wave-6 done condition from the suite's side, and it is
  now a fact rather than a design intention.
- **The dead-phrase limb is green across all 36 primitives.** No section render carries
  `when the binary is absent` or `plugins/mochiko/schemas/`, so migration `0003` covers every rule
  the renders expose. The same limb reports 32 line-level findings across 18 section renders in 14
  primitives when `0003` is removed from a copied plugin, so green is the migration working rather
  than the check looking at nothing.
- **`converted-shape` still matches all 36 frozen floor sets** against the renders in both
  directions with no drift, which is the signal that `0003` moved rule text without disturbing
  floor membership — the thing a mass reword is most likely to break silently.

`deliverables` passed all 19 invocations, and `hook-input` passed with the reworded leave-alone
rows against a real rule-less subject (`analysis-iterative`).

## 6. The full run: tally and figures

Run output: `full-run-2.txt` in this session's scratchpad, 1,692 lines, `EXIT=0`. Evidence:
`evals/.work/contract-*`, one directory per case. Every figure below was recomputed from those
directories rather than read off the printed summary; where the two agree I say so, and where the
summary omits a figure I derived it from the verdicts.

**Tally.**

| | |
|---|---|
| cases | **82 of 82 passed, 82 ran** |
| sessions | **151** |
| assertions | **1,109 `ok` · 0 `FAIL` · 0 `pending`** |
| measurements | **285 `rec`**, none of which can move an exit code |
| exit | **0** |

**Sessions: 151, and the 149 transcript files are not a shortfall.** Counting the evidence
directories gives 151 `stream*.jsonl`, 149 `transcript*.jsonl` and 3 `sidechain*.jsonl`. The
stream count *is* the session count, one per `claude -p` invocation, and it matches the budget
exactly: 2 fixture + 18 command delivery + 6 command absence + 3 mechanism + 90 skill delivery +
30 skill absence + 2 preload. The two sessions with no transcript are the fixture cases `absence`
and `skew`, checked directly: each holds `stream.jsonl` and no `transcript*.jsonl`. Neither ever
calls `fetch_transcript`, because both halt before any model turn — there is no session transcript
to fetch and their assertions read the stream. So the figure to carry is 151; 149 is a count of
transcripts, not of sessions.

**The no-Read sweep ran on 75 cases, and the other three are accounted for, not missing.** The 78
sandbox cases split as 75 + 2 + 1. The two fixture cases keep the narrow stream-only form under
its own name, `no schema file was Read`, because they capture no transcript for a sweep to read.
`brainstorm-policy` asserts nothing by design (D8), so its sweep result reaches the check list
through `report()` and reads `a schema file was read as a fallback — no`. All three were verified
by name in the run output. **Across all 78, no channel carried a schema read of any kind**, and no
`schema paths listed but never read` row appeared either, so nothing globbed one.

**Criterion (1): every floor rule delivered, on every primitive.** Thirty-six `every one of the N
floor rules was delivered (criterion (1))` rows, all N/N, none carrying a failure detail. By
family, and these are the frozen floor counts the sets were graded against:

| family | primitives | floor ids | result |
|---|---|---|---|
| commands | 6 | 110 (`implement` 34, `architecture` 22, `setup` 18, `specify` 16, `feature` 13, `brainstorm` 7) | 110/110 |
| review | 8 | 78 | 78/78 |
| authoring | 8 | 67 | 67/67 |
| patterns | 9 | 46 | 46/46 |
| dense five | 5 | 35 | 35/35 |
| `preload` | 1 (`review-specifications`, through the subagent) | 8 | 8/8 |

**Criterion (2), per family, delivered-at-invoke.** Bytes are the criterion; chars sit beside them
and never share a column. The pre-conversion side is the frozen wave-5 baseline, never re-measured
— its source files no longer exist in the tree.

| family | skills | converted B | pre-conversion B | Δ B | converted ch | pre-conversion ch | Δ ch | F3 ch | vs F3 |
|---|---|---|---|---|---|---|---|---|---|
| review | 8 | 121,579 | 125,499 | −3.1 % | 119,581 | 124,610 | −4.0 % | 119,895 | −0.3 % |
| authoring | 8 | 151,781 | 156,070 | −2.7 % | 149,564 | 154,950 | −3.5 % | 150,576 | −0.7 % |
| patterns | 9 | 117,823 | 102,016 | **+15.5 %** | 115,981 | 101,178 | **+14.6 %** | 95,858 | +21.0 % |
| dense five | 5 | 91,309 | 85,143 | **+7.2 %** | 90,099 | 84,584 | **+6.5 %** | 81,799 | +10.1 % |

Every family moved down against wave 5 by roughly a point and a half, which is migration `0003`
removing the two-arm clause from rule after rule. Patterns and the dense five still land above
their baselines for the reason the wave open pre-stated: neither family has a common schema file,
so the render's fixed overhead has nothing to amortise against. **Against the record's F3 char
figures** the two common-bearing families now sit essentially on top of F3 — review −0.3 %,
authoring −0.7 % — where at wave 5 they sat 1.4 % and 0.9 % above it. F3 was an estimate taken
before the wave, and the converted cost has now closed on it from above.

**The read-back is 108 of 108 on both lines**, across 36 primitives at 3 replicates. Wave 5 was
107 of 108, the single miss being a `specify` replicate that refused the probe as injected text.
It is recorded, gates nothing, and is not evidence about delivery — criterion (1) is.

**Preload, both shapes.** `devils-advocate` preloading `review-specifications`. Binary present:
the subagent received all seven blocks, every one of its 8 floor rules arrived, 2 turns, 2
sidechain transcripts fetched. Binary absent: zero blocks in either transcript, no version triple,
2 turns, 1 sidechain transcript, and the parent survived to explain that the spawn's preload shell
command failed because `mochiko-cli` was missing. That is the wave-5 measured shape reproducing
exactly, and the absent half remains recorded rather than asserted for the reason wave 5 gave.

**Latency**, timed inside the sandbox and load-dependent: per-section means of **27 to 29 ms**
across the 36 primitives, a worst single run of **42 ms**, and whole-fire figures of **180 to
197 ms** for all seven sections. Far tighter than wave 5's 26 to 86 ms and 159 ms worst, on a
quieter machine; the binary did not change, and every individual run is in each case's
`latency.json`.

**Two host results worth carrying beside the sessions.** The dead-phrase limb found **zero**
occurrences across every section render of all 36 primitives, so no rendered rule still offers a
schema file to read. And `deliverables` rendered all 19 subjects — 8 templates through the
producer and checklist views, 3 documents through `doc` — with the discovered set matching the
written-down one in both directions.

## 7. Deviations and open items

- **The README's "Measured figures" section was updated in a second pass** on the lead's lifted
  freeze, and §4 tables what moved and where each figure came from. `run.py`,
  `freeze_expectations.py` and `expected-skills.json` were never touched.
- **A fourth README pass landed V3's F5**, one clause on the replacement's field scope —
  `floor_ids` and `floor_pin` move, the four byte columns never do, so the audit is a field-scoped
  diff — and softened the criterion (1) refusal to "cannot be edited … only replaced by ruling".
- **A third README pass landed V3's F3**, the only remedy the file was missing: what a maintainer
  does when a migration legitimately moves a floor set. It sits in two places, both outside
  "Measured figures" — beside the criterion (1) refusal where the reader first meets it, and as a
  pointer at the end of the read-cost section's second refusal. **The remedy as written is not the
  one the finding proposed**, and the difference is load-bearing: `freeze_expectations.py` cannot
  be re-run on a converted tree at all. It exits if its output exists, and exits if any `SKILL.md`
  carries a `!` line, and all thirty do. So the sentence describes a new pre-registration landed
  with the migration by ruling and under the audit, and names the two guards rather than telling a
  maintainer to run a script that will refuse them. V3's F4 records the same script failing on the
  documented `--verify` path; the two findings are the same file.
- **The full run was blocked for a while and is not any more.** The first attempt exited 3 at the
  preflight's authentication rung with `terminal_reason: api_error` and zero tokens moved. The
  user's `sbx login` cleared it; the run above is the one that happened. I never attempted the
  login, which is the suite's own rule.
- **I ran the host cases before "P1 closed".** They are free, read-only, and P1's `doc` command,
  migration `0003` and views layout were already in the worktree, so the run was the cheapest way to
  find out whether my code was correct. They were re-run twice on signal afterwards, against the
  settled P1 tree and against P2's end-state tree, with the same tally each time.
- **`template` keeps its shape**, per your ruling: the case asserts its `# Title` first line and
  `schemas: replayed from <log dir>` last line, with the head/end pair required of `doc` alone. The
  wrapping follow-up is yours to book.
- **I did not sweep P2's `.md` bodies** for `plugins/mochiko/schemas/`. That is V2's audit. Say the
  word and it is a four-line host check.
- Nothing was committed.

## Files

Repo-relative, from the `cli-context` worktree root.

- `evals/contract/run.py` — the unit; frozen for V3's cold grade
- `evals/contract/README.md` — the unit; frozen, and §4 names what is stale in it
- `.mochiko/brainstorms/cli-schema-delivery/wave6-reports/p3-endstate-plan.md` — the approved plan
- `evals/.work/contract-*` — 82 evidence directories, one per case: staged plugin, streams,
  transcripts, sidechain transcripts, `verdict.json`, and `latency.json` on the delivery cases
- session scratchpad `full-run-2.txt` — the green full run, 1,692 lines, `EXIT=0`
- session scratchpad `host-run-p2closed.txt` — the 349-assertion host run on the end-state tree
- session scratchpad `full-run.txt` — the earlier skipped run, kept for the preflight reason
- session scratchpad `prove.py` — the bite proofs of §2
- session scratchpad `tally.py` / `tally2.py` — the recomputation of every §6 figure from the
  evidence directories
