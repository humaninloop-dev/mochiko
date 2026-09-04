# evals/contract/ — the plugin contract suite (maintainer-side, never shipped)

Provenance: `.mochiko/brainstorms/cli-schema-delivery/record.md` **D8 as amended**. This is the
layer the crate's own tests can never be: `cargo test` proves the store is sound, and proves
nothing at all about whether a real Claude Code session, loading a real plugin, actually receives
the rules before the model acts. That claim — "the plugin doesn't fail" — is what the user asked
for, and it lives here.

## What one case is

Most cases are one headless `claude -p` run inside the Docker AI sandbox `claude-mochiko`, loading
a plugin under `--plugin-dir` with `mochiko-cli` placed on the sandbox `PATH` the way a user would
install it (D4). Three of the twenty cases need no session and no sandbox at all. The run is then
asserted against D8's deterministic set:

| assertion | what it catches |
|---|---|
| the `!` line executed | preprocessing did not run, or the grant was missing and the line was denied |
| the version-triple line present | the render never reached the model |
| the closing end line present | an oversized render truncated, keeping only its head line (wave-0 probe (e)) |
| no schema file Read | the model fell back to a file — the posture no-fallback exists to rule out |
| absence halts | a missing binary degraded silently instead of halting |
| skew halts | an out-of-range grammar was read best-effort instead of halting |

## The plugin under test

Two different plugins are staged, and the difference matters when reading a result.

- **The fixture** at `fixture/probe-plugin/` is a one-command stand-in that carries the delivery
  shape in miniature. It has **no hooks**, which is exactly why the two wave-1 cases still run
  against it: they measure what the harness does with a failing `!` line, with nothing else in the
  way.
- **The real plugin**, `plugins/mochiko/`, copied whole into the case directory — its migration log
  and its hooks included. Every per-command case stages this. A case that wants to perturb the log
  or add a subject perturbs its own copy, never the repository.

## The cases

The case list is **not written down**. `converted_commands()` reads the plugin's own `.md` files
and takes every command carrying a `` !`mochiko-cli rules `` line — the same test the dependency
hook makes, and for the same reason: the primitive's file is the truth, so the suite grows with
the conversion waves and can never disagree with what ships. `--list` prints the real set.

| case | sessions | what it holds the plugin to |
|---|---|---|
| `hook-input` | 0 | the two hook scripts, fed the committed stdin captures, on the host |
| `converted-shape` | 0 | each converted `.md`'s `!` lines against its own render, and every pre-registered floor set against the ids the binary renders |
| `render-ceiling` | 0 | every render of every converted primitive, against the inline ceiling |
| `absence` `[fixture]` | 1 | the binary is off `PATH`: the run halts and reads no schema file |
| `skew` `[fixture]` | 1 | the log declares `grammar: 99`: the D5 message, not a best-effort read |
| `<cmd>-delivery` ×6 | 3 each | the happy path — every block the command's render declares, delivered, nothing Read, plus the read-back metric and the delivered read cost |
| `<cmd>-absence` ×6 | 1 each | no binary, hooks on: the install line reaches the user |
| `brainstorm-skew` | 1 | the staged plugin's own log is out of range |
| `brainstorm-hooks-off` | 1 | no binary, hooks off: the harness path is the only guard left |
| `brainstorm-policy` | 1 | shell execution disabled by policy — recorded, never asserted (D8) |

Twenty cases, twenty-nine sessions: two fixture cases, six delivery cases of three replicates,
six single-session absence cases, and the three mechanism cases.

**The last three run against the pilot only, and that is deliberate.** They exercise what happens
when the log, the hooks, or the shell is broken — log resolution beating `MOCHIKO_MIGRATIONS`, the
fail-open floor on a hook that cannot run, the placeholder a disabled shell leaves behind. None of
that varies with which command fired, so repeating them per command would buy fifteen metered
sessions and no new fact. The two cases that *are* per command are the two whose subject is the
primitive: what it delivers, and what it does when delivery is impossible.

Three more details are easy to get wrong.

**The skew lever is not the same in both skew cases.** The fixture command passes no plugin root,
so `MOCHIKO_MIGRATIONS` reaches it. A converted command's `!` lines pass `--plugin-root
"${CLAUDE_PLUGIN_ROOT}"`, and the resolution order is `--log-dir` › `--plugin-root
<root>/migrations` › `MOCHIKO_MIGRATIONS` › `./migrations`, so the environment variable loses to
the flag the command itself passes. `brainstorm-skew` therefore swaps the **staged plugin's own**
`migrations/`, and proves the swap took by running the binary directly against that root before
the session.

**`brainstorm-hooks-off` is not redundant with `brainstorm-absence`.** D7's floor is that a hook
which cannot run never blocks anything. That is the right default, and it means the hooks must not
be the only thing between a missing binary and a run that improvises. With every hook off, the
wave-1 harness shape has to come back.

**`converted-shape` catches the one failure no session assertion can.** A `.md` that enumerates
six sections when the schema declares seven delivers six blocks, every one of them correctly
formed, and the only symptom is a rule the model was never given. The session assertions check
that what arrived is well-formed, not that everything was asked for, so the `.md` and the render
are compared directly — on the host, before a metered run is spent on it.

## The read-back metric

Each delivery case asks, through its probe argument, for one line before anything else:

```
FLOOR: <every class: floor rule id you were delivered, comma-separated>
```

A replicate passes only if that line's token set is **exactly** that command's `class: floor` ids
— all present, nothing else, no partial credit. An id counts bare or wrapped in backticks; any
other decoration is a miss. A missing `FLOOR:` line is a failed replicate, not a harness error.

**Bar: 3 of 3 replicates, for every command**, `implement`'s thirty-four ids included. The sets
are pre-registered in `EXPECTED` before the wave's first session and unchanged after it —
`brainstorm`'s seven in `wave3-reports/p3-suite-plan.md`, the other five in `wave4-plan.md` §4:

| command | floor ids |
|---|---|
| `architecture` | 22 |
| `brainstorm` | 7 |
| `feature` | 13 |
| `implement` | 34 |
| `setup` | 18 |
| `specify` | 16 |

A written-down bar needs a cross-check, or a floor rule added at a later wave leaves it quietly
grading the wrong set while the metric keeps reporting a clean 3/3. `converted-shape` compares
every set against the ids the binary actually renders and reports both directions of difference.
It runs for each command in `EXPECTED` whether or not that command's `.md` is converted yet,
because the render comes from the migration log rather than from the `.md` — which is what lets a
wave validate its constants before spending a metered session on them.

The metric is **reported and never gates** (D8): it lands in the case's `verdict.json` under
`read_back`, prints in the per-command summary block, and cannot set a non-zero exit code. The
lead reads the first abort criterion off it, per command.

**What it measured at wave 4**, so a later reader knows what the metric does and does not say:
3/3 for `brainstorm`, `architecture` and `feature`; 1/3 for `implement` and `setup`; 0/3 for
`specify`. Every failing replicate was a near miss rather than an absence — 33 of 34 ids, 15 of
16, 15 of 18 — and in `specify`'s case the same single id was omitted all three times. The ids
that went unnamed were verifiably delivered: every block arrived, every end-line count matched its
preamble pin, and the missing ids are literally present in the transcripts the model read. The
metric grades recall of a long list, not delivery, and the two must not be read as one number.

## The hook-input case

The cheapest gate in the suite, and one of the three that need neither sandbox nor session: each
committed capture under `fixture/hook-input/` is fed on stdin to the two hook scripts on the host,
with `CLAUDE_PLUGIN_ROOT` and `PATH` controlled per row.

The rows cover the three limbs the hooks are allowed to have — leave an unconverted primitive
alone, block on absence, block on an out-of-range log — plus the `SessionStart` reporting
branches. The absence and presence rows **iterate every converted command**, because the hook
extracts the command name from its own stdin and puts it back in the message: a per-command row is
what proves the user is told which command halted rather than being handed a generic notice. The
skill rows, the skew row, the foreign-namespace row and the `SessionStart` rows stay single, being
hook limbs rather than commands.

The captures are the **shape** source, not the case matrix: each row substitutes only the one
field that names the primitive, so the field set stays whatever the platform actually sends. They
are real captures with the session ids, transcript path and working directory replaced by
placeholders; `fixture/hook-input/README.md` records what was substituted.

**Two rows stage their own subject, and neither touches `plugins/mochiko/`.** The converted
`PreToolUse` rows need a converted skill and none exists before wave 5, so the case writes a stub
`SKILL.md` into its own staged copy. From wave 4 the mirror problem appears on the other side:
every shipped command is converted, so the transition-clause row — an unconverted primitive is
never gated — has no subject left, and the case writes a stub `contract-unconverted.md` into the
staged copy instead. The clause is still live for skills until wave 6 and the limb is the same
one, so the row runs rather than being dropped. Both substitutions are recorded as `row
provenance` observations in the case's check list and in its `verdict.json`.

## What a halt looks like to the harness

**Measured, not assumed.** Both wave-1 cases were run against the authenticated `claude-mochiko`
sandbox, and both halt in the same place — earlier than the fixture's own halt clause:

1. The `!` line runs and exits non-zero.
2. Claude Code **aborts the prompt expansion** and injects the failing command's stderr as a user
   message, wrapped in `<local-command-stderr>`.
3. **No model turn happens.** The session ends with `num_turns: 0`, an empty `result`,
   `is_error: false`, `subtype: "success"`, and `claude` itself exits **0**.

The `.md` halt clause never executes, because the model never runs. That is a stronger guarantee
than the clause — nothing can be delivered and nothing can be improvised — but it means a suite
keyed to the clause's text would assert on something that is never written. The assertions are
keyed to the measured shape instead: zero turns, no assistant event, and the injected
`<local-command-stderr>` naming the right thing.

The injected block, verbatim from the skew run:

```
Shell command failed for pattern "!`mochiko-cli rules brainstorm --section preamble 2>&1`": [stderr]
0001-skew.yaml: the migration log is written in grammar 99, and this binary reads grammar 1..1. Update the binary: cargo install mochiko-cli
```

Two consequences worth carrying forward:

- **`claude` exit 0 proves nothing.** A suite reading only the exit code would call both halts
  clean runs. The result event has to be parsed.
- **The harness reports the failing command's output under `[stderr]` even though the fixture
  redirects with `2>&1`.** The message reaches the model either way, so the redirect is not what
  carries the halt.

**The second halt shape, measured at wave 3.** With the plugin's own hooks in play the
`UserPromptExpansion` dependency hook exits 2 *before* expansion, and the halt looks different:

- **no `<local-command-stderr>` block is injected at all** — the `!` line never runs;
- the `result` field carries the harness's own notice rather than being empty —
  `UserPromptExpansion operation blocked by hook: [<script path>]: <the script's stderr>`;
- everything else matches wave 1: `num_turns: 0`, no assistant event, `is_error: false`, exit 0.

So a non-empty `result` is acceptable only when it is that notice, and `assert_halt_before_model`
is keyed to exactly that. Every `<cmd>-absence` case halts this way; the recorded `which limb
halted first` says so in each `verdict.json`, and `channels` names where the install line was
carried. `brainstorm-hooks-off` is the case that still exercises the wave-1 shape, which is the
reason it exists.

## Where the delivered rules actually are

**The stream does not carry them.** `--output-format stream-json` emits no row containing the
expanded prompt, so the rendered blocks appear nowhere in `stream.jsonl`. They are in the
**session transcript** — the JSONL the hook stdin names as `transcript_path`, living under the
session user's `~/.claude/projects/`. Every assertion about what the model was *given* reads that
file, which each case copies out of the sandbox into its evidence directory as `transcript.jsonl`.

The split is worth stating plainly, because getting it backwards produces assertions that pass by
never looking at anything:

| read from the transcript | read from the stream |
|---|---|
| the seven head and end lines | tool uses, and so the no-Read assertion |
| the delivered read cost | the init event's `slash_commands` |
| the `UserPromptExpansion` hook's presence line — that hook emits **no stream row at all** | the `SessionStart` hook's output, which does appear as hook rows |

One more trap: a command's own halt clause quotes both the head-line shape and the phrases
`[shell command execution disabled by policy]` and `mochiko-cli rules not delivered`, and the
clause travels with the expanded prompt into the transcript. A search for those strings finds the
instructions, not the outcome. So placeholder-shaped section ids are dropped from the head-line
count, and the prose-halt check reads only what the model itself wrote.

### What a positive assertion may read

**A check that something arrived reads the session transcript and the stream's own events, and
nothing wider.** The process streams — `claude`'s stdout and stderr as the sandbox saw them — are
kept as evidence in each case's `channels` record, never as assertion subjects. The reason is a
false pass this suite exists to rule out: if a hook's message stopped reaching the session but
still landed on the binary's stderr, an assertion reading the wider union would keep a check named
"the install line reached the session" green while the user saw nothing.

Negative assertions invert it. "This string appears nowhere" is a stronger claim the more places
it looks, so those read the widest union available, transcript and process streams included.
`asserted_output` is the narrow one and `session_output_with` the wide one; the docstrings on both
say which is which.

## Measured figures

Plugin 0.105.0, binary 0.1.0 · grammar 1..1, after migration `0002`. Comparisons are **bytes to
bytes**; chars are reported beside them and are never the criterion.

The baseline is the pre-conversion read each `.md` obligated — `wc -c` of `<cmd>.yaml` plus
`common.yaml`. The **pre-registered** column is the figure fixed before the wave ran and is what
abort criterion (2) compares against; the **post-`0002`** column is the same measurement after the
migration reworded one intent line per command. Both sides moved by the same seven bytes, so the
criterion reads the same either way — the pair is shown so it can be checked rather than trusted.

| command | baseline, pre-registered | baseline, post-`0002` | seven blocks rendered | largest render |
|---|---|---|---|---|
| `architecture` | 23,026 | 23,033 | 18,576 | `arch.sec.boundaries` 4,761 |
| `brainstorm` | 12,819 | 12,826 | 10,940 | `preamble` 2,335 |
| `feature` | 21,020 | 21,027 | 17,353 | `feat.sec.tools` 5,354 |
| `implement` | 44,266 | 44,273 | 35,418 | `impl.sec.tools` 15,617 |
| `setup` | 20,245 | 20,252 | 16,290 | `setup.sec.tools` 5,289 |
| `specify` | 23,434 | 23,441 | 19,463 | `spec.sec.tools` 5,965 |

The render column carries the wave-4 preamble legend at nine lines, which is why every command
sits 233 bytes above the same measurement taken against the wave-3 six-line legend.

`implement`'s whole render exceeds the ≈ 30,000-character inline ceiling, which is exactly why
delivery is chunked per section: every individual block is under it, the largest at 51.5 %.

The **delivered** figure is the blocks as they actually arrive in the session, head line through
end line, read from the transcript rather than from a fresh render — what a later render would
print is not evidence of what the session was given. All three replicates agreed to the byte for
every command.

| command | delivered | against the pre-registered baseline |
|---|---|---|
| `architecture` | 18,569 | −19.4 % |
| `brainstorm` | 10,933 | −14.7 % |
| `feature` | 17,346 | −17.5 % |
| `implement` | 35,411 | −20.0 % |
| `setup` | 16,283 | −19.6 % |
| `specify` | 19,456 | −17.0 % |

Each figure lands in its delivery case's `verdict.json` under `read_cost`, beside the two
`brainstorm`-only figures the wave-3 README quoted (12,753 chars, and 14,349 bytes counting
`command-labels.yaml`). The per-command summary block prints it against the baseline and names
either abort criterion when one trips.

Store latency, timed inside the sandbox and load-dependent: per-section means of 26 to 33 ms
across the six commands, a worst single run of 67 ms, and whole-fire figures of 181 to 220 ms for
all seven sections. Every individual run is in each case's `latency.json`.

## The policy environment, recorded

`brainstorm-policy` asserts nothing (D8), and what it observed across five runs is why. Every run
saw the `!` lines replaced by the placeholder, no blocks delivered, the dependency hook's presence
line, one model turn, and **no schema file read as a fallback** — the posture held every time.
What differed is the model's own response: three runs surfaced the not-delivered line and refused
to proceed, two replied `FLOOR: none`, which invents nothing but does not halt in the way the
clause asks. The prose clause is the only guard in that environment, and it is not reliable there.
GI-020 already declares the environment unsupported; this is the evidence for that, not against
it.

## Evidence

Every case writes its evidence to `evals/.work/contract-<case>-<id>/` (gitignored):

| file | what it holds |
|---|---|
| `stream.jsonl` | the full stream-json event stream (suffixed `-1`, `-2`, `-3` for the replicates) |
| `transcript.jsonl` | the session transcript copied out of the sandbox — where the delivered rules are |
| `stderr.txt` | the sandbox process's stderr |
| `argv.txt` | the exact `claude` argv |
| `script.sh` | the `sh -c` script the sandbox ran |
| `verdict.json` | per-check status, the measured halt shape, the read-back scores, the read-cost figures |
| `latency.json` | delivery only: every individual timed run, per section |
| `direct-binary.txt` | skew only: the binary's own exit code, stdout and stderr |
| the staged plugin | the subject, exactly as the run saw it |

A pass/fail line is not evidence. A failing case has to be readable afterwards without re-running
it, and a passing one has to be auditable by someone who was not there.

## Running it

```
python3 evals/contract/run.py              # every case
python3 evals/contract/run.py --host-only  # only the three that need no sandbox
python3 evals/contract/run.py --list       # print the case list and exit
```

The host cases run first even in a full run: they are free, they need nothing built, and a broken
hook script or a `.md` that asks for the wrong sections should be visible before twenty minutes of
sandbox build and twenty-nine metered sessions.

**The host binary must match the source tree.** The host cases render through
`target/release/mochiko-cli`, so a binary built before the current migration log fails every
render with the log's own error — `op-unknown`, when the log uses a change op that binary does not
carry. Rebuild with `cargo build --release -p mochiko-cli`. In a full run the host binary is also
compared against the sandbox build, which catches the same staleness from the other side.

Exit codes are deliberately three-valued, because a suite that silently does nothing is worse than
one that fails:

| exit | meaning |
|---|---|
| 0 | every declared case ran and passed |
| 1 | a case ran and an assertion failed |
| 3 | **SKIPPED** — the suite could not run, with the reason printed |

A failed assertion outranks a skip: if a host case fails and the sandbox is then unreachable, the
exit is 1, not 3. The case list prints on every path, so "0 cases ran" is visible rather than
inferred, and `--host-only` narrows the declared set rather than partially running the full one.

A check has four statuses, and two of them are deliberately not passes. `pend` marks an assertion
whose subject arrives in a later wave. `rec` marks a measurement D8 records rather than asserts —
the read-back metric, the delivered read cost, the latency band, the staged-subject provenance
notes, and everything `brainstorm-policy` observes. A case whose checks are all `rec` has asserted
nothing, and says so.

The run ends with a per-command block: read-back score, delivered bytes, baseline, delta, and a
named abort criterion when one trips. It is printed from the same `report()` figures the verdicts
carry and changes no exit code; it exists so both criteria can be read off one place per command.

## Prerequisites, and why a skip is the honest answer

The host cases need only a runnable `mochiko-cli` — `target/release/mochiko-cli` or one on `PATH`,
verified by parsing its `--version` line rather than by `test -x`, which passes on a binary that
cannot run.

The sandbox cases check, in order: `sbx` on `PATH` · the sandbox reachable · `claude` runnable
inside it · **the sandbox authenticated for headless runs** · the fixture present · `cargo`
available in the sandbox to build the binary there. Any failing rung skips the sandbox cases with
its reason.

The authentication rung matters most. An unauthenticated sandbox still starts a session and then
returns an error result, which a careless suite would read as a failed assertion — a red result
that says nothing about the plugin. **`sbx login` is the user's own action**; the suite never
attempts it.

The binary is built **inside** the sandbox rather than copied in: the sandbox is Linux and the
maintainer's host is macOS, so a host build is the wrong architecture. Before the crates.io publish
that is the only way to get a binary there; after it, `cargo install mochiko-cli` inside the
sandbox is the D4 shape and this step is replaced by it.

## Gate split, and the cost

Per D8 as amended: the **full contract suite is a maintainer-side gate at `plugin.json` bumps**,
run in the sandbox at no metered cost, and a SKIPPED suite is not green (GI-012 gate 6). **GitHub
CI keeps the four crate layers** (`cargo test` · `fmt` · `clippy` · `audit`) and runs no headless
sessions — the original D8's "API key in CI secrets" clause is withdrawn. The sandbox is Linux and
the host is macOS; together they are the two OS rows, and there is no CI matrix.

Twenty-nine sessions per full run at wave 4: two fixture cases, eighteen delivery replicates
across six commands, six single-session absence cases, and three mechanism cases.

## Caveat carried on record

The kinako record marks sandbox subscription auth a `Contested` ruling sustained against adverse
Terms-of-Service evidence: automated headless use of a consumer subscription may sit outside what
it permits. The user adopted D8's amendment with that on record. It is repeated here because this
is the file a future maintainer reads before running the suite.

## What is not here

The suite never edits the plugin, never dispatches an agent, and never grades a primitive's
content — it asserts delivery mechanics and nothing else (GI-019). Nothing in this directory ships
with the plugin (GI-020).
