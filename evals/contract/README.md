# evals/contract/ — the plugin contract suite (maintainer-side, never shipped)

Provenance: `.mochiko/brainstorms/cli-schema-delivery/record.md` **D8 as amended**. This is the
layer the crate's own tests can never be: `cargo test` proves the store is sound, and proves
nothing at all about whether a real Claude Code session, loading a real plugin, actually receives
the rules before the model acts. That claim — "the plugin doesn't fail" — is what the user asked
for, and it lives here.

## What one case is

Most cases are one headless `claude -p` run inside the Docker AI sandbox `claude-mochiko`, loading
a plugin under `--plugin-dir` with `mochiko-cli` placed on the sandbox `PATH` the way a user would
install it (D4). Three cases need no session and no sandbox at all. The run is then
asserted against D8's deterministic set:

| assertion | what it catches |
|---|---|
| the `!` line executed | preprocessing did not run, or the grant was missing and the line was denied |
| the version-triple line present | the render never reached the model |
| the closing end line present | an oversized render truncated, keeping only its head line (wave-0 probe (e)) |
| **every `class: floor` rule delivered** | a floor rule the primitive declares never reached the model — criterion (1), added at wave 5 |
| no schema file Read | the model fell back to a file — the posture no-fallback exists to rule out |
| absence halts | a missing binary degraded silently instead of halting |
| skew halts | an out-of-range grammar was read best-effort instead of halting |

The fourth row is the wave-5 re-key and it deserves its own paragraph, because it replaced
something. Through wave 4 the floor question was asked of the *model* — a read-back line naming
the ids it had been given — and at wave 4 that bar tripped on three commands while every missed id
sat verbatim, with its `class: floor` line, in the transcript the model had read. The bar was
measuring recall of a long list and reporting it as delivery. `assert_floor_delivery` asks the
question that was meant: for each floor id, the transcript must carry a `### <id>` line whose next
attribute line contains `class: floor`. That pair is the render's own rule shape and nothing else
in a session produces it — a converted `.md` body carries no rule ids, and a model's read-back
names them comma-separated on one line. It is deterministic, it gates, and the read-back stays
beside it as a recorded measurement of the other thing.

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

The case list is **not written down**. `converted_commands()` and `converted_skills()` read the
plugin's own `.md` files and take every primitive carrying a `` !`mochiko-cli rules `` line — the
same test the dependency hook makes, and for the same reason: the primitive's file is the truth,
so the suite grows with the conversion waves and can never disagree with what ships. `--list`
prints the real set, which is how a partial wave shows exactly the families that have landed.

| case | sessions | what it holds the plugin to |
|---|---|---|
| `hook-input` | 0 | the two hook scripts, fed the committed stdin captures, on the host |
| `converted-shape` | 0 | each converted `.md`'s `!` lines against its own render, and every pre-registered floor set against the ids the binary renders |
| `render-ceiling` | 0 | every render of every converted primitive, against the inline ceiling |
| `absence` `[fixture]` | 1 | the binary is off `PATH`: the run halts and reads no schema file |
| `skew` `[fixture]` | 1 | the log declares `grammar: 99`: the D5 message, not a best-effort read |
| `<cmd>-delivery` ×6 | 3 each | the happy path — every block the command's render declares, every floor rule delivered, nothing Read, plus the read-back measurement and the delivered read cost |
| `<cmd>-absence` ×6 | 1 each | no binary, hooks on: the install line reaches the user |
| `brainstorm-skew` | 1 | the staged plugin's own log is out of range |
| `brainstorm-hooks-off` | 1 | no binary, hooks off: the harness path is the only guard left |
| `brainstorm-policy` | 1 | shell execution disabled by policy — recorded, never asserted (D8) |
| `<skill>-delivery` ×30 | 3 each | the same happy path, with the skill registered and the invocation path recorded |
| `<skill>-absence` ×30 | 1 each | no binary: the halt fires before any model turn and the install line still reaches the user |
| `preload` | 2 | a plugin agent's `skills:` frontmatter renders a converted skill at spawn — both binary states |

At the wave-5 landing that is **eighty-one cases and a hundred and fifty-one sessions**: two
fixture cases, six command delivery cases of three replicates, six single-session command absence
cases, three mechanism cases, thirty skill delivery cases of three replicates, thirty
single-session skill absence cases, and the two-session preload case. During the wave itself the
list is shorter, because it is discovered: after the review family lands it declares thirty-seven
cases, and it grows by sixteen or eighteen with each family.

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

## How a skill's delivery differs from a command's

**This section was written from a prediction, corrected by measurement, and then corrected again
when the plugin changed under it.** What follows is the measured state; the history is kept
because it is what the assertions are keyed to.

**A `/mochiko:<skill>` prompt line takes the prompt-expansion path, not the `Skill` tool.** The
seven blocks arrive in the expanded prompt exactly as a command's do — all seven head and end
lines, every floor rule present, nothing read as a fallback — and **no `Skill` tool call happens at
all**, in any of the thirty skills across a full run. A natural-language dispatch ("use the
`mochiko:<skill>` skill on …") did not produce one either; that session made three Bash calls and
delivered nothing. Wave-0 probe (c) *did* see the tool fire for a probe skill, so the routing is
prompt-shape dependent rather than fixed, which is why the suite records the path it observed per
run instead of assuming one.

**One primitive name therefore reaches the dependency hook down two different limbs**, and both
have to be right. `UserPromptExpansion` resolves the name against `commands/<bare>.md` and falls
back to `skills/<bare>/SKILL.md`; `PreToolUse`/`Skill` resolves the skill directly. The suite
asserts the confirmation's **noun** rather than its prefix, so a skill confirmed as a command
fails.

**That fallback did not always exist, and its absence is why these rows do.** For the length of
the wave-5 full run the hook resolved the expansion path against `commands/` only, so a converted
skill invoked as `/mochiko:<skill>` ran **ungated** — no limb fired at all. The suite measured it,
the gap was fixed in the hook, and `hook-input` now carries a row per converted skill down the
expansion limb in both binary states. Those are the rows that would have caught it.

**The absence halt moved when the fix landed, and the case is keyed to where it landed — only
there.** Before: the `!` line exited non-zero, the harness aborted the expansion and injected the
shell's stderr, the wave-1 shape. After: the hook blocks before expansion, so **no
`<local-command-stderr>` is injected** and the harness's own notice rides the result event, the
wave-3 command shape. An assertion pinned to the injected stderr went red the moment the hook was
fixed, which is the keying discipline working rather than failing.

`<skill>-absence` now asserts the hook block **and nothing else** (lead ruling, applying the
wave-3 audit's ruling on tolerant unions): the result event carries the hook's notice *and* no
`<local-command-stderr>` was injected. Both shapes halt safely, which is exactly why accepting
either would be wrong — a union that tolerates both stays green through the hook silently losing
its skill limb, which is the regression the fix exists to prevent. The pre-fix harness halt is the
record on disk, not a second accepted outcome. Beside it the case still asserts no model turn, the
install line in the session, the halt naming `/mochiko:<skill>`, no block delivered, no version
triple, and no schema read.

**The turn budget stays at 3 for skills** even though the expansion path answers in one turn. It
costs nothing and leaves room if the routing ever changes.

## The preload case

D3's skill form has two delivery channels and the `<skill>-delivery` cases exercise only one.
The other is the **preload**: a plugin agent whose `skills:` frontmatter names a skill gets that
skill rendered into it at spawn, `!` lines and all (wave-0 probe (d)), with no `Skill` tool call
and no `PreToolUse` limb in the way. If that channel silently stopped delivering, every other
assertion in this suite would still pass while a whole class of runs got no rules at all.

One case, two sessions, on `devils-advocate` preloading `review-specifications` — the subject is
that pairing because that skill converts in the first family, which makes the case runnable from
the wave's first landing. With the binary present the subagent must receive every block and every
floor rule. With it absent, nothing may be delivered and nothing read as a fallback.

The absent half was **measured before it was asserted**, and the distinction mattered: wave-0
measured a *permission-denied* `!` line failing the spawn outright, and a `command not found` line
is a different failure that could plausibly leave the spawn alive with an empty block. Measured at
wave 5: the spawn fails and the parent survives to explain it — two turns, no blocks in either
transcript, the parent reporting that the agent's preload shell command errored because
`mochiko-cli` is missing. So the assertions are the ones that hold under either outcome — nothing
delivered, nothing read, no version triple in the union of parent and sidechain transcripts — and
the spawn's own fate stays a recorded observation.

A subagent's turns are not in the parent's stream, and may not be in the parent's transcript file
either. Rather than guess, the case touches a marker file before each session and afterwards
copies out every transcript under `~/.claude/projects/` newer than it; the assertions read the
union and every fetched file lands in the evidence directory as `sidechain-<state>-N.jsonl`.

**`converted-shape` catches the one failure no session assertion can.** A `.md` that enumerates
six sections when the schema declares seven delivers six blocks, every one of them correctly
formed, and the only symptom is a rule the model was never given. The session assertions check
that what arrived is well-formed, not that everything was asked for, so the `.md` and the render
are compared directly — on the host, before a metered run is spent on it.

## Criterion (1): floor delivery, and the frozen sets it grades against

Each delivery case asserts that every `class: floor` rule the primitive declares arrived. The
keying is the render's own rule shape, read from the session transcript: a `### <id>` line whose
next attribute line — the one opening `[` — contains `class: floor`. It is a superset test, so an
extra pair cannot buy a false pass; a wrong primitive's blocks would still fail the head-line and
end-line assertions beside it. **It gates**, on every converted primitive and on the preload
case's binary-present session.

The id set is read **from the render at case time**, two ways. The preamble carries a `floors:`
index line listing every floor id in render order, which is the cheap read and the one the `.md`'s
own read-back sentence cites; walking the section renders is the expensive read and the one that
cannot be wrong by construction, because it is the rule bodies themselves. Both are computed and
their disagreement is a failing check, so the index line's correctness is tested rather than
trusted.

That case-time set is then cross-checked **in both directions** against a pre-registration, and
any difference fails the case. Commands are pre-registered in `EXPECTED` in `run.py`
(`brainstorm`'s seven in `wave3-reports/p3-suite-plan.md`, the other five in `wave4-plan.md` §4);
the thirty skills are in `expected-skills.json` beside this file, because thirty rows of floor ids
run to a few hundred entries. A bar read off the thing it grades is not a bar — the freeze cannot
be edited to match a render that changed, so a floor rule added or renamed later breaks a check
instead of quietly regrading.

| | commands | skills |
|---|---|---|
| pre-registration | `EXPECTED` in `run.py` | `expected-skills.json` |
| floor ids | 110 across six | 226 across thirty |
| baseline | `wc -c` of `<cmd>.yaml` + `common.yaml` | `wc -c` of `schema.yaml` + the family common |
| also frozen | — | the family, and the pre-conversion `SKILL.md` size |

`expected-skills.json` was written by **`freeze_expectations.py`**, in this directory, from the
render and the working tree **before any skill was converted and before the wave's first
session**, and is never edited afterwards. The pre-conversion body size is frozen because after
the conversion lands `HEAD` no longer carries it.

The script is kept in the tree so the file is reproducible rather than a hand-made artifact nobody
can re-derive, and it carries two guards that are not advisory: it refuses to write if **any**
`SKILL.md` already carries a `!` line, so a late freeze cannot record post-conversion floor sets or
body sizes, and it refuses to overwrite an existing freeze, so a second run cannot replace one a
wave has already been graded against. Both were exercised: run against the converted tree it names
the thirty converted skills and exits 1, and run against a pre-conversion root with the freeze in
place it exits 1 on the output file; neither wrote anything.

`--verify` checks the committed file instead of trusting it. It rebuilds every derived field from a
given plugin root, reuses only the metadata the original run recorded — the timestamp and the two
version strings, which are facts about that run rather than about the tree — and byte-compares.
Rebuilt against a plugin root whose `SKILL.md` files are restored to `7d098b9`, the committed
freeze is **byte-identical**.

Three things establish the ordering: those guards, the file's mtime against the run's own start
(reported by `converted-shape` on every run), and the commit adding it preceding every session in
git history.

## The read-back, recorded

Beside the gating assertion, each delivery case asks the model what it was given. This measures
something different — whether a model can enumerate a long list it holds — and it **gates
nothing**. The probe is the two-line form the wave-4 diagnostic pre-registered:

```
FLOOR-COUNT: <how many class: floor rules you were delivered, as a number>
FLOOR: <every class: floor rule id you were delivered, comma-separated>
```

Scored four ways per replicate: the count against the preamble's own `class: floor` pin (read from
the render, never `len()` of the expectation, so the two sides cannot agree by construction), the
ids exactly equal to the pre-registered set, the ids a superset of it, and the omitted ids named.
An id counts bare or wrapped in backticks; any other decoration is a miss.

The prompt puts a **gate-valid argument first** and the instruction after it. At wave 4 the
instruction was the whole argument, and a model following its primitive's own entry protocol
sometimes refused it as injected text — which is the behaviour the primitives are written to
produce. `$ARGUMENTS` takes everything after the name, so the instruction cannot be moved out of
the argument in a headless run; putting an acceptable token at the front is what can be done.
`PROBE_ARGUMENTS` carries one row per primitive with a sentence saying why its own entry accepts
it, written by reading that primitive's opening and first procedural step before the first
session. Two shapes appear: a free-text entry gets a one-word subject, and an artifact-taking
entry gets a path that does not exist, so its own missing-input or routing branch runs instead of
the argument becoming the thing under test. The six command rows take their arguments from
`diagnostic.py`'s table unchanged; `implement`'s justification is reworded, the other five are
verbatim. They are **duplicated rather than imported**, because that file imports this one and the
dependency may not run both ways, and because it is a frozen wave-4 artifact rather than a live
input.

**Why this is a measurement and not a bar**, on the evidence: at wave 4 the read-back was
criterion (1) and tripped on `implement` (1/3), `setup` (1/3) and `specify` (0/3). Every failing
replicate was a near miss — 33 of 34 ids, 15 of 16, 15 of 18 — and every unnamed id was verifiably
delivered, present with its `class: floor` line in the transcript the model read. The wave-4
diagnostic then re-ran all six under the two-line form with gate-valid arguments and got 16 of 18
replicates exact, `implement`'s thirty-four ids four times out of four; the two misses named zero
ids rather than a partial list, which is a refusal, not an attention ceiling. The bar was
measuring probe compliance and recall. Criterion (1) now asks about delivery, and this stays as
what it always was.

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

**From wave 5 the skill rows have real subjects.** They iterate the converted skills exactly as
the command rows iterate the converted commands, and for the same reason — the hook puts the
primitive's own name into the deny reason, so a per-skill row is what proves the user is told
which skill halted. Before that landing there was nothing converted to point at and the case
wrote a stub `SKILL.md` into its own staged copy; that path is kept for exactly as long as it is
needed and announces itself as a `row provenance` observation when it fires.

**One row still stages its own subject, and it touches nothing in `plugins/mochiko/`.** From
wave 4 every shipped command is converted, so the transition-clause row — an unconverted primitive
is never gated — has no command left to point at, and the case writes a stub
`contract-unconverted.md` into the staged copy. Its skill-side twin needs no stub even after
wave 5: the eight prose skills that carry no `schema.yaml` are never converted, so
`analysis-iterative` and its siblings remain real subjects for the limb. Both substitutions are
recorded as `row provenance` observations in the case's check list and in its `verdict.json`.

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

## Read cost, and the two units

Criterion (2) is the read cost, and at wave 5 it is read **per family** for the skills, as the
wave open ruled. Six quantities are recorded per skill: the delivered bytes and chars (the blocks
as they arrived, head line through end line, from the transcript of replicate 1, with the
three-replicate spread named on disagreement), the new body size (`wc -c` of the staged converted
`SKILL.md`), and the pre-conversion body size and raw baseline (both from the freeze).
Delivered-at-invoke is body plus render on each side — new is body-new plus delivered, old is
body-pre plus baseline — and the summary block prints the family sums.

**One trap, stated once.** The wave-5 plan's §0 table is in **bytes** and the record's F3 family
figures are in **chars**. Both units are carried per skill and per family, the byte comparison is
the criterion, and the two never share a column. A second, smaller gap is worth knowing: the
delivered figure sits seven bytes below the render total for a seven-block primitive, because
`delivered_blocks` captures head line through end line and each block's trailing newline falls
outside the capture.

## Measured figures

**Under binary `mochiko-cli 0.1.0 · grammar 1..1` with the wave-5 preamble** — the `floors:` index
line and the nine-line legend — against plugin 0.105.0 after migration `0002`. The crate version
does not move before the first publish, so a render-shape change like the `floors:` line is named
by the `plugin.json` bump rather than by a binary version (GI-012); every figure here is therefore
keyed to the plugin version plus the preamble shape, not to a crate release. Comparisons are
**bytes to bytes**; chars are reported beside them and are never the criterion.

### The six commands

The baseline is the pre-conversion read each `.md` obligated — `wc -c` of `<cmd>.yaml` plus
`common.yaml` — pre-registered before wave 4 ran. The rendered and delivered columns are wave 5's,
re-measured under the `floors:` line, which adds between 98 and about 1,000 bytes to a preamble
depending on how many floor ids the primitive carries.

| command | baseline | seven blocks rendered | largest render | delivered | vs baseline |
|---|---|---|---|---|---|
| `architecture` | 23,026 | 19,193 | `arch.sec.boundaries` 4,761 | 19,186 | −16.7 % |
| `brainstorm` | 12,819 | 11,209 | `preamble` 2,604 | 11,202 | −12.6 % |
| `feature` | 21,020 | 17,714 | `feat.sec.tools` 5,354 | 17,707 | −15.8 % |
| `implement` | 44,266 | 36,398 | `impl.sec.tools` 15,617 | 36,391 | −17.8 % |
| `setup` | 20,245 | 16,872 | `setup.sec.tools` 5,289 | 16,865 | −16.7 % |
| `specify` | 23,434 | 19,917 | `spec.sec.tools` 5,965 | 19,910 | −15.0 % |

`implement`'s whole render exceeds the ≈ 30,000-character inline ceiling, which is exactly why
delivery is chunked per section: every individual block is under it, the largest at 51.5 %. The
delivered figure sits seven bytes below the render total for every command because
`delivered_blocks` captures head line through end line and each block's trailing newline falls
outside the capture.

### The four skill families

Criterion (2) is read **per family** for the skills (wave-open ruling), at delivered-at-invoke —
body plus render on each side, which is the record's F3 measure. The pre-conversion side is the
body plus the schema-and-family-common baseline, both frozen in `expected-skills.json` before any
conversion landed.

| family | skills | rendered | converted B | pre-conversion B | Δ B | converted ch | pre-conversion ch | Δ ch |
|---|---|---|---|---|---|---|---|---|
| review | 8 | 92,595 | 123,570 | 125,499 | −1.5 % | 121,548 | 124,610 | −2.5 % |
| authoring | 8 | 110,385 | 154,143 | 156,070 | −1.2 % | 151,902 | 154,950 | −2.0 % |
| patterns | 9 | 82,335 | 120,144 | 102,016 | **+17.8 %** | 118,273 | 101,178 | **+16.9 %** |
| dense five | 5 | 56,973 | 92,531 | 85,143 | **+8.7 %** | 91,305 | 84,584 | **+7.9 %** |

**Against the record's F3 figures**, which are in chars and were estimates taken before the wave:
review ~119.9k, authoring ~150.6k, patterns ~95.9k, dense five ~81.8k. The pre-conversion column
above lands 2 to 6 % above each of them — 124.6k, 155.0k, 101.2k and 84.6k — so F3 slightly
understated the old cost, and the deltas measured here are correspondingly conservative.

Patterns and the dense five land **above** their baselines, which the wave open pre-stated: neither
family has a common schema file, so the render's fixed overhead has nothing to amortise against.
The two families that do have one come in below. The largest skill render is
`authoring-constitution.sec.artifact` at 9,938 bytes, 33 % of the ceiling; `impl.sec.tools` is
larger still at 15,617.

Store latency, timed inside the sandbox and load-dependent: per-section means of 26 to 86 ms across
the thirty-six primitives, a worst single run of 159 ms, and whole-fire figures of 183 to 531 ms for
all seven sections. The spread is wider than wave 4's six-command figures because thirty-six
primitives were timed under varying load. Every individual run is in each case's `latency.json`.

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
| `transcript.jsonl` | the session transcript copied out of the sandbox — where the delivered rules are (suffixed per replicate, and `-present` / `-absent` in the preload case) |
| `sidechain-<state>-N.jsonl` | preload only: every transcript the sandbox wrote during that half of the case, state-tagged `-present` / `-absent` so the two halves cannot overwrite each other; where a subagent's own turns land |
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

# the frozen skill expectations: written once, checked any time
python3 evals/contract/freeze_expectations.py --verify evals/contract/expected-skills.json \
    --plugin-root <a tree whose SKILL.md files predate the conversion>
```

The host cases run first even in a full run: they are free, they need nothing built, and a broken
hook script or a `.md` that asks for the wrong sections should be visible before twenty minutes of
sandbox build and a hundred and fifty-one sessions. During a conversion wave they are also the
per-family gate: after each family lands they validate that family's frozen floor sets, its `!`
line enumeration and its renders against the ceiling, at no session cost.

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

The run ends with a per-primitive block — read-back scores, floor rules delivered, delivered
bytes, baseline, delta — and, for the skills, a per-family block carrying delivered-at-invoke on
both sides. Criterion (1) is deliberately **not** in either: it became a gating assertion inside
each delivery case at wave 5, so a floor rule that failed to arrive fails the run rather than
printing a number someone has to notice. Both blocks are printed from the same `report()` figures
the verdicts carry and change no exit code.

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

A hundred and fifty-one sessions per full run at wave 5: two fixture cases, eighteen delivery
replicates across six commands, ninety across thirty skills, thirty-six single-session absence
cases, three mechanism cases, and the two-session preload case. Wave 4's figure was twenty-nine.

## Caveat carried on record

The kinako record marks sandbox subscription auth a `Contested` ruling sustained against adverse
Terms-of-Service evidence: automated headless use of a consumer subscription may sit outside what
it permits. The user adopted D8's amendment with that on record. It is repeated here because this
is the file a future maintainer reads before running the suite.

## What is not here

The suite never edits the plugin and never grades a primitive's content — it asserts delivery
mechanics and nothing else (GI-019). It dispatches no agent of its own; the one case that involves
a subagent, `preload`, has the headless session under test dispatch it, which is the behaviour
being measured. Nothing in this directory ships
with the plugin (GI-020).
