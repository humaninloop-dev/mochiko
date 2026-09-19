---
report: disclosure
round: 1
seat: P3 (crate tests)
wave: wave 1 — the gate form
unit: crates/mochiko-cli/tests/{fidelity,validate,matrix_similar}.rs + the crate version bump
measured_against: working tree with 0008-gate-form.yaml and P4's grammar widening already present
---

## Plan — re-key the three frozen censuses to post-0008, and bump the crate to 0.2.0

**Every figure below is measured, not predicted.** 0008 is already in the log and P4's `src/`
change is in the tree, so the post-0008 state is readable now, from the source build (the installed
`0.1.0` binary predates P4's widening and rejects the log). At resume I re-run §7 and write what
that run prints, never the values below; a divergence halts rather than gets typed over. Measured:
`migrate status` → `sequences 1..8 · 74 documents · 1067 rules`; `migrate validate --report` →
`0 rejecting · 105 advisory`, similarity `scanned 1067 · scored 156764 · clusters 0 · suppressed
171`; views emitted to the scratchpad (never the repo) give skill floors 252, command floors 117.

### 1. Assertions whose value moves — 18

| file:line | assertion | old | new (measured) |
|---|---|---|---|
| fidelity.rs:170 | `replay.state.docs.len()` | 73 | 74 |
| fidelity.rs:172 | `replay.sequences()` | `vec![1..7]` | `vec![1..8]` |
| fidelity.rs:619 | `command_rules` | 327 | 329 |
| fidelity.rs:620 | `skill_rules` | 716 | 738 |
| fidelity.rs:621 | `command_rules + skill_rules` | 1043 | 1067 |
| fidelity.rs:622 | `skill_floors` | 240 | 252 |
| fidelity.rs:623 | `command_floors` | 116 | 117 |
| fidelity.rs:985 | the skill's floor pin | 7 | 8 |
| validate.rs:1099 | `state.docs.len()` | 73 | 74 |
| validate.rs:1112 | `command_rules` | 327 | 329 |
| validate.rs:1113 | `skill_rules` | 716 | 738 |
| validate.rs:1114 | total | 1043 | 1067 |
| validate.rs:1115 | `skill_floors` | 240 | 252 |
| validate.rs:1120 | `command_floors` | 116 | 117 |
| matrix_similar.rs:965 | command-family tuple | `(327, 12_421, 0, 60)` | `(329, 12_607, 0, 60)` |
| matrix_similar.rs:1016 | `report.scanned` | 1043 | 1067 |
| matrix_similar.rs:1017 | `report.scored` | 150_489 | 156_764 |
| matrix_similar.rs:1019 | `report.suppressed_hits` | 168 | 171 |

Arithmetic each figure must satisfy, so a mistyped number is caught: documents +1 (`common.gate-loop-bound`
is a block in an existing library and mints none) · command rules +2 · skill rules +22 (21 new, 1
on tiering) · command floors +1 · skill floors +12 · +24 total.

### 2. Pins I confirm and do not touch — 4 (no edit)

- **fidelity.rs:624, validate.rs:1130 — `fail_nodes` 36.** 0008 mints no `kind: fail` node; counted
  36 `*.fail.*` ids in the emitted command views. **validate.rs:2611 — `report.checked` 84
  pointers:** the new skill's view carries zero.
- **fidelity.rs:496/516 — sidecar census 597 and `RETIRED_SIDECAR_ANCHORS`.** 0008 carries no
  `supersede-rule` and no `tombstone-rule`, so no anchor retires; the array stays length 1.
- **matrix_similar.rs:1018 — `clusters.len()` 0.** Measured 0; §4 is why that is a confirmation.

### 3. Comments I rewrite beside the assertions

- **fidelity.rs:175** — append `0008's gate form` to the `sequences()` roll-call.
- **fidelity.rs:611-618, validate.rs:1106-1111** — one sentence: 0008 is pure mints, no retirement,
  two command rules on `setup` (one a floor) and twenty-two skill rules (one on tiering, twenty-one
  in the new document, eleven of them floors).
- **fidelity.rs:893-897** — the floor pin reads eight after 0008. **validate.rs:1099**'s message
  string carries `73`, so it moves with the value.
- **matrix_similar.rs:958-964** — the tail clause *"the log carries no op that adds a rule to a
  common library"* is **now false**: P4 widened `mint-rule` this wave, 0008 being the first use. I
  correct that clause rather than append to it, and record that the command-family scan moved +2
  from `setup` while suppressed held at 60 — all three new allowlist rows are skill-side.
- **matrix_similar.rs:1005-1015** — a 0008 paragraph saying why `scored` **rises** where 0007's
  fell: 0007 retired a rule out of the large `constraint` bucket; 0008 retires nothing and adds
  twenty-two skill rules across buckets, so every bucket it touches grows.

### 4. Confirming `clusters` is genuinely 0 before re-baselining the sweep

**0 is a suppressed zero, not a naive one** — P1's probe reported `clusters: 3` before the
allowlist rows existed. So the confirmation is not "the number came back 0" but four steps, any one
failing a hard stop to the lead under the near-dup-convergence doctrine:

1. `migrate validate --report` prints `none — no pair clears the threshold` **and**
   `suppressed: 171` — measured, both hold.
2. `git diff scripts/similar-rules-allowlist.yaml` shows **exactly three** added rows naming the
   three edges P1 recorded — measured: `review-sufficiency.evidence-floor`,
   `review-brainstorm.never-default-ready` and `review-code-minimalism.diff-and-report-both-read`
   against `validation-primitive-edit`'s `.evidence-floor`, `.default-fail` and `.from-file-floor`.
3. 168 + 3 = 171 exactly. More would mean a row suppressed an edge nobody adjudicated; fewer would
   mean a drafted row has no edge and should not have landed.
4. Each is the stub-versus-local shape the allowlist already suppresses eleven times — the new
   skill binds a `review-common` block, the counterparty keeps local text. I read all three
   `reason:` strings against that claim rather than accept each row's own wording. Only then do
   1067 / 156_764 / 171 go in, as a fixture refresh.


### 5. The version bump — and which `0.1.0` literals do **not** move

`crates/mochiko-cli/Cargo.toml:3` `"0.1.0"` → `"0.2.0"`, and `Cargo.lock` as `cargo build` rewrites
the package entry. **Nothing else.** `GRAMMAR_RANGE` stays `(1, 1)` (`src/migration.rs:16`).
Every `0.1.0` in the test tree is a **synthetic render-context fixture**, not a read of the crate
version, so none moves. `render.rs:280`, `home.rs:1041`, `home.rs:1353` and `cli.rs:1059` set
`binary: "0.1.0"` in a hand-built `render::Context`, each paired with a synthetic `plugin:
"0.103.0"` already stale against the live 0.111.0; `cli.rs:1052-1054` says so outright — the triple
is fixed "so the walk asserts the render's shape rather than the resolution's". `render.rs:318`,
`render.rs:339` and `cli.rs:1085` are the head lines those fixtures produce, so they move only if
the fixture moves. Tests that observe the real version read it through `env!("CARGO_PKG_VERSION")`
on both sides — `cli.rs:215`, `cli.rs:353`, `cli.rs:1160`, `fidelity.rs:399` — so they track the
bump with no edit, which is also the check that the bump is real.

### 6. Outside my ownership — two files fail `cargo test` and I have not touched them

The wave plan's P3 row names three test files. Two more carry post-0008 stale pins, and gate 5
(`cargo test -p mochiko-cli` green) cannot go green while they stand. Measured failures:
`views.rs:267`, `views.rs:338` and `views.rs:442`, all `73` → `74` (338 also needs P1's re-emitted
`skills/validation-primitive-edit.yaml` committed, which it is); and `render.rs:1167`, `checked`
36 → 37 with its message "six commands and thirty skills" moving to "thirty-one skills".

Four assertions, same drift class as my eighteen, mechanically identical evidence. Same shape as
the scope gap that fired mid-wave in `8968d43`, where P3 stopped rather than widen its own mandate;
I do the same and ask. `render.rs`'s widest-floor-index pin still passes.

### 7. Command sequence at resume
```
M="cargo run -q -p mochiko-cli -- --plugin-root plugins/mochiko"
$M migrate status ; $M migrate validate --report ; $M views emit --out "$SCRATCH"
git diff scripts/similar-rules-allowlist.yaml
# ... apply the edits ...
cargo build -p mochiko-cli                      # rewrites Cargo.lock for the 0.2.0 bump
cargo test -p mochiko-cli
MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
```

The first four re-derive every figure first-hand before a number is typed, `views emit` to the
scratchpad only. The full sweep needs its own invocation: the default run never exercises
`matrix_similar.rs:1016-1019`, so skipping it leaves three of my eighteen assertions unverified.

### 8. Risks
- **R1 — `views.rs` and `render.rs` are outside my row (§6).** Gate 5 stays red until someone takes
  those four assertions. Needs a lead ruling; I will not take them unasked.

- **R2 — `clusters: 0` is suppression-dependent (§4).** Three allowlist rows in another seat's file
  are the whole reason it is 0. Drop or reword one later and `matrix_similar.rs` fails reading as a
  detector regression; my comment at 1005-1015 names the rows so the next reader finds the cause.
- **R3 — the figures could move again before I execute.** Any re-emit or edit to 0008 moves every
  number in §1; §7 re-derives them all.
- **R4 — `evals/contract/expected-skills.json:4` freezes `"binary": "mochiko-cli 0.1.0 · grammar
  1..1"`.** Not my file, not a live gate (`run.py` reads only `families` and `skills` off that
  freeze), but stale at this bump, as is the same string at `evals/contract/README.md:499`.
- **R5 — the bump is a behaviour change no test can see.** An older binary rejects a section-less
  `mint-rule`, which is what makes 0.2.0 right rather than a patch; nothing asserts it, since the
  suite only runs the current build — the claim rests on P4's diff, not a green run.

## Execution

Complete. **22 assertions moved** — the planned 18, plus the 4 the lead's Risk-1 ruling widened in.
Every figure was re-derived first-hand before a number was typed (§7 steps 1-4) and reproduced the
plan's table exactly: `sequences 1..8 · 74 documents · 1067 rules`, `0 rejecting · 105 advisory`,
similarity `1067 / 156764 / 0 / 171`, and a scratchpad `views emit` giving skill floors 252,
command floors 117, 74 view files, 36 command fail ids.

### Assertions moved

| file:line | old | new |
|---|---|---|
| fidelity.rs:170 `docs.len()` | 73 | 74 |
| fidelity.rs:172 `sequences()` | `[1..7]` | `[1..8]` |
| fidelity.rs:624 `command_rules` | 327 | 329 |
| fidelity.rs:625 `skill_rules` | 716 | 738 |
| fidelity.rs:626 total | 1043 | 1067 |
| fidelity.rs:627 `skill_floors` | 240 | 252 |
| fidelity.rs:628 `command_floors` | 116 | 117 |
| fidelity.rs:991 floor pin | 7 | 8 |
| validate.rs:1101 `docs.len()` | 73 | 74 |
| validate.rs:1119 `command_rules` | 327 | 329 |
| validate.rs:1120 `skill_rules` | 716 | 738 |
| validate.rs:1121 total | 1043 | 1067 |
| validate.rs:1122 `skill_floors` | 240 | 252 |
| validate.rs:1127 `command_floors` | 116 | 117 |
| matrix_similar.rs:971 tuple | `(327, 12_421, 0, 60)` | `(329, 12_607, 0, 60)` |
| matrix_similar.rs:1041 `scanned` | 1043 | 1067 |
| matrix_similar.rs:1042 `scored` | 150_489 | 156_764 |
| matrix_similar.rs:1044 `suppressed_hits` | 168 | 171 |
| views.rs:269 `views.len()` | 73 | 74 |
| views.rs:342 `found.len()` | 73 | 74 |
| views.rs:447 `written.len()` | 73 | 74 |
| render.rs:1169 `checked` | 36 | 37 |

Four pins confirmed and deliberately not touched: `fail_nodes` 36 in both census tests, the
84-pointer pin at `validate.rs:2623`, the sidecar census 597 with `RETIRED_SIDECAR_ANCHORS` still
length 1 (0008 retires nothing), and `clusters.len()` 0. `render.rs`'s widest-floor-index pin
`("implement", 966, 1000)` also held — the new skill's eleven-floor index is not the widest.

Each moved assertion carries a comment beside it naming `0008-gate-form` and the 2026-09-19
`author-grader-consolidation` ruling. Two comment edits were corrections rather than additions:
`matrix_similar.rs:970`'s claim that *"the log carries no op that adds a rule to a common
library"* is now false and is struck, since P4 widened `mint-rule` this wave; and `render.rs`'s
"thirty skills" message became "thirty-one skills".

### The clusters confirmation — a suppressed zero

`clusters` is 0, and the record should say plainly that **this is a suppressed zero, not a naive
one**. All four steps ran and passed:

1. `migrate validate --report` printed `none — no pair clears the threshold` and
   `allowlist-suppressed edges: 171`.
2. `git diff scripts/similar-rules-allowlist.yaml` showed **exactly three** added rows, naming
   exactly the three edges P1 recorded.
3. 168 + 3 = 171 exactly — no row suppressed an unadjudicated edge, and no drafted row lacked one.
4. Reading the three `reason:` strings against the claim corrected one detail of the plan: only
   **two** are the stub-versus-local shape (`.evidence-floor` and `.default-fail` are `extends:`
   stubs binding a `review-common` block while the counterparty keeps local text). The third,
   against `review-code-minimalism.diff-and-report-both-read`, is a **keep-distinct** adjudication
   between two local texts with different read boundaries. The plan said all three were the same
   shape; that was wrong, and the comment now written into `matrix_similar.rs` records the split.

### Version bump

`Cargo.toml` 0.1.0 → 0.2.0 with a three-line comment giving the reason (a section-less `mint-rule`
is rejected by an older binary; the grammar version stays 1 because no binary carrying the narrower
rule was ever released). `cargo build` rewrote `Cargo.lock`, one line. **No test literal moved**,
exactly as planned: every `0.1.0` in the test tree is a synthetic `render::Context` fixture, and
the tests that observe the real version read `env!("CARGO_PKG_VERSION")` on both sides — they went
green against 0.2.0 without an edit, which is the check that the bump is real.

### Gates — all four green

| gate | result |
|---|---|
| `cargo test -p mochiko-cli` | 472 passed, 0 failed |
| `MOCHIKO_FULL_SIMILAR=1 … --test matrix_similar` | 48 passed, 0 failed (111s) |
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --all-targets -- -D warnings` | exit 0 |

The opt-in sweep was run separately and deliberately: the default suite never exercises the
four full-corpus assertions, so a green default run alone would have left them unverified.

Seven files changed, no production source among them: `crates/mochiko-cli/Cargo.toml`,
`Cargo.lock`, and `tests/{fidelity,validate,matrix_similar,views,render}.rs`. No git mutation run.
