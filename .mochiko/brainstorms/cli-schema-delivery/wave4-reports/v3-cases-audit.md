# V3 — contract-suite audit (wave 4, `cli-schema-delivery`)

**Verdict: PASS.** All eleven items pass. Graded on the diff (`git diff -- evals/contract/run.py evals/contract/README.md`), the twenty newest `evals/.work/contract-*` directories, and my own runs — never P3's report alone. Every figure in the abort-criteria table reproduces exactly from the evidence, the three tripped read-back scores included, and every id that went unnamed is present verbatim with its `class: floor` line in the transcript the model read. My own re-run of the three host cases plus `implement-delivery` and `setup-absence` is exit 0. Binary under test: `mochiko-cli 0.1.0 · grammar 1..1`. Eight advisories at the end; none blocks the landing. `evals/contract/diagnostic.py` exists in the tree and is out of scope here.

## 1. Generalization keeps the wave-3 keying — PASS

`assert_delivery` and `_aggregate` are the wave-3 functions with `PILOT_COMMAND` replaced by a `command` parameter and nothing else touched; I diffed both against `git show HEAD:`. `case_command_absence` carries the same four assertions and two `report` lines as `case_brainstorm_absence`, adding only `"command": command` to the verdict payload. To rule out a quiet weakening I extracted every `ok(...)` name literal from both revisions and compared the sets: nothing was dropped, four names differ only because a parameter widened them, two are genuinely new (deviation 2's guards).

Positive assertions read `asserted_output` — stream events plus session transcript — or the transcript alone; negatives keep the wide union. I traced every call site: the install line and `grammar 99` through `assert_in_session`, the SessionStart line through `_session_start_line`, the blocks and the hook presence line through the transcript, the three `assert_no_version_triple` calls through `session_output_with`. No tolerant leftover. `assert_no_schema_read` still routes both limbs through `is_schema_path`, so wave 3's V3-b fix survives.

## 2. Discovery from the `!` lines — PASS

`converted_commands()` calls `converted_primitives()`, which reads each `plugins/mochiko/commands/*.md` and keeps those containing `` !`mochiko-cli rules ``. `SANDBOX_CASES = build_sandbox_cases()` is built from it at import and `--list` prints twenty. All six shipped commands carry seven `!` lines, so a command without one would not appear. The `hook-input` absent and present rows both iterate `converted_commands(staged.plugin)`, six rows each. The unconverted-command subject is a stub written to `staged.plugin/commands/contract-unconverted.md` after the discovery call; `plugins/mochiko/` holds no such file and no `contract-stub` skill.

## 3. Pre-registered constants — PASS

Scripted, not read. Parsing plan §4's five bullets out of the plan text and comparing to `EXPECTED` gives set equality on all five (22 · 13 · 34 · 18 · 16), each bullet's stated count agreeing with its own list, and the five `baseline_bytes` equal to plan §0's raw-baseline column to the byte. `brainstorm` is one ordinary row at 7 ids and 12,819 bytes; `FLOOR_IDS` is now an alias for it.

The cross-check bites. Running `case_converted_shape` in memory against mutated tables: renaming `spec.gate-selection` to `spec.gate-selection-TYPO` turns the specify row red naming both directions of the difference, and deleting the `feature` row turns the unregistered-command check red. Unmutated: 26 checks, zero failures.

## 4. Bars never gate — PASS

The read-back score reaches the check list only through `report()`, as do the delivered read cost, the latency band and every `brainstorm-policy` observation. `run_cases` increments `failures` only on `status == "fail"`, and `main` returns `EXIT_ASSERT` solely on that counter. Not merely traced: my own `implement-delivery` run scored 0/3, printed `ABORT CRITERION: read-back`, and exited 0. Across the twenty newest verdicts the tally is 158 `ok`, 41 `report`, zero `fail`, zero `pending` — matching P3's claim exactly.

## 5. Abort-criteria numbers reproduce — PASS

I recomputed both columns from the evidence rather than from `verdict.json`: each `transcript-N.jsonl` through the suite's own `transcript_plaintext` and `delivered_blocks` for bytes, and the `FLOOR:` line out of each `stream-N.jsonl` scored against the expectation set myself.

| command | read-back mine / P3 | delivered mine / P3 |
|---|---|---|
| `architecture` | 3/3 · 3/3 | 18,569 · 18,569 |
| `brainstorm` | 3/3 · 3/3 | 10,933 · 10,933 |
| `feature` | 3/3 · 3/3 | 17,346 · 17,346 |
| `implement` | 1/3 · 1/3 | 35,411 · 35,411 |
| `setup` | 1/3 · 1/3 | 16,283 · 16,283 |
| `specify` | 0/3 · 0/3 | 19,456 · 19,456 |

All three replicates agree to the byte in every command, as claimed. The delivered figure is the blocks alone, which is the README's stated definition and the wave-3 README's own convention.

Delivery held on every miss. Each unnamed id appears exactly once as a `### <id>` heading in the corresponding transcript with `class: floor` on the next line: `impl.graded-fold` in implement replicate 3; `setup.acceptance-plain-text`, `setup.author-grader-default-fail`, `setup.no-git-mutations` and `setup.blind-map-dispatch` in setup replicates 1 and 3; `spec.author-grader-default-fail` in all three specify replicates. P3's positional claims check out against the render — that specify id is sixth of eight rules in `spec.sec.ways-of-working`, and `impl.graded-fold` is 42nd of 44 in `impl.sec.tools`.

## 6. `implement` replicate 2 — PASS, and it refused rather than failed

Plainly: the model refused the probe argument at the Entry gate. Its reply opens "I need to flag something before going further: the text passed as the command's argument … isn't a valid feature/epic identifier", declines to treat an injected `$ARGUMENTS` value as a directive that would skip the capability lookup and the sufficiency check, and asks which `FEAT-XXX` or `EPIC-XXX` was meant. It ran one turn and produced text. Delivery was untouched: that replicate passed all ten assertions and its block bytes are identical to the other two. Scoring rather than discarding it is right under the wave-3 replicate rule, and deviation 7 discloses it.

## 7. Evidence completeness — PASS

I enumerated the newest verdict per case and checked the file set. All twenty present, each with its staged plugin. The six delivery directories hold `stream-{1,2,3}.jsonl`, `transcript-{1,2,3}.jsonl`, `argv-N.txt`, `script-N.sh`, `stderr-N.txt`, `latency.json`, `verdict.json`; the seven single-session cases hold the unsuffixed equivalents plus a transcript where the case fetches one; the three host cases their verdict and staged copy. Nothing missing. Verdicts were written 16:19:47 to 16:23:31, postdating `run.py`'s 16:10:29 mtime, so the run is of the shipped code.

## 8. Scope — PASS

P3's unit under `evals/contract/` is `run.py` and `README.md` and nothing else, with no new or modified fixture file. Every perturbation acts on a staged copy: `stage()` copies out of `plugins/mochiko/` into `evals/.work/`, `swap_plugin_log` acts on that copy, both stubs are written into it. The only file under `plugins/mochiko/` touched since the run is P1's `migrations/README.md`, documentation no render reads — and my re-run reproduced the delivered bytes exactly, confirming the rendering surface is unchanged. The suite dispatches no agent (no `Task(`, `Agent(`, `subagent` or `--agents` in the file) and grades no content: the read-back metric is exact token-set equality against a pre-registered constant and never gates.

## 9. Independent re-run — PASS, not SKIPPED

`python3 evals/contract/run.py --host-only`: **exit 0**, `3/3 cases passed, 3 ran, 2 measurements`. All 23 `hook-input` assertions green including the six per-command absent and six present rows; all 26 `converted-shape` checks green including the six floor-set cross-checks; `render-ceiling` green at `implement · impl.sec.tools`, 15,450 chars, 51.5 %.

There is no case-selection flag, so for the two sandbox cases I drove `M.SANDBOX_CASES` through the suite's own `preflight`, `build_binary`, `sandbox_path` and `run_cases`, editing nothing. `implement-delivery` and `setup-absence`: **exit 0**, 2/2 cases, 5 measurements, every delivery assertion green on all three replicates. My `implement` read-back is **0/3** against P3's 1/3 — two replicates named 33 of 34, omitting `impl.graded-fold`, the same id P3's failing replicate dropped, and the third named all 34 plus one invented `impl.seat-sufficiency-independence`. Delivered: 35,411 bytes, identical to P3's. That strengthens P3's reading rather than weakening it: the miss is recall, it is reproducible on the same id, and delivery is sound in both runs.

## 10. README — PASS

The twenty-case table, the per-command family, the discovery rule, the `op-unknown` failure mode with `cargo build --release -p mochiko-cli` as the fix, the 3-of-3 bar with per-command id counts, and the per-command summary block naming a tripped criterion are all present. I checked the figures rather than trusting them: the seven-block render totals (18,576 · 10,940 · 17,353 · 35,418 · 16,290 · 19,463), the post-`0002` baselines (each exactly seven bytes above the pre-registered figure) and the largest render per command all reproduce against the host binary to the byte. The policy claim of five observations, three surfacing the not-delivered line and two replying `FLOOR: none`, matches the five `brainstorm-policy` verdicts on disk.

## 11. Report honesty — PASS

All seven deviations verify. The stale-binary rebuild is build output and the README now names `op-unknown`. The two `converted-shape` additions are in the diff and I proved both red by mutation. The replicate-disagreement clause exists and never fired. The relabel is real: HEAD reads `capture provenance` and `capture_notes`, the new code `row provenance` and `provenance_notes`. The column widened from `{name:20s}` to `{name:22s}`. `pending()` is defined and unreferenced. Replicate 2 was scored. Of the fifteen red probes I reproduced nine independently — exact set bare, backticked and bolded pass; a dropped id, an extra id, quoted ids and a missing `FLOOR:` line fail; `brainstorm`'s set scored as `implement` fails while `implement`'s own passes — plus both cross-check mutations. Nothing overstates the evidence, and the closing note correctly declines to rule on the trip.

## Advisories

- **V3-a.** The report is at `p3-cases.md`; its own approved plan §5 names `p3-suite.md`. Harmless, and the lead's brief uses the new name, but it is an undisclosed departure from the plan.
- **V3-b.** Wave 3's audit quoted the delivered figure as 10,839 — blocks plus the 80-byte SessionStart line and the 66-byte hook presence line. Wave 4's tables quote blocks only, which is the wave-3 README's own convention (10,693) and is stated in the new README. A reader comparing 10,839 to 10,933 compares two conventions; worth a line in the record.
- **V3-c.** The delivered figure sits exactly seven bytes below the render total for every command, because `delivered_blocks` captures head line through end line and drops each block's trailing newline. Benign and consistent, but the README's two tables invite the question and never answer it.
- **V3-d.** `assert_message` is defined and unreferenced, as `pending()` is; deviation 6 discloses only the latter.
- **V3-e.** `channels_of` still has a `process-stderr` channel and no `process-stdout`, so a stdout-only fragment yields an empty parenthetical in the failure message. Carried from wave 3, still cosmetic.
- **V3-f.** No case-selection flag. An independent partial re-run has to drive the case functions directly, as I did. A `--case` argument would make the next audit cheaper for a few lines.
- **V3-g.** The evidence table notes the `-1`/`-2`/`-3` suffix on the `stream.jsonl` row only; the transcripts are suffixed too.
- **For the lead, not a defect.** My re-run makes `implement` 1 of 6 replicates across two independent runs, with `impl.graded-fold` — 42nd of 44 rules in the largest block in the suite — omitted in three of the four failures. That is a positional recall pattern, not a delivery one, and it is exactly what the pre-registered diagnostic in plan §8 is designed to separate.

Nothing blocks the landing from the suite side.

---

# Delta audit — the read-back diagnostic — 2026-09-04

**Verdict: PASS.** All seven delta items pass. `git diff -- evals/contract/run.py evals/contract/README.md` is empty against both `9cdac97` and the worktree, so the audited suite is untouched; `diagnostic.py` and `p3-diagnostic.md` are the only untracked files in the tree. I recomputed all eighteen replicates from the evidence and every figure matches. Two advisories on the report's wording and two on the runner; none changes a conclusion.

## 1. Design against §8 as amended — PASS

Three replicates, `--max-turns 2`, six commands discovered through `converted_commands()`, and sonnet on every session — the argv files carry `--model sonnet --max-turns 2` verbatim, and `run_probe` hardcodes the model rather than taking it from the caller. The prompt is `/mochiko:<cmd> <token>` followed by the two-line instruction, which is the amendment's shape: a gate-valid token at the front of the argument, the instruction after it. `INSTRUCTION` asks for `FLOOR-COUNT:` first and `FLOOR:` second.

I checked every argument against its command's own Entry step rather than against the runner's note. All five free-text commands accept `caching` — `architecture` and `feature` surface health before taking the request, `brainstorm` and `specify` send back only an empty argument, `setup` says empty is fine. The `implement` justification is verbatim right: its Entry gates on "a spec's accepted selection … or a desk-confirmed delta card", and "Neither → route: … a feature-keyed delta to `/mochiko:feature`", so a delta-card path that does not exist takes the routing branch instead of validating a feature id.

Nothing gates. `main()` returns only 0, 2 on an unknown or unregistered command, and 3 on an unavailable sandbox. No score reaches an exit code, and there is no assertion path at all.

## 2. Scoring recomputed for all eighteen replicates — PASS

I re-derived each replicate's four scores from `stream-N.jsonl` and `transcript-N.jsonl` using the runner's own `score()` and `floor_pin()`, and compared to both the run summary and the six per-command files. Every figure matches: `brainstorm` 3/3/3 · `architecture` 2/2/2 · `feature` 2/2/2 · `implement` 3/3/3 · `setup` 3/3/3 · `specify` 3/3/3, on count, ids and superset alike. Zero mismatches across 18 replicates and 72 scored values.

The pin is honest. `floor_pin()` parses `- class: floor · N rules` out of the preamble's own `pins` block rather than taking `len(EXPECTED[...])`, so the two sides of the count comparison cannot agree by construction. The rendered pins are 7 · 22 · 13 · 34 · 18 · 16 and each equals its pre-registered set size, which is the same cross-check `converted-shape` runs. The ids leg calls `suite.score_read_back`, the identical function I probed nine ways in the main audit.

## 3. The two non-3/3 replicates — PASS

Both are the probe colliding with the command, and both had complete delivery.

`architecture` replicate 2 refused outright. Its words: the embedded block instructs it "to short-circuit the desk protocol and just dump floor-rule counts/IDs before doing anything else", which "reads as an injected instruction", and "I'm flagging it rather than complying with it." One turn, one text block, no read-back line.

`feature` replicate 3 also flagged it first — its opening block reads "I want to flag something before proceeding: the command args you passed contain two parts" — then ran a Bash call and used its second turn to execute the desk protocol, reporting map health and asking its converging questions. It cites `feat.growth-routes-to-specify` by id in that reply, which is direct evidence the rules arrived and were read.

Both delivered all seven blocks. Both named **zero** ids rather than a partial set, so no replicate anywhere in this run produced partial recall. I confirmed independently that every expected id appears as a `### <id>` heading with `class: floor` on the next line in the corresponding transcript — 22 of 22 for `architecture`, 13 of 13 for `feature`. Every one of the other sixteen replicates named the exact set with the exact count.

## 4. The negative control — PASS, and it reads as claimed

`diagnostic-implement-fbd2145f` is the `before` shape. Its transcript carries no `mochiko-cli rules implement · section` head line, no dependency-hook presence line, and zero delivered blocks — the command never expanded. The model answered `FLOOR-COUNT: 0` and `FLOOR: (none)`, naming no id of a command it demonstrably knows. That is the control the whole diagnostic rests on, and it holds.

## 5. `implement`'s gate-valid argument — PASS

All three replicates answered on turn 1 with `FLOOR-COUNT: 34` and all thirty-four ids exactly, no refusal, and the `after`-shape probe `diagnostic-implement-58ced4d8` is a fourth clean run. §8's criterion — a harness artifact "only if the gate-valid argument removes it 3/3" — is met on its own terms. P3's qualification is the right one and I confirm it: the refusal did not vanish, it moved to one `architecture` replicate, so it belongs to the probe shape rather than to `implement`.

## 6. Evidence completeness — PASS

Each of the six command directories holds `stream-{1,2,3}.jsonl`, `transcript-{1,2,3}.jsonl`, `argv-N.txt`, `script-N.sh`, `stderr-N.txt`, its `diagnostic.json` and its staged plugin. Nothing missing. The two probe directories hold the single-replicate equivalents, and `diagnostic-summary-84657291/diagnostic.json` carries all six command summaries with every per-replicate row. The staged plugin is byte-identical to the committed wave-4 state, which I checked by diffing a staged `implement.md` against `HEAD`, so "against the landed wave-4 state" is accurate even though the sessions ran shortly before the commit.

## 7. Report honesty — PASS

Every claim in `p3-diagnostic.md` verifies: sixteen of eighteen exact, the six-row table, the wave-4 comparison column, the "all N" cells backed by transcript presence, the four-run `implement` result, the negative control, and the four deviations. The scope claims hold — `run.py` and `README.md` are untouched with modified times predating `diagnostic.py`, nothing under `plugins/mochiko/` changed, and the runner writes only under `evals/.work/diagnostic-*`. The recommendation correctly declines to rule and hands the §8 branch question to the lead with its reasoning shown.

## Advisories

- **V3-h.** The headline calls both failures "refusals" while the table calls `feature` replicate 3 "no read-back line" and the §8 section calls it "one no-answer". The headline is the more accurate of the two: that replicate did flag the instruction before proceeding. The body's account of it — "ran its Entry step as written … and never emitted the two lines" — omits the flag and understates P3's own case.
- **V3-i.** Deviation 1 says the instruction is substituted "six or seven times per run". The recorded range across the eighteen is five to seven: `implement` is 7 in all three, and two of the other replicates are 5 rather than 6. The `$ARGUMENTS`-site explanation covers the 7-versus-6 split and not the 5.
- **V3-j.** `score()` reads only the final assistant text. I checked every replicate: sixteen produced a single text block, `feature` replicate 3 produced two, and no non-final block carries a read-back line the final one lacks. Nothing was lost here, but a replicate that answered and then continued would be scored as a miss.
- **V3-k.** The summary's `"model": "sonnet"` is a hardcoded literal, not read back from the argv it claims to describe. It is correct — `run_probe` hardcodes sonnet and all twenty argv files show `--model sonnet` — but it is a self-report rather than a measurement, unlike every other figure in the file.

## Reading

The diagnostic separates the three things cleanly, and only the third was ever in play. **Delivery** was never in question and is now doubly confirmed: all seven blocks arrived in every one of the eighteen sessions, the failing replicates included, and every id a model did not name is present verbatim with its `class: floor` line in the transcript it read. **Recall** is sound at every floor count the suite carries — sixteen of eighteen replicates named the exact set and the exact count, `implement`'s thirty-four ids four times out of four, and the two misses came in at 22 and 13 ids rather than at the top of the range, so nothing scales with count. Decisively, neither miss was partial: both named zero ids, and an attention or enumeration ceiling produces short lists, not empty ones. What remains is **probe compliance** — an instruction that rides inside `$ARGUMENTS` is indistinguishable from injected text, and a model following its command's own protocol will sometimes refuse it, which is the behavior the primitives are written to produce. The wave-4 trip was a property of the bar, not of the render: `implement`, `setup` and `specify` all come back 3/3 on both legs. I read §8's "counts below 3/3" branch as not firing, for the reason P3 gives — its antecedent is an attention problem at a floor count and the evidence contradicts that mechanism — and I agree the wave-5 question is probe design, not render design. The ruling is the lead's.
