# V3 — contract-suite audit (wave 5, `cli-schema-delivery`)

**Verdict: FAIL.** Nine of eleven items pass. The engineering is sound and I could not break it: criterion (1) is correctly keyed, it gates, I proved it red four ways, and every number in P3's report reproduces from the evidence to the unit. Two items fail — the README does not carry the wave-5 numbers and one of its tables no longer reproduces against the binary it names, and the freeze script that P3's earlier ordering argument rested on is not in the tree. Both are hours of work. Three further defects are in the fix list. Graded on the diff against `7d098b9`, the 81 sweep directories, the 30-case absence re-run, and my own runs — never the report alone.

**What I graded.** The unit moved under the audit: `run.py` was modified at 21:52 and again at 21:59:29, both after the 21:13–21:37 sweep; the README at 22:00:16; the report at 22:00:57; and P3 was still dispatching sessions at 22:00:48 while I worked. I pinned `run.py` at sha1 `ff78aaeea6972f4f2822f8844f7f44b7cd17965e` and graded that; it has been stable since. Binary: `mochiko-cli 0.1.0 · grammar 1..1`.

## 1. Criterion (1), re-keyed — PASS

`assert_floor_delivery` pairs a `### <id>` line with the next attribute line opening `[` and requires `class: floor` on it, the pairing `rendered_floor_ids` walks on the binary's own output. `floors_from_render` reads the `floors:` index line *and* walks the section renders, and their disagreement is its own `ok()` check; the case-time set is then compared to the freeze in both directions, either difference failing.

It gates. `ok()` yields `status == "fail"`, `run_cases` counts only that, and `main` returns `EXIT_ASSERT` solely on that counter. It runs on all 36 delivery cases and on the preload case's binary-present session.

I proved it red myself against a real `review-brainstorm` render. A transcript naming all nine ids in prose on a `FLOOR:` line and nowhere else fails all nine — the wave-4 bar would have scored that a clean pass. One `### <id>` heading removed fails naming that one id. `class: floor` downgraded fails all nine. An empty transcript fails all nine. The unmutated render passes.

## 2. Read-back demoted — PASS

Every read-back figure reaches the check list through `report()` alone; no `ok()` wraps `ids_exact` or `count_exact`. The two-line form is recorded per replicate with omitted ids named, and the count is graded against `floor_pin()`, parsed from the preamble's own `pins` block, never `len()` of the expectation. Exit-code path traced end to end: no score can reach it.

## 3. The skill family — PASS

Discovery is `converted_skills()`, the same `!`-line test the hook makes. Exactly 30 skills carry a `schema.yaml` and exactly those 30 are converted, so the unnarrowed no-Read assertion is safe. Delivery is `/mochiko:<skill> <argument>` ×3 at three turns; all 36 `PROBE_ARGUMENTS` rows carry a justification.

**The platform fact holds, confirmed on two skills.** Across all six delivery streams of `review-brainstorm` and `patterns-sound-loop` there are zero `tool_use` blocks of any kind, the string `"Skill"` appears in none of the six transcripts, and each transcript carries the seven blocks. The prompt-expansion path is what runs; the hook's Skill limb never fires there. The absence shape is asserted as measured — no model turn, install line in the session, no block, no version triple, no schema read — with the halting limb recorded.

**All three assertion calls were right.** Dropping `assert_halted` was correct for exactly the stated reason: it fails only on `CONTRACT-PROBE: delivered`, the fixture's marker, which a real skill never prints, so it would have passed unconditionally. Demoting the `Skill` tool-use assertion was correct, since it was keyed to a route that does not happen. Demoting the skill-noun presence line was correct then and restoring it after the hook fix is correct now — I proved the restored row bites by reverting the hook's skill fallback on a temp copy, where it exits 0 silently while the row demands exit 2.

## 4. The preload case — PASS, with an evidence defect

`devils-advocate` preloading `review-specifications`, two sessions. Present: the subagent's own transcript (`…/subagents/agent-a5be063738b7f8108.jsonl`, landed as `sidechain-2.jsonl`) carries all seven blocks and passes criterion (1) on all 8 floor ids — I ran the assert on that file alone. Absent: zero blocks in the union, no version triple, the spawn failing at preload with the parent surviving to report it, recorded then asserted.

**Defect V3-1.** `fetch_sidechain_transcripts` writes `sidechain-{n}.jsonl` with no state tag, while every other preload file carries `-present`/`-absent`. The absent run overwrote the present run's `sidechain-1.jsonl`, so the file under that name is the absent session's and a reader replaying from disk reconstructs neither union. Nothing was lost only because the absent run fetched one file and the block-bearing file was the second.

## 5. The freeze — FAIL on one limb of six

Five limbs pass, two of them against sources P3 does not control. The file holds 30 skills and 226 floor ids with per-skill family, floor set, baseline, source and pre-conversion body size. Its `body_bytes_pre` matches `git show HEAD:` for 30 of 30 skills; its `baseline_bytes` matches `schema.yaml` plus the named family common for 30 of 30. Mtime 20:49:47 precedes the earliest converted `SKILL.md` at 20:50:56 by 69 seconds and the sweep by 24 minutes, and no converted file predates it. `converted-shape` cross-checks all 36 pre-registered sets against the render in both directions and every `floors:` line against its section walk.

**The freeze script is not in the tree** — not under `scripts/`, not under `evals/contract/`, not untracked anywhere. Nothing in the repository carries the refusal rule, so I cannot read it, run it, or confirm it fired. The earlier report called that refusal the strongest of its three ordering proofs; the updated report drops the claim rather than supplying the file.

## 6. Numbers reproduce — PASS

I recomputed all 36 primitives from the transcripts through the suite's own helpers rather than reading `verdict.json`: criterion (1) per replicate, the read-back against the live pin, delivered bytes and chars, block count, floor count. **Every figure in the per-primitive table matches to the unit**, across four skill families and both command families, and all three replicates agree to the byte on all 36. Criterion (1) passes everywhere. Read-back is 107 of 108 on both lines; the miss is `specify` replicate 2, which named zero ids and opened by flagging the probe as injected text — a refusal, not partial recall.

| family | new B | old B | Δ B | new ch | old ch | Δ ch | F3 ch | vs F3 |
|---|---|---|---|---|---|---|---|---|
| review | 123,570 | 125,499 | −1.5 % | 121,548 | 124,610 | −2.5 % | 119,895 | +1.4 % |
| authoring | 154,143 | 156,070 | −1.2 % | 151,902 | 154,950 | −2.0 % | 150,576 | +0.9 % |
| patterns | 120,144 | 102,016 | +17.8 % | 118,273 | 101,178 | +16.9 % | 95,858 | +23.4 % |
| dense five | 92,531 | 85,143 | +8.7 % | 91,305 | 84,584 | +7.9 % | 81,799 | +11.6 % |

All four aggregates reproduce exactly in both units. My per-family baseline sums reproduce plan §0's raw-baseline column to the byte (98,278 · 117,014 · 72,549 · 53,989). Bytes and chars never share a column, in the code or the report. For the lead, not a defect: the record's F3 char figures sit 2 to 6 percent below the body-plus-schema figure recomputed from `HEAD`, so the F3 column and the pre-conversion column are not the same measurement, and the two deltas differ in sign for review and authoring.

## 7. The `tool_uses()` crash — PASS

Real bug, real fix. `message = event.get("message") or {}` then `message.get("content")` raises `AttributeError` on a row carrying `message` as a string; the guarded form skips non-dict messages and non-list content, and a string message carries no content blocks, so nothing previously reachable is now skipped. To rule out a quiet weakening I extracted every `ok()`, `report()` and `pending()` name literal from both revisions: three `ok` names disappear and all three reappear widened by a parameter, 17 are genuinely new, none was dropped.

## 8. Evidence, scope, GI-019 — PASS, with one open item

All 81 sweep directories are complete — three suffixed streams, transcripts, argv, script and stderr per delivery case, `latency.json`, the single-session equivalents elsewhere, the preload pair, and a staged plugin in every one. 151 session streams on disk in the window.

Scope holds on the suite's side: every write goes to a staged copy under `evals/.work/`, `stage()` copies out of `plugins/mochiko/` and never into it, and both stubs are written into the copy. No `Task(`, `Agent(` or `--agents` anywhere. The read-back is exact token-set equality against a frozen constant, not content grading.

**Open item, not P3's.** `plugins/mochiko/hooks/scripts/dependency-halt.sh` is modified in the tree — the skill fallback on `UserPromptExpansion`. P3 disclaims authorship and says V2 grades it. Authorship cannot be established from an uncommitted worktree; I record that the change is real, outside P3's declared unit, and owed its own author≠grader audit before the bump.

## 9. Independent re-run — PASS, not SKIPPED

`python3 evals/contract/run.py --host-only`: **exit 0**, 3/3 cases, **326 assertions, zero failures**, including 60 expansion-limb skill rows, 30 `PreToolUse` deny rows, 36 presence rows and 72 floor cross-checks. Check counts match the 21:51 evidence exactly (142 · 184 · 3).

There is still no case-selection flag, so I drove the case functions through the suite's own `preflight`, `build_binary`, `sandbox_path` and `run_cases`, editing nothing. `review-brainstorm-delivery` and `patterns-sound-loop-delivery`: **exit 0**, every assertion green on all three replicates, criterion (1) **9 of 9** and **6 of 6**, delivered bytes 10,172 and 9,761 — identical to P3's. Then `patterns-transport-floor-absence` against the pinned sha: **exit 0**, seven assertions green including the hook-block shape and the halt naming the skill.

## 10. README — FAIL

Present and true: the 81-case table and both totals, the skill family, the expansion-path fact with its history, the freeze and what it holds, the re-keyed criterion with its keying and the statement that it gates while the read-back does not, the preload case in both states, the two-unit warning, and `hook-input`'s move to real subjects.

**The wave-5 numbers are not there.** `## Measured figures` still says the skill and family figures "land here after the wave-5 full run" — the run has landed. Worse, the section stamps itself "binary 0.1.0 · grammar 1..1" and its seven-blocks-rendered column does not reproduce against that binary: I measure 19,193 · 11,209 · 17,714 · 36,398 · 16,872 · 19,917 against the README's 18,576 · 10,940 · 17,353 · 35,418 · 16,290 · 19,463. P1's `floors:` line moved every render and the version stamp cannot distinguish the two, because the binary version did not change.

Smaller: `## What is not here` says the suite "never dispatches an agent", which the preload case in the same file contradicts. The suite does not dispatch; the headless subject does, on the suite's prompt.

## 11. Report honesty — PASS, with two corrections

The headline reproduces exactly. Reconstructing the sweep from mtimes gives **81 cases, 967 `ok`, 284 `report`, zero `fail`, zero `pending`, 151 session streams**. The per-primitive table, the family table, the 107-of-108 read-back, `specify` replicate 2's refusal, the latency band (26–86 ms means, 159 ms worst, 183–531 ms whole fire), the preload subagent path and all nine deviations verify. The 30-case absence re-run verifies independently: 30 of 30 green, `num_turns: 0` in all 30, no `<local-command-stderr>` in any, and the halt naming `/mochiko:<skill>` in 30 of 30 read straight out of the streams. The post-hoc replay of that last assertion is disclosed rather than passed off as a live run, which is the right call.

Two corrections. "The six command rows are carried verbatim from `diagnostic.py`'s table" is false for `implement`: the argument is identical, the justification reworded. The README repeats it. And the currency caveat understates itself — it says the 30 skill cases recorded their hook observation against the pre-fix hook, but `run.py` also gained the restored presence assertion after the sweep, so those verdicts came from code lacking a check the graded code runs. My re-run closes that for two skills; 28 remain.

## Fix list

1. **README `## Measured figures`** — put the wave-5 run's command figures in, add the per-skill and per-family tables, and either restate the render column against the current binary or say plainly that it predates the `floors:` line.
2. **README `## What is not here`** — qualify "never dispatches an agent" against the preload case.
3. **Commit the freeze script** under `scripts/`, or drop every claim resting on its refusal rule and lean on the mtime plus the `HEAD` and render cross-checks, which are strong alone.
4. **State-tag the preload sidechains** (`sidechain-present-N.jsonl` / `sidechain-absent-N.jsonl`). One line; without it the case's evidence overwrites itself.
5. **Correct the `implement` verbatim claim** in the report and the README.
6. **Name the 28 skill delivery cases with no evidence under the graded code**, and put the re-run to the lead as a cost rather than leaving the caveat general.
7. **Freeze the unit before the next audit.** Three edits to `run.py` and one to the hook landed after the sweep, two of them after this audit opened.

## Advisories

- **V3-a.** `READ_BACK_BAR` and the per-replicate `read_back_passed` field are now unreferenced, as `pending()` and `assert_message` were at wave 4. Deviation 9 discloses none of them.
- **V3-b.** `case_preload` takes the section walk while `case_delivery` prefers the `floors:` line. The walk is the stronger read, but the preload case therefore never exercises the index line and never cross-checks the freeze.
- **V3-c.** Still no case-selection flag (wave-4 V3-f). Two audits have now driven the case functions by hand.
- **V3-d.** The seven-byte gap between render total and delivered figure (wave-4 V3-c) is now explained in the README. Carried forward as closed.

---

# Delta audit — the two fixes and the gate run — 2026-09-04

**Verdict: PASS.** Both items that failed the first audit are fixed, the gate run reproduces
exactly, and the report is honest. Graded on `evals/contract/run.py` at sha1
`ff78aaeea6972f4f2822f8844f7f44b7cd17965e` — unchanged since the first audit and, at mtime
21:59:29, ahead of the 22:00:05–22:07:52 gate window, so for the first time in this wave the
evidence is of the graded code. Three text corrections below; none blocks the bump.

## 1. README `## Measured figures` — PASS

Every figure reproduces against the binary the section names. I re-rendered all 36 primitives and
recomputed the family aggregates from the gate run's own transcripts rather than from the report.

The six-command table matches exactly on all five columns — baselines, the render totals under the
`floors:` line (19,193 · 11,209 · 17,714 · 36,398 · 16,872 · 19,917), every largest-render section
id and byte figure (`preamble` 2,604 · `arch.sec.boundaries` 4,761 · `feat.sec.tools` 5,354 ·
`impl.sec.tools` 15,617 · `setup.sec.tools` 5,289 · `spec.sec.tools` 5,965), the delivered figures,
and the deltas. The four family rows match to the byte in both units, and the per-family render
column (92,595 · 110,385 · 82,335 · 56,973) is the true render total, exactly seven bytes per
primitive above the delivered sums I computed in the first audit. The F3 comparison figures
(124.6k · 155.0k · 101.2k · 84.6k) are mine. The latency band — 26 to 86 ms, worst 159 ms, whole
fire 183 to 531 ms across the 36 — reproduces from the sweep's `latency.json` files, which is the
run the sentence attributes it to. The crate-version note is correct and is the right thing to say.

Three sentences are wrong where the numbers are right. **"The largest render anywhere is
`authoring-constitution.sec.artifact` at 9,938 bytes"** — that is the largest *skill* render;
`impl.sec.tools` at 15,617 bytes is larger and is named two paragraphs above, so "anywhere" makes
the file contradict itself. **The `floors:` line "adds between 7 and about 1,000 bytes"** — measured
across the shipped 36 the range is 98 bytes (`patterns-plan-minimalism`, 2 ids) to 979
(`implement`, 34 ids); the 7 looks borrowed from the trailing-newline gap explained three lines
later. **The F3 band "2 to 5 %"** is 2.9 % to 5.6 %, patterns being the outlier.

## 2. `freeze_expectations.py` — PASS

Both refusals are in the code and both fire. Against today's converted tree the script names all
thirty converted skills and exits 1; against a pre-existing output path it exits 1 on the file; in
neither case was anything written, which I checked rather than assumed.

`--verify` against a plugin root whose 38 `SKILL.md` files I restored to `7d098b9` returns
**byte-identical**, exit 0. I built that root myself: 30 files differed from the worktree and were
restored, 8 were already at that commit, none carries a `!` line, and only those 30 plus the hook
are modified under `plugins/mochiko/`, so P3's claim that schemas, the log and the labels registry
are unchanged holds. A fresh write against that clean root exits 0 and produces skill rows and
family blocks identical to the committed freeze, differing only in `frozen_utc` — the one field
that is a fact about the run rather than about the tree. My first audit's FAIL is discharged.

## 3. The gate run — PASS

**62 cases, 1,196 checks, zero failed**, reproduced from the evidence: 954 `ok`, 242 `report`, 60
session streams, `hook-input` and `converted-shape` on the host and 30 delivery plus 30 absence
cases in the sandbox. My own `--host-only` run on the pinned sha is exit 0 at 142 · 184 · 3 checks.

**The 30 absence cases carry the hook-block shape and only that.** From the raw streams: `num_turns:
0` in all 30, zero injected `<local-command-stderr>` in all 30, the hook's notice on the result
event in all 30, and the halt naming the skill in all 30. `assert_hook_block` accepts exactly that
conjunction, so a tolerant union is ruled out by construction.

I reproduced the discrimination replay and extended it. There are now **96** saved skill-absence
sessions on disk, not 66: the assertion **rejected all 32 pre-fix harness-shape sessions and
accepted all 64 hook-block ones, zero misclassified**. P3's 32-and-34 split is a true statement
about the smaller corpus that existed when they ran it; the pre-fix count matches exactly.

**The 30 delivery cases pass criterion (1) and the presence line under the graded code.** I re-ran
`assert_floor_delivery` myself over all 30 transcripts against each render's `floors:` line: zero
failures. The skill-noun line `mochiko-cli present · rules delivered by the skill's own render` is
in all 30. The frozen set equals the floors line in all 30. One replicate each, 16 gating checks
per case.

## 4. Report honesty — PASS

The headline, the 36-row table, the family table, the freeze section's four numbered claims, the
absence re-run section and the gate-run section all verify against my own recomputation. The
two-run comparison table is accurate and neither run's evidence overwrites the other — the 32
pre-fix absence directories and the sweep's delivery and preload directories are all still on disk.

One carry-over. The `implement` correction from the first audit was not applied: the README still
says the six command rows are carried verbatim from `diagnostic.py`, and that row's justification is
still reworded. Two other advisory fixes are also unapplied — the "never dispatches an agent"
sentence and the untagged preload sidechain filenames — both outside this delta's scope, both still
worth doing before the bump.

## Corrections for P3

1. "the largest render **anywhere**" → the largest skill render; `impl.sec.tools` is larger.
2. The `floors:` line adds **98 to about 1,000 bytes**, not 7.
3. The F3 band is **2 to 6 %**, not 2 to 5 %.
4. Still open from the first audit: the `implement` verbatim claim, the "never dispatches an agent"
   sentence, and state-tagging the preload sidechains.
