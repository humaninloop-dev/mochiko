# P3 — read-back diagnostic (wave 4, plan §8)

**Bottom line: the wave-4 trip does not reproduce, and there is no enumeration-recall ceiling.
Sixteen of eighteen replicates named the count and the full id set exactly, `implement`'s
thirty-four included, 3/3. The two that did not were refusals, not partial recall — both had all
seven blocks delivered and every unnamed id present in the transcript. No replicate anywhere in
this run named a partial set.**

Recorded, never gating. Eighteen sessions plus two probes, sonnet, `--max-turns 2`, against the
landed wave-4 state. Runner: `evals/contract/diagnostic.py`, new; `run.py` and
`evals/contract/README.md` were not touched.

## The six rows

`count` = the model's `FLOOR-COUNT:` equals the preamble's own `class: floor` pin. `ids` = the
token set equals the pre-registered set exactly. `superset` = every expected id present, extras
tolerated.

| command | pin | count | ids | superset | omitted ids, per replicate |
|---|---|---|---|---|---|
| `architecture` | 22 | 2/3 | 2/3 | 2/3 | rep 1 none · **rep 2 all 22 (refusal)** · rep 3 none |
| `brainstorm` | 7 | 3/3 | 3/3 | 3/3 | none |
| `feature` | 13 | 2/3 | 2/3 | 2/3 | rep 1 none · rep 2 none · **rep 3 all 13 (no read-back line)** |
| `implement` | 34 | 3/3 | 3/3 | 3/3 | none |
| `setup` | 18 | 3/3 | 3/3 | 3/3 | none |
| `specify` | 16 | 3/3 | 3/3 | 3/3 | none |

Every id in an "all N" cell is present verbatim in that replicate's transcript: 22 of 22 for
`architecture`, 13 of 13 for `feature`. Nothing went unnamed because it was missing.

Three of the six commands are the ones that tripped abort criterion (1) at wave 4. All three come
back clean:

| command | wave 4 | diagnostic |
|---|---|---|
| `implement` | 1/3, one refusal, one 33-of-34 | 3/3 count, 3/3 ids |
| `setup` | 1/3, two partial sets | 3/3 count, 3/3 ids |
| `specify` | 0/3, the same id omitted three times | 3/3 count, 3/3 ids |
| `architecture` | 3/3 | 2/3, one refusal |
| `feature` | 3/3 | 2/3, one no-answer |

## The §8 reading, applied per command

Plan §8 pre-registered two branches. Applied literally:

- **`brainstorm`, `implement`, `setup`, `specify` — counts 3/3, ids 3/3.** Neither branch fires.
  Delivery is sound and enumeration is sound at 7, 34, 18 and 16 floor rules. Nothing here asks
  for a wave-5 design change.
- **`architecture` and `feature` — counts 2/3.** The literal reading is "an attention problem at
  that floor count; wave 5 needs a design change before opening."

**That second reading does not survive the evidence, and I am flagging it rather than applying it
mechanically.** Its antecedent is an attention problem at a floor count, and three things
contradict that mechanism. The failures sit at 22 and 13 ids while 34 ids succeeded three times
out of three, so they do not scale with count. Both failing replicates delivered all seven blocks
and held every id in the transcript. And neither produced a partial list, which is what an
attention or recall limit looks like; each produced no read-back line at all.

The mechanism is visible in what the two models wrote. `architecture` replicate 2 identified the
instruction embedded in its own command arguments and declined it, in its own words objecting to
being told to "short-circuit the desk protocol and just dump floor-rule counts/IDs before doing
anything else" as "not part of the legitimate command input". `feature` replicate 3 ran its Entry
step as written, reported map health and addressed the one-word ask, and never emitted the two
lines. Both are the probe colliding with the command, not the render failing to arrive.

**Recommended reading, for the lead's ruling:** the wave-4 trip was a probe artifact and there is
no delivery finding and no recall finding to remedy. If wave 5 keeps a read-back metric at all,
the thing to fix is the probe — an instruction that rides inside `$ARGUMENTS` is indistinguishable
from injected text, and some fraction of replicates will correctly refuse it at any floor count.
That is a bar-design question, not a render-design question. The ruling is the lead's; I have not
assumed it.

## The `implement` refusal, disposed

Plan §8: "`implement`'s wave-4 refusal is counted as a harness artifact only if the gate-valid
argument removes it 3/3." **It does: `implement` is 3/3 on both count and ids, with no refusal in
three replicates, and a fourth clean run in the pre-flight probe.** The refusal is therefore a
harness artifact by the pre-registered rule.

With one qualification the lead should carry: the refusal did not disappear, it moved. One
`architecture` replicate refused on the same grounds this run. So the artifact belongs to the
probe shape rather than to `implement`, and calling it "removed" would overstate what changed.

## The negative control

The `before` prompt shape puts the instruction ahead of the command, which keeps it out of
`$ARGUMENTS` but leaves a prompt that no longer starts with a slash. The command never expands and
nothing is delivered. The model answered `FLOOR-COUNT: 0` and `FLOOR: (none)`.

It invented nothing about a command it knows well. That is the control every reading of this
diagnostic rests on: the read-back lines track what was delivered in this session, not what the
model can reconstruct from training. The shape is kept in the runner, documented as a control.

## Deviations

1. **§8's change (1) is not implementable as written, and the runner amends it.** §8 asked for the
   instruction to move "out of `$ARGUMENTS` into the prompt after the command". `$ARGUMENTS` takes
   everything after the command name, newlines included: the `after` shape still substitutes the
   whole instruction into the command body, six or seven times per run depending on how many
   `$ARGUMENTS` sites that command's `.md` carries. The `before` shape keeps it out but stops the
   command expanding at all. There is no third shape in a headless run. What the runner does
   instead is put a **gate-valid token at the front of the argument** rather than making the
   argument nothing but instruction text. Both shapes were probed on `implement` before the
   eighteen sessions, and the finding was reported to the lead at that checkpoint.
2. **Two probe sessions were spent beyond §8's eighteen**, one per shape, for that finding.
3. **Entry gates were read before choosing every argument**, as §8 requires. Five commands take
   free text at Entry, so a one-word topic passes. `implement` gates on a capability entry — an
   accepted selection or a desk-confirmed delta card — so a delta-card path that does not exist
   makes Entry take its own routing branch instead of validating a feature id. Each rationale is
   recorded in the runner beside its argument.
4. **A `--shape` flag exists** that §8 did not specify, carrying the two shapes above. `after` is
   the default and the one all eighteen sessions used.

## What `diagnostic.py` touches

It is a separate runner and it gates nothing.

- **Reads, never writes:** `evals/contract/run.py`, imported through `importlib` for its helpers —
  staging, the sandbox preflight and build, `run_probe`, the transcript fetch, `score_read_back`,
  `EXPECTED`, the render helpers. No second copy of any of them.
- **Never touched:** `run.py` and `evals/contract/README.md`, both frozen for V3, whose modified
  times predate this file. Nothing under `plugins/mochiko/`. No git mutation, no commit.
- **Writes only:** its own evidence under `evals/.work/diagnostic-<command>-<id>/` — the staged
  plugin copy, argv, script, stream, stderr, transcripts, and a per-command `diagnostic.json` —
  plus the run summary at `evals/.work/diagnostic-summary-84657291/diagnostic.json`. The
  `diagnostic-` prefix keeps this evidence out of the `contract-*` namespace a suite run or its
  audit reads.
- **Cannot gate:** it has no assertion path and no non-zero exit on a score. It exits 3 only when
  the sandbox is unavailable, and 2 on an unknown command.

## Evidence

`evals/.work/diagnostic-summary-84657291/diagnostic.json` carries every replicate: the two lines
as written, the count against the pin, the id set, the omissions, whether each omitted id was in
the transcript, blocks delivered, turns, and the instruction-occurrence count that measured the
`$ARGUMENTS` substitution. The six per-command directories hold the transcripts behind it.
