# evals/contract/ — the plugin contract suite (maintainer-side, never shipped)

Provenance: `.mochiko/brainstorms/cli-schema-delivery/record.md` **D8 as amended**. This is the
layer the crate's own tests can never be: `cargo test` proves the store is sound, and proves
nothing at all about whether a real Claude Code session, loading a real plugin, actually receives
the rules before the model acts. That claim — "the plugin doesn't fail" — is what the user asked
for, and it lives here.

## What one case is

Most cases are one headless `claude -p` run inside the Docker AI sandbox `claude-mochiko`, loading
a plugin under `--plugin-dir` with `mochiko-cli` placed on the sandbox `PATH` the way a user would
install it (D4). Three of the ten cases need no session and no sandbox at all. The run is then
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
  and its hooks included. Every wave-3 case stages this. A case that wants to perturb the log
  perturbs its own copy, never the repository.

## The ten cases

| case | sessions | what it holds the plugin to |
|---|---|---|
| `hook-input` | 0 | the two hook scripts, fed the committed stdin captures, on the host |
| `converted-shape` | 0 | a converted `.md`'s `!` lines against the section list its own render declares |
| `render-ceiling` | 0 | every render of every converted primitive, against the inline ceiling |
| `absence` `[fixture]` | 1 | the binary is off `PATH`: the run halts and reads no schema file |
| `skew` `[fixture]` | 1 | the log declares `grammar: 99`: the D5 message, not a best-effort read |
| `brainstorm-delivery` | 3 | the happy path — seven blocks delivered, nothing Read, plus the read-back metric |
| `brainstorm-absence` | 1 | no binary, hooks on: the install line reaches the user |
| `brainstorm-skew` | 1 | the staged plugin's own log is out of range |
| `brainstorm-hooks-off` | 1 | no binary, hooks off: the harness path is the only guard left |
| `brainstorm-policy` | 1 | shell execution disabled by policy — recorded, never asserted (D8) |

Two details in that table are easy to get wrong.

**The skew lever is not the same in both skew cases.** The fixture command passes no plugin root,
so `MOCHIKO_MIGRATIONS` reaches it. The converted command's `!` lines pass `--plugin-root
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

The delivery case asks, through its probe topic, for one line before anything else:

```
FLOOR: <every class: floor rule id you were delivered, comma-separated>
```

A replicate passes only if that line's token set is **exactly** the seven `class: floor` ids of
`brainstorm` — all present, nothing else, no partial credit. An id counts bare or wrapped in
backticks; any other decoration is a miss. A missing `FLOOR:` line is a failed replicate, not a
harness error.

**Bar: 3 of 3 replicates**, pre-registered in `wave3-reports/p3-suite-plan.md` before the first run
and unchanged after it. The metric is **reported and never gates** (D8): it lands in the case's
`verdict.json` under `read_back`, prints in the summary, and cannot set a non-zero exit code. The
lead reads the pilot's first abort criterion off it.

## The hook-input case

The cheapest gate in the suite, and one of the three that need neither sandbox nor session: each
committed capture under `fixture/hook-input/` is fed on stdin to the two hook scripts on the host,
with `CLAUDE_PLUGIN_ROOT` and `PATH` controlled per row. Thirteen assertions cover the three limbs
the hooks are allowed to have — leave an unconverted primitive alone, block on absence, block on an
out-of-range log — plus the `SessionStart` reporting branches.

The captures are the **shape** source, not the case matrix: each row substitutes only the one field
that names the primitive, so the field set stays whatever the platform actually sends. They are
real captures with the session ids, transcript path and working directory replaced by placeholders;
`fixture/hook-input/README.md` records what was substituted.

The converted `PreToolUse` rows need a converted skill, and none exists at wave 3, so the case
writes a stub `SKILL.md` **into its own staged copy**. Nothing in `plugins/mochiko/` is touched.

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
is keyed to exactly that. Both `brainstorm-absence` and `brainstorm-skew` halt this way; the
recorded `which limb halted first` says so in each `verdict.json`, and `channels` names where the
install line was carried. `brainstorm-hooks-off` is the case that still exercises the wave-1 shape,
which is the reason it exists.

## Where the delivered rules actually are

**The stream does not carry them.** `--output-format stream-json` emits no row containing the
expanded prompt, so the seven rendered blocks appear nowhere in `stream.jsonl`. They are in the
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

One more trap: the command's own halt clause quotes both the head-line shape and the phrases
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

Plugin 0.103.0, binary 0.1.0 · grammar 1..1. The delivered figure is read from the transcript of
`brainstorm-delivery`, and all three replicates agreed to the byte.

| figure | bytes | chars |
|---|---|---|
| the seven `brainstorm` blocks as delivered, head line through end line | 10,693 | 10,506 |
| the pre-conversion read the `.md` obligated (`brainstorm.yaml` + `common.yaml`) | 12,819 | 12,753 |
| the same, counting `command-labels.yaml` | 14,349 | — |
| largest single render (`preamble`, post-legend) | 2,102 | 2,055 |

The delivered cost is **16.6 % below** the pre-conversion baseline in bytes. The largest render
sits at 6.9 % of the ≈ 30,000-character inline ceiling. Comparisons are **bytes to bytes**; chars
are reported beside them and are never the criterion.

## The policy environment, recorded

`brainstorm-policy` asserts nothing (D8), and what it observed across two runs is why. Both runs
saw all seven `!` lines replaced by the placeholder, zero blocks delivered, the dependency hook's
presence line, one model turn, and **no schema file read as a fallback** — the posture held. What
differed is the model's own response: one run surfaced the not-delivered line and refused to
proceed, the other replied `FLOOR: none`, which invents nothing but does not halt in the way the
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
sandbox build and nine metered sessions.

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
the read-back metric, the delivered read cost, and everything `brainstorm-policy` observes. A case
whose checks are all `rec` has asserted nothing, and says so.

## Prerequisites, and why a skip is the honest answer

The host cases need only a runnable `mochiko-cli` — `target/release/mochiko-cli` or one on `PATH`,
verified by parsing its `--version` line rather than by `test -x`, which passes on a binary that
cannot run. In a full run the host binary is also compared against the sandbox build, so a stale
host binary is caught rather than silently measured.

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

Nine sessions per full run at wave 3: two fixture cases, three delivery replicates, and four
single-session brainstorm cases.

## Caveat carried on record

The kinako record marks sandbox subscription auth a `Contested` ruling sustained against adverse
Terms-of-Service evidence: automated headless use of a consumer subscription may sit outside what
it permits. The user adopted D8's amendment with that on record. It is repeated here because this
is the file a future maintainer reads before running the suite.

## What is not here

The suite never edits the plugin, never dispatches an agent, and never grades a primitive's
content — it asserts delivery mechanics and nothing else (GI-019). Nothing in this directory ships
with the plugin (GI-020).
