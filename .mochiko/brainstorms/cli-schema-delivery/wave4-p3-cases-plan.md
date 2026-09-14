# P3 — contract suite, per-command cases (wave 4)

**Host cross-check already run** (read-only, 2026-09-04, binary 0.1.0): plan §4's five floor-id sets
match the binary's render exactly (22 · 13 · 34 · 18 · 16 ids, every id identical), §0's render
figures reproduce to the byte, and the five raw baselines reproduce under `wc -c`. All six commands
declare six sections, so every delivery is seven blocks. Nothing was written.

## 1. The per-command family

`PILOT_COMMAND` and its brainstorm-shaped functions become one family parameterized by command name,
over a set discovered from the `.md` files through the existing `converted_primitives()` — the `!`
line, the truth source the hook and `converted-shape` already use. Command becomes an argument on
`brainstorm_expectations`, `assert_delivery`, `score_read_back`, `measure_latency` and `_aggregate`;
the four text scanners already take one. `brainstorm-delivery` and `brainstorm-absence` become the
brainstorm instances with assertions unchanged line for line, so the wave-3 keying survives intact;
`SANDBOX_CASES` is built at import, so `--list` prints the real set on any wave.

## 2. Pre-registered expectation constants

Plan §4's five sets go into the code verbatim as a frozen table beside `FLOOR_IDS`, which becomes
its `brainstorm` row — written down, never derived. `converted-shape` cross-checks each set against
`rendered_floor_ids()` for every command in the table and reports both directions of difference, so
a floor rule added or renamed turns that check red instead of quietly regrading. Each row also
carries that command's `baseline_bytes` from plan §0. The cross-check runs even for an unconverted
`.md`, because the render comes from the log — which is what let me validate all five sets today.

## 3. The cases

`<cmd>-delivery`, three replicates, the wave-3 shape exactly: `--max-turns 2`, prompt
`/mochiko:<cmd> <PROBE_TOPIC>` with `PROBE_TOPIC` unchanged (it instructs the single `FLOOR:` line
and is command-agnostic), the transcript fetched per replicate into the evidence directory, the
no-Read assertion scoped to that run's tool uses, and brainstorm's same nine per-replicate
assertions. Reported per case: read-back against the 3/3 bar, delivered bytes and chars from the
transcript against that command's baseline, and the latency band. `<cmd>-absence` is the
single-session halt with the binary off `PATH` and hooks in play, asserting what
`brainstorm-absence` asserts. `skew`, `hooks-off` and `policy` stay brainstorm-only: they exercise
the delivery mechanism — log resolution beating `MOCHIKO_MIGRATIONS`, the hook fail-open floor, the
policy placeholder — which does not vary by command (plan §7 assumption 2; fifteen sessions saved).
The fixture pair is unchanged.

## 4. `hook-input` rows

The absent and present rows iterate every converted command instead of `PILOT_COMMAND`: six
`UserPromptExpansion` absent rows asserting exit 2 with the install line and `/mochiko:<cmd>` on
stderr, six present rows asserting the presence line and nothing else. The skill-stub, skew,
foreign-namespace and `SessionStart` rows stay single, being hook limbs rather than commands. **One
row loses its subject:** the transition-clause check needs an unconverted command, and after P2
there is none, so `unconverted_primitive(plugin, "command")` returns `None` and the case fails on a
check about the hook rather than about wave 4. Fix, mirroring the converted-skill row — a stub
unconverted `.md` in the staged copy only, `plugins/mochiko/` untouched. The unconverted-skill row
needs no stub until wave 5.

## 5. Outputs, run order, and two flags

Each delivery case writes the wave-3 `verdict.json` shape with its own `expected_sections`,
`expected_counts`, `read_back`, `read_cost` (that command's `baseline_bytes`) and `latency`. The
summary gains a per-command block printing read-back and delivered-versus-baseline for all six, both
abort criteria in one place; every line is `report()` and the exit path is untouched. The README is
rewritten, not appended to: twenty cases and twenty-nine sessions in the table, a per-command
read-back bar against a named pre-registered set, and a figures row per command. Host cases first.
On "approved" I write the parametrization, the constant table, the `hook-input` rows, the stub and
the README, then run `--host-only`, which needs no sandbox and validates the five constant sets
before a metered session is spent. On the lead's "P1 and P2 closed" I re-run `--host-only` (P2's `!`
lines are what `converted-shape` grades; P1's migration changes the delivered `fail-conditions`
line) and then the full run. The report goes to `wave4-reports/p3-suite.md`: tally, abort criteria
per command, measurements, deviations. Nothing is committed. Two flags: the baselines are measured
**before** P1's migration, whose reworded intent line moves the raw baseline and the delivered
figure by a few bytes in the same direction, so the criterion is unaffected and the constants do not
move after the fact; and a full run is twenty-nine sessions, up from nine.
