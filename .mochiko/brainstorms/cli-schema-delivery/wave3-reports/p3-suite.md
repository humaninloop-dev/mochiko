# P3 — contract suite and measurements (wave 3)

**Tally: 10 of 10 cases passed, 10 ran, exit 0.** Fifteen further observations are recorded and
not asserted. Neither pilot abort criterion trips. Nothing was committed, and no file under
`plugins/mochiko/` was touched.

This is the post-rework state. V3's audit returned FAIL on one blocking item, and it was right:
the plan committed in writing that the tolerant channel union was interim, the probes measured the
shape, and the union shipped anyway. That is fixed under deviation 8 below, along with all five
advisories. Every fix was written test-first — the red probes are listed with each entry — and the
full suite was re-run afterwards, so every `verdict.json` on disk comes from the final code.

Files changed: `evals/contract/run.py`, `evals/contract/README.md`. The plan this was built to is
`wave3-reports/p3-suite-plan.md`, whose sections 8 and 9 carry the lead's rulings and the
deviations disclosed at build time.

## The abort criteria

| criterion | measured | trips? |
|---|---|---|
| (1) floor read-back below the pre-registered 3/3 bar | **3 of 3** replicates named all seven `class: floor` ids exactly, nothing else | no |
| (2) delivered read cost above the 12,819-byte baseline | **10,839 bytes** delivered — 15.4 % below it | no |

The bar was pre-registered in the plan before the first session and is unchanged. All three
replicates agreed to the byte, and no replicate needed the turn-cap exemption: every one produced
a `FLOOR:` line, so all three are scored and none was discarded.

## Measurements

**Delivered read cost**, read from the session transcript of `brainstorm-delivery`, replicate 1,
with the other two identical:

| component | bytes | chars |
|---|---|---|
| the seven rendered blocks, head line through end line | 10,693 | 10,506 |
| the `SessionStart` hook's line | 80 | 76 |
| the dependency hook's presence line | 66 | 65 |
| **total delivered** | **10,839** | **10,647** |
| pre-conversion baseline (`brainstorm.yaml` + `common.yaml`) | 12,819 | 12,753 |
| the same, counting `command-labels.yaml` | 14,349 | — |

That is **15.4 % below the baseline in bytes**, or 24.5 % below the three-file figure. Comparisons
are bytes to bytes throughout; chars sit beside them and are never the criterion.

**Store latency is load-dependent and should be read as a band, not a number.** The suite now
emits it: `measure_latency` times ten runs of each section inside the sandbox in a single shell, so
the figure is the binary and not the `sbx exec` transport, and writes every individual run to
`latency.json` in the delivery case's evidence directory. Four independent passes on the same
machine:

| pass | per-section mean | worst single run | whole fire |
|---|---|---|---|
| P3, first | 27–54 ms | 87 ms | 458 ms |
| V3, two passes | 42–77 ms | — | 648 ms |
| P3, post-rework (the emitted artifact) | 26–32 ms | 36 ms | 182 ms |

Same order of magnitude, spread by roughly a factor of three, and sandbox load is the obvious
explanation. **The conclusion is unchanged and does not depend on which pass you take:** a fire
costs somewhere between a fifth of a second and two thirds of a second of wall clock, once, before
the model reads anything. V3 could not reproduce the first band because nothing recorded it; that
is what the artifact now fixes.

**Log cost inside the plugin**, apparent size:

| figure | bytes |
|---|---|
| plugin before the move | 1,366,116 |
| the migration log and its README | 626,780 |
| plugin after the move | 1,992,896 |

The plugin grows 45.9 %, to just under 1.9 MiB. The genesis file itself is 618,122 bytes and is
byte-identical to the pre-move file.

**Largest render:** the preamble, at 2,102 bytes / 2,055 chars, which is 6.9 % of the roughly
30,000-character inline ceiling. Every converted render is far below it.

## What the suite now covers

Ten cases. Three need no sandbox and no session, and run first even in a full run, so a broken
hook or a mis-enumerated command is visible before any metered work.

- **`hook-input`** — thirteen assertions feeding P2's committed captures to both hook scripts on
  the host. It covers all three limbs the hooks may have: an unconverted primitive is left
  completely silent, absence blocks with the install line on the right channel per registration,
  and an out-of-range log blocks with the binary's own message. Both `SessionStart` branches and
  the unsupported-environment notice are covered too.
- **`converted-shape`** — the converted command's `!` lines against the section list its own
  preamble render declares, plus the Bash grant.
- **`render-ceiling`** — every render of every converted primitive against the inline ceiling.
- **`absence`** and **`skew`** — the wave-1 fixture cases, unchanged.
- **`brainstorm-delivery`** — three replicates: seven head lines, seven end lines, every end-line
  count matching the preamble's own section list, no literal `!` line, no Bash denial, no schema
  read, both hooks heard from, and all six commands registered.
- **`brainstorm-absence`**, **`brainstorm-skew`**, **`brainstorm-hooks-off`** — the three halt
  paths.
- **`brainstorm-policy`** — recorded, never asserted.

Every assertion was mutation-tested rather than merely observed passing. Stripping the converted
mark, breaking the plugin's log, lowering the ceiling, deleting one `!` line and removing the Bash
grant each turn their own check red, and the read-back scorer was unit-checked against seven
inputs including the backtick and quoted-decoration cases.

## Three measured facts that changed how assertions are written

**The event stream does not carry the delivered rules.** `--output-format stream-json` emits no row
containing the expanded prompt, so the seven blocks appear nowhere in `stream.jsonl`. They are in
the session transcript under the sandbox user's `~/.claude/projects/`, which each case now copies
out into its evidence directory. The head-line, end-line, hook-presence and read-cost assertions
all read that file; tool uses and the init event's command registry still come from the stream.
The transcript is found by globbing the session id rather than by rebuilding the project directory
name, which is a lossy munge of separators and dots.

**The wave-3 halt has a different shape from wave 1, and it is now measured.** The dependency hook
exits 2 before expansion, so no `<local-command-stderr>` block is injected at all and the `result`
field carries the harness's own notice: `UserPromptExpansion operation blocked by hook: [<script>]:
<message>`. Everything else matches wave 1. The assertion accepts a non-empty result only when it
is that notice, which still rejects any actual model output. Both halt cases fire this way, and
`brainstorm-hooks-off` is what still exercises the wave-1 harness shape, which is why it exists.

**The halt clause quotes the strings a naive assertion would search for.** The command body
contains `[shell command execution disabled by policy]` and `mochiko-cli rules not delivered`, and
the body travels into the transcript, so searching for either finds the instructions rather than
the outcome. The prose-halt check therefore reads only what the model itself wrote. The body does
not contain the install line, so the absence and skew assertions on that string are sound as
written.

## The policy environment, recorded

`brainstorm-policy` asserts nothing, per D8, and the observations are the reason that was the right
call. **Four are now on disk** — three of my runs and V3's independent re-run. Every one of them
saw all seven `!` lines replaced by the placeholder, zero blocks delivered, the dependency hook's
presence line, one model turn, and **no schema file read as a fallback**.

What varies is the model's own response, and it is a coin flip:

| observation | outcome |
|---|---|
| P3 run 1 | surfaced the not-delivered line |
| P3 run 2 | replied `FLOOR: none` |
| V3 re-run | replied `FLOOR: none` |
| P3 run 3, post-rework | surfaced the not-delivered line |

**The prose clause has held in two observations of four.** The lead's brief said one of three,
which was right as of the audit; my post-rework run is the fourth and it held. Correcting it
upward does not change the reading — a guard that fires half the time is not a guard.

The posture held in all four: nothing was fabricated and no fallback was taken. That is the part
that matters, and it is why this strengthens GI-020's unsupported declaration rather than
weakening it. It is worth the lead's attention because it is the one path where the prose clause is
load-bearing, and the evidence says it cannot be relied on there.

## Deviations from the plan, with reasons

1. **A tenth case, `converted-shape`.** A command that enumerates six sections when the schema
   declares seven delivers six well-formed blocks, and no session assertion in the plan calls that
   a failure — they grade what arrived, not what was asked for. Additive, free, and it moves a
   class of failure off the metered path.
2. **`hook-input` grew from eight rows to thirteen.** Added: the dependency hook against an
   out-of-range log, which is its only gate other than absence and needed no session; and
   `SessionStart` against a settings file setting `disableSkillShellExecution`, which is the
   unsupported-environment notice GI-020 obliges.
3. **The no-Read assertion was too narrow to survive staging.** It matched only
   `plugins/mochiko/schemas/` and a `schema.yaml` suffix, and the staged copy's own
   `schemas/brainstorm.yaml` matched neither — it would have passed a run that did exactly the
   thing no-fallback exists to rule out. It now matches any `.yaml` under a `schemas/` directory,
   and a Bash command naming such a path counts as the same failure.
4. **The head-line count dropped placeholder captures.** The halt clause quotes the head-line
   shape, and the first policy run reported `1 of 7` blocks delivered when the true figure was
   zero. It never affected a gating assertion, because the captured id is `<id>` and that is never
   an expected section, but a wrong count is worse than none. Both figures in this report are
   post-fix, and the full suite was re-run so every `verdict.json` on disk comes from the final
   code.
5. **A `report` status was added beside `ok`, `fail` and `pending`.** The policy case has no gating
   assertion, and a case whose check list is empty printed as a clean pass — a suite reporting
   success for having asserted nothing. Recorded measurements now print as `rec` and are counted
   separately in the summary.
6. **`--host-only` was added.** It narrows the declared case set rather than partially running the
   full one, so exit 0 stays honest. A failed host assertion now outranks a sandbox skip: the exit
   is 1, not 3.
7. **No synthesized `SessionStart` capture was needed.** P2 committed a real one, which is the
   path the lead preferred.

### Found by V3 and fixed in rework attempt 1

8. **The tolerant channel union shipped past the probes, and is now removed.** The plan said in
   writing that searching the union of transcript and process streams was interim and "a tolerant
   assertion is not the shipped form". The probes ran, the shape was measured, and the union
   survived in three gating positive assertions — the install line in both halt cases, `grammar 99`
   in the skew case, and the SessionStart line in delivery — with a docstring that had quietly
   promoted it to permanent. This was a real hole, not a cosmetic one: a regression in which a
   hook's message stopped reaching the session but still landed on the binary's stderr would have
   kept a check named "the install line reached the session" green. Positive assertions now read
   `asserted_output`, which is the session transcript plus the stream's own events and nothing
   else; the process streams stay in `channels_of` as evidence. Negative assertions keep the wider
   union, where breadth is strictness, and both docstrings now say which is which. Red-first
   probes: a fragment present only on `proc.stderr`, and only on `proc.stdout`, must fail the
   assertion while still appearing in the channel record. Both were red before the fix.
9. **V3-b — the two limbs of the no-Read check had drifted.** The `Read` limb caught
   `skills/<name>/schema.yaml` through `is_schema_path`; the `Bash` limb's separate regex required
   a `/schemas/` segment and returned clean for the same file. The Bash limb now routes through
   `is_schema_path`. No live risk at wave 3 with no skill converted; it would have been one at
   wave 4. Red-first probe: a shell `cat` of a skill schema.
10. **V3-a — the pre-registered floor set was never cross-checked.** `FLOOR_IDS` stays a written
    constant, because a bar derived from the thing it grades is not a bar, but `converted-shape`
    now compares it against the ids the binary actually renders and goes red on any difference. A
    floor rule added at wave 4 would otherwise have left the metric reporting a clean 3/3 while
    grading six of seven. Red-first probe: an invented eighth id turns the case red.
11. **V3-c — a vacuous assertion in `brainstorm-hooks-off`.** Its "no version triple was delivered"
    check read a text that excluded the session transcript, the one channel measured to carry
    delivered rules, so it could not fail. The case now fetches its transcript and reads the union.
    That also closes the evidence gap V3 named: every brainstorm case now leaves a
    `transcript.jsonl` on disk.
12. **V3-d — the latency figures had no artifact.** The suite now measures and emits them; see the
    band table above.
13. **V3-e — three places still described a nine-case suite.** The module docstring and two README
    sentences are corrected.

## Left undone

Nothing in the plan's scope. Two things are worth naming for whoever reads this next.

The corpus-wide render ceiling sweep stayed optional and was not run: only `brainstorm` is
converted, so the case measures what exists. When waves 4 and 5 convert more, the same case covers
them automatically, because it discovers converted primitives from the `!` line in each `.md` —
the same truth source the dependency hook uses, so the two can never disagree about what is
converted.

The suite's own evidence lives under `evals/.work/`, which is gitignored. The transcripts backing
every figure in this report are there, one directory per case, and they are readable without
re-running anything. Since the rework that includes `latency.json` and a transcript for every
brainstorm case, so nothing in this report rests on a number only I saw.

One piece of dead code survives deliberately: the `pending()` helper has no caller now that the
wave-1 assertion resolved. It stays because the status it creates is part of the summary contract —
a later wave that defers an assertion needs it to be impossible to mistake for a pass.
