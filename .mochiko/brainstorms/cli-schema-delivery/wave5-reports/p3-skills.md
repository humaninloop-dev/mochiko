# P3 — wave 5 contract suite: the full run, green, with one finding that changed the plugin

**`contract suite: 81/81 cases passed, 81 ran, 284 measurement(s) recorded and not asserted` ·
exit 0.** Across the 81 verdicts on disk: **967 `ok`, 284 `report`, zero `fail`, zero `pending`.**
**Criterion (1) PASS on all thirty-six primitives** — every `class: floor` rule of every converted
command and skill arrived in every replicate, and all thirty-six `floors:` lines agree with their
section walks. 151 sandbox sessions, binary `mochiko-cli 0.1.0 · grammar 1..1`, plugin 0.105.0.
Unit: `evals/contract/run.py`, `evals/contract/README.md`, `evals/contract/expected-skills.json`.
Nothing under `plugins/mochiko/` was touched by me; nothing is committed.

**One caveat on the run's own currency, stated first because it changes what part of it means.**
The dependency hook gained a skill fallback *after* the sweep finished, in response to the gap in
§4. Every figure below stands, and criterion (1) is unaffected, but the thirty skill cases recorded
their hook observation against the pre-fix hook. §5 says what I did about that.

## 1. Criterion (1) and the read-back, per primitive

Criterion (1) is the gating assert: each `class: floor` id must appear in the transcript as a
`### <id>` heading whose next attribute line carries `class: floor`. The read-back beside it is
recorded and gates nothing. Delivered bytes are the blocks as they arrived, from replicate 1's
transcript; **all three replicates agreed to the byte for every one of the thirty-six**.

| primitive | floors | criterion (1) | count/ids | delivered B | baseline B | delta |
|---|---|---|---|---|---|---|
| `architecture` | 22 | PASS | 3/3 · 3/3 | 19,186 | 23,026 | -16.7% |
| `brainstorm` | 7 | PASS | 3/3 · 3/3 | 11,202 | 12,819 | -12.6% |
| `feature` | 13 | PASS | 3/3 · 3/3 | 17,707 | 21,020 | -15.8% |
| `implement` | 34 | PASS | 3/3 · 3/3 | 36,391 | 44,266 | -17.8% |
| `setup` | 18 | PASS | 3/3 · 3/3 | 16,865 | 20,245 | -16.7% |
| `specify` | 16 | PASS | 2/3 · 2/3 | 19,910 | 23,434 | -15.0% |
| `review-brainstorm` | 9 | PASS | 3/3 · 3/3 | 10,172 | 10,821 | -6.0% |
| `review-code-minimalism` | 3 | PASS | 3/3 · 3/3 | 7,095 | 7,155 | -0.8% |
| `review-feasibility` | 9 | PASS | 3/3 · 3/3 | 9,692 | 10,063 | -3.7% |
| `review-governance-intent` | 16 | PASS | 3/3 · 3/3 | 13,334 | 13,705 | -2.7% |
| `review-plan-artifacts` | 11 | PASS | 3/3 · 3/3 | 14,916 | 16,454 | -9.3% |
| `review-specifications` | 8 | PASS | 3/3 · 3/3 | 12,972 | 14,160 | -8.4% |
| `review-sufficiency` | 8 | PASS | 3/3 · 3/3 | 12,380 | 13,702 | -9.6% |
| `validation-constitution` | 14 | PASS | 3/3 · 3/3 | 11,978 | 12,218 | -2.0% |
| `authoring-architecture-store` | 9 | PASS | 3/3 · 3/3 | 14,581 | 15,361 | -5.1% |
| `authoring-constitution` | 12 | PASS | 3/3 · 3/3 | 22,243 | 24,566 | -9.5% |
| `authoring-epic` | 10 | PASS | 3/3 · 3/3 | 11,136 | 11,849 | -6.0% |
| `authoring-feature-map` | 16 | PASS | 3/3 · 3/3 | 16,629 | 17,514 | -5.1% |
| `authoring-prototype` | 4 | PASS | 3/3 · 3/3 | 10,552 | 10,946 | -3.6% |
| `authoring-requirements` | 4 | PASS | 3/3 · 3/3 | 9,099 | 9,380 | -3.0% |
| `authoring-technical-requirements` | 8 | PASS | 3/3 · 3/3 | 17,005 | 18,154 | -6.3% |
| `authoring-user-stories` | 4 | PASS | 3/3 · 3/3 | 9,084 | 9,244 | -1.7% |
| `patterns-adopt-first` | 7 | PASS | 3/3 · 3/3 | 11,377 | 10,739 | +5.9% |
| `patterns-architecture-shelves` | 5 | PASS | 3/3 · 3/3 | 10,668 | 9,513 | +12.1% |
| `patterns-code-minimalism` | 3 | PASS | 3/3 · 3/3 | 7,190 | 5,833 | +23.3% |
| `patterns-map-minimalism` | 3 | PASS | 3/3 · 3/3 | 8,114 | 6,951 | +16.7% |
| `patterns-model-tiering` | 4 | PASS | 3/3 · 3/3 | 8,355 | 7,232 | +15.5% |
| `patterns-plan-minimalism` | 2 | PASS | 3/3 · 3/3 | 7,124 | 5,835 | +22.1% |
| `patterns-sound-loop` | 6 | PASS | 3/3 · 3/3 | 9,761 | 8,996 | +8.5% |
| `patterns-transport-floor` | 11 | PASS | 3/3 · 3/3 | 9,623 | 8,200 | +17.4% |
| `patterns-vertical-tdd` | 5 | PASS | 3/3 · 3/3 | 10,060 | 9,250 | +8.8% |
| `analysis-codebase` | 3 | PASS | 3/3 · 3/3 | 9,251 | 8,345 | +10.9% |
| `brownfield-integration` | 6 | PASS | 3/3 · 3/3 | 8,172 | 6,728 | +21.5% |
| `executing-tdd-cycle` | 10 | PASS | 3/3 · 3/3 | 12,926 | 12,558 | +2.9% |
| `testing-end-user` | 7 | PASS | 3/3 · 3/3 | 12,336 | 12,163 | +1.4% |
| `testing-gap-finding` | 9 | PASS | 3/3 · 3/3 | 14,253 | 14,195 | +0.4% |

**Read-back: 107 of 108 replicates named the exact id set, and 107 of 108 named the exact count.**
The single miss is `specify` replicate 2, and it is not partial recall — that replicate named
**zero** ids, the refusal shape the wave-4 diagnostic identified, with all sixteen ids listed as
omitted. Every other replicate across thirty-six primitives, `implement`'s thirty-four ids
included, was exact on both lines. Delivery was untouched in that replicate: criterion (1) passed
on it, as it did everywhere.

## 2. Read cost, per family, in both units

Delivered-at-invoke is body plus render on each side: new is the converted `SKILL.md` plus the
delivered blocks, old is the pre-conversion body plus the schema-and-family-common baseline. Bytes
are the criterion; chars are carried beside them and the two never share a column.

| family | skills | converted B | pre-conversion B | delta B | converted ch | pre-conversion ch | delta ch |
|---|---|---|---|---|---|---|---|
| review | 8 | 123,570 | 125,499 | −1.5 % | 121,548 | 124,610 | −2.5 % |
| authoring | 8 | 154,143 | 156,070 | −1.2 % | 151,902 | 154,950 | −2.0 % |
| patterns | 9 | 120,144 | 102,016 | **+17.8 %** | 118,273 | 101,178 | **+16.9 %** |
| dense five | 5 | 92,531 | 85,143 | **+8.7 %** | 91,305 | 84,584 | **+7.9 %** |

Patterns and the dense five land above their baselines, which is what the wave open pre-stated —
they carry no family common file, so the render's fixed overhead has nothing to amortise against.
The two families that do have one come in below. The six commands all come in below on the
render-only comparison, in a band from −12.6 % (`brainstorm`) to −17.8 % (`implement`).

Latency, timed in the sandbox and load-dependent: per-section means of **26 to 86 ms**, a worst
single run of **159 ms**, and whole-fire figures of **183 to 531 ms** for all seven sections. The
spread is wider than wave 4's because thirty-six primitives were timed under varying load rather
than six. Every individual run is in each case's `latency.json`.

## 3. The preload case, both measured shapes

The second delivery channel, and the only one no `<skill>-delivery` case covers.

**Binary present.** The spawn succeeds, the subagent receives all seven blocks of
`review-specifications` and all eight of its floor rules, and it answers the probe — the parent
relays the subagent's reply verbatim as `FLOOR-COUNT: 8`. Two transcripts were fetched, and the
second is the subagent's own —
`…/75adb03a-…/subagents/agent-a5be063738b7f8108.jsonl` — which is where the blocks are. Criterion
(1) gates on that union.

**Binary absent.** The spawn fails at preload, fail-closed, and the parent survives to say so: it
reports that the agent's preamble invokes `mochiko-cli`, which the startup hook had already
flagged as absent, and that the shell errored immediately. Zero blocks in either transcript, no
schema read, no version triple.

## 4. The shape probes, and the gap they found in the plugin

Four probes ran before the sweep, per the approved plan. They cost four sessions and changed three
assertions, one of which was load-bearing.

1. **A `/mochiko:<skill>` prompt line takes the prompt-expansion path, not the `Skill` tool.** All
   seven blocks arrive in the expanded prompt exactly as a command's do, and **no `Skill` tool
   call happens at all** — confirmed across all thirty skills in the full run, whose recorded
   `invocation path` reads `prompt expansion` in every case.
2. **A natural-language dispatch does not reach the tool either.** That probe made three Bash
   calls and delivered nothing. Wave-0 probe (c) did see the tool fire for a probe skill, so the
   routing is prompt-shape dependent rather than fixed.
3. **Neither dependency-hook limb fired on that path — a converted skill ran ungated.** The hook
   resolved the name against `commands/<bare>.md`, which does not exist for a skill, and
   `PreToolUse`/`Skill` was never reached. All thirty skill delivery cases in the sweep recorded
   `dependency-hook limb that spoke — none`.
4. **It still failed closed, by the harness rather than the hook**: the `!` line exits non-zero,
   the expansion aborts, no model turn happens. The install line still reached the user, from the
   `SessionStart` hook.

Finding 3 is a fact about the plugin, not the suite, which is why it was reported rather than
absorbed into a weaker assertion.

## 5. The hook was fixed after the run, and what I did about it

`plugins/mochiko/hooks/scripts/dependency-halt.sh` now falls back to `skills/<bare>/SKILL.md` when
`UserPromptExpansion` resolves no command file. I verified it directly: fed a skill-named
expansion payload with the binary present, it returns
`mochiko-cli present · rules delivered by the skill's own render`.

That makes the thirty skill cases' recorded hook observation **stale — true of the hook during the
run, false of the hook now in the tree**. Criterion (1), delivery, read cost, read-back and
latency are all unaffected. What I did, all of it cheap:

- **Added sixty host rows** to `hook-input`: every converted skill down the prompt-expansion limb,
  absent (exit 2, install line, the deny naming `/mochiko:<skill>`) and present (the presence line,
  noun `skill`). The host layer is now **326 assertions, exit 0**. These are the rows that would
  have caught the gap.
- **Restored the presence-line assertion for skills** in the delivery cases. Before the fix it
  could only have been a recorded observation; now it is an assertion, and the noun is asserted, so
  a skill confirmed as a command fails.
- **Re-ran two cases end to end** against the fixed hook — four sessions, `review-brainstorm`
  delivery ×3 and absence. Delivery: `dependency-hook limb that spoke — skill`, presence assertion
  green, criterion (1) green on 9 ids, read-back 3/3 on both lines. Absence: green, and the
  recorded limb now reads `the dependency hook on UserPromptExpansion`.

**The absence assertion I had keyed to the pre-fix shape went red on that re-run, exactly as it
should.** The halt moved from the harness shape (the `!` line fails, stderr injected) to the
hook-block shape (the hook exits 2 before expansion, no `<local-command-stderr>`, the notice on the
result event). I re-keyed the case to assert what is true of both shapes and to record which limb
halted, which is what the command absence case has always done.

**Ruled (lead, 2026-09-04):** re-validate under the final code rather than accept the sample —
all thirty absence cases plus one delivery replicate per skill and both host cases, about sixty
sessions, with the ×3 read-back from this run standing as the recorded metric. That gate run is
reported at the end of this file.

## 6. Deviations from the approved plan

1. The freeze-ordering check compares against run start, not the earliest `.work` directory; the
   planned form reported a false negative because that directory accumulates across waves.
2. `command_expectations` renamed `primitive_expectations`, no alias. `diagnostic.py --list`
   re-verified after every edit and still exits 0.
3. `assert_halted` is not used in `<skill>-absence` — it reads the fixture's own marker string,
   which a real skill never prints, so it would have passed without looking at anything.
4. `hook-input` keeps its stub as a fallback rather than deleting it, so the case still runs on a
   tree with nothing converted.
5. The skill registration assertion searches the whole init event for `mochiko:<skill>` rather than
   one named field; the probes did not isolate the field.
6. `preload` runs at `--max-turns 6` and fetches sidechains by marker file. Neither was specified,
   and the marker approach is what found the `subagents/agent-*.jsonl` file. The fetched files are
   state-tagged `sidechain-present-N` / `sidechain-absent-N`; untagged, the absent half overwrote
   the present half's files (V3-1), which the fix closes for future runs.
7. Command delivery prompts moved to the two-line form, so wave-5 command read-back figures are not
   directly comparable with wave-4's. Delivery figures are unaffected.
8. Two planned assertions became recorded observations and one was strengthened, per §4; the
   presence line has since been restored as an assertion, per §5. I removed rows rather than leave
   them passing with nothing to check.
9. **A latent crash in the suite's own `tool_uses()`**, pre-existing since wave 3, was fixed: an
   event carrying `message` as a string raised `AttributeError`, and the natural-language probe
   produced exactly that shape. Every case calls that helper, including the no-Read assertion, so
   it would have taken down the sweep rather than failing a check.

## 7. Evidence, and the freeze's ordering

Eighty-one evidence directories under `evals/.work/contract-*`, each with its `verdict.json`,
stream, transcripts, argv, script and staged plugin; the delivery cases carry three suffixed
replicates and a `latency.json`; the preload case carries its sidechain transcripts.

The frozen expectations are at `evals/contract/expected-skills.json`, written by
**`evals/contract/freeze_expectations.py`** — in the tree, so the file is reproducible rather than
a hand-made artifact nobody can re-derive. Four things establish that it predates the wave it
grades, and the last is the strongest:

1. **Mtime.** Stamped `2026-09-04 20:49:47`; the earliest converted `SKILL.md` is
   `review-brainstorm` at `20:50:56`, with the rest of the review family at `20:53:38`–`20:53:39`.
   Sixty-nine seconds ahead of the first conversion landing.
2. **In the run itself.** `converted-shape` reports the freeze mtime against the run's own start:
   `frozen 20:49:47, run started 20:58:31 — ordered`, and again at `21:04:52`.
3. **Two refusals, both exercised now.** The script exits without writing if *any* `SKILL.md`
   carries a `!` line, and again if the output already exists. Against today's converted tree it
   names all thirty converted skills and exits 1; against a pre-conversion root with the freeze in
   place it exits 1 on the output file. Neither wrote a file. That it wrote at all in the first
   place means no skill was converted at that moment — which does not depend on timestamps.
4. **Byte-identical on rebuild.** `--verify` regenerates every derived field from a given plugin
   root, reusing only what the original run recorded about itself (the timestamp and the two
   version strings), and byte-compares. I materialised a plugin root whose thirty-eight `SKILL.md`
   files are restored to `7d098b9` — schemas, migration log and labels registry are unchanged
   between that commit and now, so only the bodies needed restoring — verified no skill in it
   carries a `!` line, and ran it: **byte-identical**. The committed freeze is exactly what that
   pre-conversion tree produces.

---

# Re-run — the thirty skill absence cases against the fixed hook (2026-09-04)

**Tally: 30 of 30 green, zero failed checks; `hook-input` 142 checks, zero failed.** Thirty
sandbox sessions plus the host case. The plugin was not touched — `dependency-halt.sh` arrived
fixed and V2 grades it.

**The measured shape, identical in all thirty:** `num_turns: 0`, **no `<local-command-stderr>`
injected**, and the harness's own notice on the result event —

```
UserPromptExpansion operation blocked by hook:
[…/mochiko/hooks/scripts/dependency-halt.sh]: mochiko-cli is not installed —
/mochiko:<skill> cannot run without it. Install: cargo install mochiko-cli
```

That is the wave-3 command absence shape exactly, and the recorded `which limb halted first` reads
`the dependency hook on UserPromptExpansion` in **30 of 30** — against `none` in all thirty of the
pre-fix run. The install line reaches the session through `session-transcript, stream-events`.

**Keying.** One probe measured the shape first (`review-brainstorm-absence`), and the case was
re-keyed to it before the family ran: the assertion I had pinned to the injected stderr went red on
that probe, because the halt had moved from the harness to the hook, and it was replaced with what
holds under both shapes. The pre-fix evidence stays on disk as the earlier record; the thirty
directories from this re-run supersede it only for the absence cases.

**One assertion was added after these sessions were dispatched and evaluated post hoc.** The lead's
brief called for the halt to name the skill, which the pre-fix path could not do. Rather than claim
it or spend thirty more sessions, I replayed it over the evidence on disk through the suite's own
helpers, reading the same two channels the live assertion reads: **the halt names `/mochiko:<skill>`
in 30 of 30.** It is now a live assertion in `case_skill_absence` and will run as one from the next
sweep.

**`hook-input` at 142 checks** now carries, per converted skill, a deny row and a presence row down
the prompt-expansion limb as well as the `PreToolUse` limb — sixty rows that did not exist when the
gap was found, and the ones that would have caught it.

**Unchanged by this re-run:** criterion (1) on all thirty-six, the read-back, per-family read cost,
latency, and the preload case. Those figures come from the full run above and the hook fix does not
touch them.

---

# Gate run — under the final code (2026-09-04)

**Tally: 62 cases, 1,196 checks, zero failed.** The run the bump is graded on, per the lead's
ruling: `hook-input` and `converted-shape` on the host, then one delivery replicate and the
absence case for each of the thirty converted skills — sixty sandbox sessions. Exit 0.

| case group | cases | result |
|---|---|---|
| `hook-input` (host) | 1 | 142 checks, 0 failed |
| `converted-shape` (host) | 1 | 184 checks, 0 failed |
| `<skill>-delivery` ×1 replicate | 30 | 0 failed; **criterion (1) PASS 30/30**; presence line PASS 30/30 |
| `<skill>-absence` | 30 | 0 failed; hook-block shape 30/30 |

**Criterion (1) is re-asserted post-fix on all thirty skills** and passes on every one. The
presence-line assertion — restored and gating, and only assertable because the hook now resolves
the skill on the expansion path — passes on all thirty.

**The absence shape is deterministic and identical in all thirty:** `num_turns: 0`, **no
`<local-command-stderr>` injected**, and the hook's own notice on the result event. Per the ruling
the case asserts that shape *only*; the harness halt is the pre-fix record on disk, not a second
accepted outcome.

I proved the new assertion discriminates rather than asserting that it does, using the two shapes
already on disk: across **sixty-six saved sessions** it rejected all **32** pre-fix harness-shape
sessions and accepted all **34** hook-block sessions. A tolerant union would have passed both sets.

**What stands from the pre-fix full run**, per the ruling, because the hook touches neither
delivery nor read-back: the ×3 read-back metric (107 of 108 replicates exact on both lines), the
per-family read cost, the latency band, and the preload case. Those figures are in the first
section of this report and are not superseded by this run.

## Two runs, distinctly

| | pre-fix full run | post-fix gate run |
|---|---|---|
| scope | all 81 cases, 151 sessions | 62 cases, 60 sessions |
| result | 81/81, 967 `ok`, 284 recorded, 0 failed | 62/62, 1,196 checks, 0 failed |
| criterion (1) | PASS on all 36 primitives | re-asserted PASS on the 30 skills |
| read-back | ×3 per primitive — 107/108 exact | ×1 per skill, recorded |
| skill absence shape | harness halt, no limb gated | hook block, 30/30 |
| skill presence line | recorded (no limb spoke) | asserted, PASS 30/30 |

Both runs' evidence is on disk and neither overwrites the other: the gate run's directories are
newer, and the pre-fix absence directories remain as the record of the shape before the fix.
