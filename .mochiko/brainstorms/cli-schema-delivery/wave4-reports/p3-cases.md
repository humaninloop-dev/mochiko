# P3 — contract suite (wave 4)

**Full run complete: `python3 evals/contract/run.py` exits 0, 20 of 20 cases passed, 158
assertions green, 41 measurements recorded, none pending, none failed.** Twenty-nine sessions in
the sandbox `claude-mochiko`, against the frozen state: log `sequences 1..2`, state hash
`8972891099f7…43fd`, the nine-line preamble legend, all six commands re-pointed.

**Abort criterion (2) is clear on every command by 15 to 20 %. Abort criterion (1) trips on
three: `implement` 1/3, `setup` 1/3, `specify` 0/3.** The evidence below says those are read-back
misses and not delivery failures, but the bar was pre-registered and a miss is a trip. The ruling
is the lead's.

Unit: `evals/contract/run.py` and `evals/contract/README.md`. Nothing else was touched; nothing
was committed. Plan: `wave4-reports/p3-cases-plan.md`, approved 2026-09-04.

## The two abort criteria, per command

| command | read-back | criterion (1) | delivered | baseline, pre-registered | delta | criterion (2) |
|---|---|---|---|---|---|---|
| `architecture` | 3/3 of 22 ids | clear | 18,569 | 23,026 | −19.4 % | clear |
| `brainstorm` | 3/3 of 7 ids | clear | 10,933 | 12,819 | −14.7 % | clear |
| `feature` | 3/3 of 13 ids | clear | 17,346 | 21,020 | −17.5 % | clear |
| `implement` | 1/3 of 34 ids | **TRIPPED** | 35,411 | 44,266 | −20.0 % | clear |
| `setup` | 1/3 of 18 ids | **TRIPPED** | 16,283 | 20,245 | −19.6 % | clear |
| `specify` | 0/3 of 16 ids | **TRIPPED** | 19,456 | 23,434 | −17.0 % | clear |

All three replicates agreed to the byte on the delivered figure for every command. The
post-`0002` baselines are seven bytes higher in every row, so criterion (2) reads the same
against either column and is nowhere near its boundary.

## What the failing replicates actually did

Every miss is a near miss, and in no case did a rule fail to arrive.

- **`specify`, 0/3.** All three replicates named 15 of 16 and omitted the same id every time,
  `spec.author-grader-default-fail`. It renders sixth of eight rules in
  `spec.sec.ways-of-working`, carries `[class: floor · labels: independence]`, and is present in
  every transcript. A consistent single-id omission across three independent sessions is the most
  diagnostic result in the run.
- **`implement`, 1/3.** Replicate 1 named all 34 exactly. Replicate 3 named 33, omitting
  `impl.graded-fold`, which renders 42nd of 44 rules in `impl.sec.tools` and is present in the
  transcript. Replicate 2 produced no `FLOOR:` line at all: the model treated the probe text
  passed as the command's argument as suspicious and said so instead of answering.
- **`setup`, 1/3.** Replicate 2 named all 18. Replicates 1 and 3 named 15, omitting
  `setup.acceptance-plain-text`, `setup.author-grader-default-fail` and `setup.no-git-mutations`;
  replicate 3 also emitted a self-correcting sentence inside the comma-separated list, which the
  scorer counted as one malformed token, correctly.

**Delivery itself is sound on all six.** Every delivery case passed all ten assertions on all
three replicates: seven head lines, seven end lines, every end-line count matching its preamble
pin, no `!` line passed through literally, no Bash denial, no schema file read, the SessionStart
line, the dependency hook's presence line, and all six commands registered as slash commands. The
ids that went unnamed are literally in the transcripts.

## Latency, per command

Timed inside the sandbox, load-dependent, ten runs per section; every individual run is in each
case's `latency.json`.

| command | per-section mean | worst single run | whole fire, seven sections |
|---|---|---|---|
| `architecture` | 27–28 ms | 29 ms | 220 ms |
| `brainstorm` | 27–28 ms | 29 ms | 181 ms |
| `feature` | 28–33 ms | 67 ms | 187 ms |
| `implement` | 27–29 ms | 35 ms | 185 ms |
| `setup` | 26–27 ms | 29 ms | 181 ms |
| `specify` | 27–29 ms | 39 ms | 196 ms |

## The other cases

- **`hook-input`**: 23 assertions green. Six per-command absent rows and six present rows, the
  skill deny and presence rows, the skew row, the foreign-namespace row and the three
  `SessionStart` rows. One recorded provenance note, for the staged unconverted-command stub.
- **`converted-shape`**: 26 assertions green. All 35 `!` lines render their own primitive, in the
  order each render declares, with the Bash grant present; every converted command has a
  pre-registered row; all six floor sets match the render exactly.
- **`render-ceiling`**: green. Largest render `implement · impl.sec.tools` at 15,450 chars, 51.5 %
  of the ceiling. `implement`'s whole render is 35,418 bytes, above the ceiling, which is why
  delivery is chunked per section.
- **The two fixture cases and the four mechanism cases**: all green, each in its wave-3 shape.
- **`brainstorm-policy`**, recorded and never asserted: this run makes **five observations, three
  of which surfaced the not-delivered line**; the other two replied `FLOOR: none`. No run in the
  five ever read a schema file as a fallback, so the posture has held every time. The README
  count is updated.

## Deviations from the approved plan, with reasons

1. **A stale host binary had to be rebuilt twice.** The first host run failed everything with
   `op-unknown`, because the binary predated migration `0002`'s `reword-section` op. A later check
   read a six-line legend because it raced P1's write of the nine-line version. Both were fixed by
   `cargo build --release -p mochiko-cli`, which is build output rather than a source edit, and the
   README now names this failure mode because the message reads like a suite bug and is not one.
2. **`converted-shape` gained a check the plan did not name:** every converted command has an
   `EXPECTED` row. Without it, a command converted later with no pre-registered bar would get a
   delivery case that measures nothing. The delivery case carries the matching guard so a missing
   row fails one case instead of raising through the run.
3. **The delivered-cost line names a replicate disagreement** when the three replicates differ.
   They did not differ anywhere in this run, so the clause never fired; it is a `report` and
   changes no exit code.
4. **The `hook-input` provenance note was relabelled** from `capture provenance` to `row
   provenance`, and its `verdict.json` key from `capture_notes` to `provenance_notes`, because it
   now carries staged-stub notes as well as synthesized-capture ones.
5. **The case-list column widened from 20 to 22 characters** so `architecture-delivery` does not
   push its description out of alignment. Cosmetic.
6. **`pending()` is still defined and unreferenced**, as at wave 3 and for the same reason: it is
   the helper a later-wave assertion would use, and the summary path still counts and prints
   pendings separately.
7. **Replicate 2 of `implement` was scored, not discarded.** The wave-3 replicate rule permits a
   discard only where the failure is demonstrably turn-cap exhaustion. This replicate ran one turn
   and produced text, so it is scored as a failed replicate and disclosed here rather than
   replaced.

## Fifteen red probes

Run against the current code in memory, touching no repo file. The scorer passes an exact set bare
or backticked and fails a dropped id, an extra id, a quoted set and a missing `FLOOR:` line; it
fails `brainstorm`'s set scored as `implement`, which is the cross-command keying the
parameterization had to get right. Dropping an id from a pre-registered set turns that command's
cross-check red, inventing one turns it red the other way, and removing a command from `EXPECTED`
is caught both by `converted-shape` and by the delivery case's own guard, which fails cleanly
rather than raising.

## Left undone

Nothing in the seat's scope. Evidence for all twenty cases is on disk under
`evals/.work/contract-*`, each with its stream, transcripts, argv, script, staged plugin and
`verdict.json`, and the delivery cases with `latency.json`.

One thing for the lead, outside my remit: the three tripped read-back criteria are a fact about
recall of a long id list, not about delivery, and the same run proves delivery on those three
commands. Whether that trips wave 5 or is ruled a metric property is the lead's call, and I have
not assumed either way.
